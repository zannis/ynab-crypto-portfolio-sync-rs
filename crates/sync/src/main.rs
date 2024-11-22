mod binance;
mod bitcoin;
mod evm;
mod exchange;

use crate::binance::get_binance_wallet_value;
use crate::bitcoin::get_total_from_coinlore;
use crate::evm::get_total_from_debank;
use crate::exchange::get_exchange_rate;
use alloy_primitives::Address;
use chrono::Utc;
use dotenv::dotenv;
use std::collections::HashMap;
use std::env;
use std::str::FromStr;
use thiserror::Error;
use tokio::join;
use tracing::{error, info, warn};
use uuid::Uuid;
use ynab_api::apis::accounts_api::{create_account, get_accounts};
use ynab_api::apis::budgets_api::get_budgets;
use ynab_api::apis::configuration::Configuration;
use ynab_api::apis::transactions_api::{
    create_transaction, get_transactions_by_account, update_transaction,
};
use ynab_api::models::{
    AccountType, ExistingTransaction, NewTransaction, PostAccountWrapper, PostTransactionsWrapper,
    PutTransactionWrapper, SaveAccount, TransactionClearedStatus,
};

#[derive(Debug, Error)]
pub enum SyncError {
    #[error("Environment variable error: {0}")]
    EnvVar(#[from] env::VarError),
    #[error("YNAB API error: {0}")]
    YnabApi(String),
    #[error("Exchange rate error: {0}")]
    ExchangeRate(String),
    #[error("Wallet value error: {0}")]
    WalletValue(String),
    #[error("Balance sync error: {0}")]
    BalanceSync(String),
}

#[tokio::main]
async fn main() -> Result<(), SyncError> {
    let docker_enabled = cfg!(feature = "docker");
    let headless_enabled = cfg!(feature = "headless");

    if docker_enabled && headless_enabled {
        error!("Features 'docker' and 'headless' cannot be enabled at the same time. Exiting...");
        std::process::exit(1);
    }

    dotenv().ok();
    setup_tracing();

    let ynab_key = env::var("YNAB_KEY").unwrap_or_else(|_| {
        error!("YNAB API key is missing. Exiting...");
        std::process::exit(1)
    });

    info!("Starting crypto portfolio sync...");

    info!("Getting YNAB account...");

    let ynab_account_name = env::var("YNAB_ACCOUNT_NAME").unwrap_or_else(|_e| {
        info!("No YNAB account name set, using the default `Crypto` one.");
        "Crypto".to_string()
    });

    let config = ynab_config(&ynab_key);

    let budget = get_budget(&config).await?;
    let currency = budget
        .currency_format
        .unwrap_or_default()
        .unwrap_or_default()
        .iso_code;

    info!("Getting accounts for budget {}...", budget.id);

    let account =
        get_or_create_account(&config, &budget.id.to_string(), &ynab_account_name).await?;

    let txns = get_transactions_by_account(
        &config,
        &budget.id.to_string(),
        &account.id.to_string(),
        None,
        None,
        None,
    )
    .await
    .map_err(|e| SyncError::YnabApi(e.to_string()))?;

    info!("Getting exchange rate...");

    let rate = get_exchange_rate(&currency, "USD")
        .await
        .map_err(|e| SyncError::ExchangeRate(e.to_string()))?;

    let values = get_wallet_balances().await?;

    for (wallet, total) in values {
        update_wallet_transaction(
            &config,
            &budget.id.to_string(),
            &account.id,
            &wallet,
            total,
            rate,
            &txns,
        )
        .await?;
    }

    info!("Sync completed successfully!");
    Ok(())
}

async fn get_budget(
    config: &Configuration,
) -> Result<Box<ynab_api::models::BudgetSummary>, SyncError> {
    let budgets = get_budgets(config, Some(true))
        .await
        .map_err(|e| SyncError::YnabApi(e.to_string()))?;

    info!(
        "Got {} budgets. Using the default or first one...",
        budgets.data.budgets.len()
    );

    Ok(budgets
        .data
        .default_budget
        .clone()
        .unwrap_or_else(|| Box::new(budgets.data.budgets.first().unwrap().clone())))
}

async fn get_or_create_account(
    config: &Configuration,
    budget_id: &str,
    account_name: &str,
) -> Result<Box<ynab_api::models::Account>, SyncError> {
    let accounts = get_accounts(config, budget_id, None)
        .await
        .map_err(|e| SyncError::YnabApi(e.to_string()))?;

    info!("Got {} accounts", accounts.data.accounts.len());

    if let Some(account) = accounts
        .data
        .accounts
        .into_iter()
        .find(|a| a.name.eq(account_name))
    {
        Ok(Box::new(account))
    } else {
        info!(
            "No YNAB account called `{}` found. Creating it now...",
            account_name
        );

        let response = create_account(
            config,
            budget_id,
            PostAccountWrapper {
                account: Box::new(SaveAccount {
                    name: account_name.to_string(),
                    balance: 0,
                    r#type: AccountType::OtherAsset,
                }),
            },
        )
        .await
        .map_err(|e| SyncError::YnabApi(e.to_string()))?;

        Ok(response.data.account)
    }
}

async fn get_wallet_balances() -> Result<HashMap<String, f64>, SyncError> {
    let evm_wallets = env::var("EVM_WALLETS")
        .unwrap_or_default()
        .split(',')
        .filter(|w| Address::from_str(w).is_ok())
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    let bitcoin_wallets = env::var("BTC_WALLETS")
        .unwrap_or_default()
        .split(',')
        .filter(|w| ::bitcoin::Address::from_str(w).is_ok())
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    info!("Getting wallet balances...");

    let evm_values = async {
        let mut values = HashMap::new();
        for wallet in &evm_wallets {
            match get_total_from_debank(wallet).await {
                Ok(Some(total)) => {
                    values.insert(wallet.clone(), total);
                }
                Ok(None) => {
                    warn!("No value returned for EVM wallet: {}", wallet);
                }
                Err(e) => {
                    error!("Error getting EVM wallet value for {}: {}", wallet, e);
                }
            }
        }
        values
    };

    let bitcoin_values = async {
        let mut values = HashMap::new();
        for wallet in &bitcoin_wallets {
            match get_total_from_coinlore(wallet).await {
                Ok(Some(total)) => {
                    values.insert(wallet.clone(), total);
                }
                Ok(None) => {
                    warn!("No value returned for Bitcoin wallet: {}", wallet);
                }
                Err(e) => {
                    error!("Error getting Bitcoin wallet value for {}: {}", wallet, e);
                }
            }
        }
        values
    };

    let (evm_results, bitcoin_results) = join!(evm_values, bitcoin_values);

    let mut values: HashMap<String, f64> = evm_results.into_iter().chain(bitcoin_results).collect();

    if env::var("BINANCE_API_KEY").is_ok() {
        info!("Getting binance wallet value...");
        match get_binance_wallet_value().await {
            Ok(value) => {
                values.insert("Binance".to_string(), value);
            }
            Err(e) => {
                error!("Error getting Binance wallet value: {}", e);
            }
        }
    }

    Ok(values)
}

async fn update_wallet_transaction(
    config: &Configuration,
    budget_id: &str,
    account_id: &Uuid,
    wallet: &str,
    total: f64,
    rate: f64,
    txns: &ynab_api::models::TransactionsResponse,
) -> Result<(), SyncError> {
    // calculate the new total in milliunits
    let new_total = (total / rate * 1000.0).ceil() as i64;

    let today = Utc::now().date_naive();

    let today_str = today.format("%Y-%m-%d").to_string();

    let last_txn = txns
        .data
        .transactions
        .iter()
        .filter(|t| t.payee_name == Some(Some(wallet.to_string())) && t.date.ne(&today_str))
        .last();

    let todays_txn = txns
        .data
        .transactions
        .iter()
        .filter(|t| t.payee_name == Some(Some(wallet.to_string())) && t.date.eq(&today_str))
        .last();

    let last_total = last_txn.map(|t| t.amount).unwrap_or(0);

    if let Some(todays_txn) = todays_txn {
        info!("Balance for {wallet} exists for today. Updating amount...");

        let put_txn: PutTransactionWrapper = PutTransactionWrapper {
            transaction: Box::new(ExistingTransaction {
                amount: Some(new_total - last_total),
                cleared: Some(TransactionClearedStatus::Cleared),
                ..Default::default()
            }),
        };

        update_transaction(config, budget_id, &todays_txn.id.to_string(), put_txn)
            .await
            .map_err(|e| SyncError::YnabApi(e.to_string()))?;
    } else {
        create_transaction(
            config,
            budget_id,
            PostTransactionsWrapper {
                transaction: Some(Box::new(NewTransaction {
                    account_id: Some(account_id.clone()),
                    date: Some(today_str),
                    amount: Some(new_total - last_total),
                    payee_name: Some(Some(wallet.to_string())),
                    cleared: Some(TransactionClearedStatus::Cleared),
                    ..Default::default()
                })),
                ..Default::default()
            },
        )
        .await
        .map_err(|e| SyncError::YnabApi(e.to_string()))?;
    }

    Ok(())
}

fn ynab_config(bearer_access_token: &str) -> Configuration {
    let mut config = Configuration::new();
    config.bearer_access_token = Some(bearer_access_token.to_owned());
    config
}

fn get_env_var<T: From<String>>(key: &str) -> Result<T, env::VarError> {
    env::var(key).map(Into::into)
}

fn setup_tracing() {
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
