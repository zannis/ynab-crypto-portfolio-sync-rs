mod binance;
mod bitcoin;
mod evm;
mod exchange;

use crate::binance::get_binance_wallet_value;
use crate::bitcoin::get_total_from_coinlore;
use crate::evm::get_total_from_debank;
use crate::exchange::get_exchange_rate;
use chrono::Utc;
use dotenv::dotenv;
use std::collections::HashMap;
use std::env;
use thiserror::Error;
use tokio::join;
use tracing::{error, info, warn};
use uuid::Uuid;
// Added error
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

// Add custom error type for better error handling
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
}

#[tokio::main]
async fn main() -> Result<(), SyncError> {
    dotenv().ok();
    setup_tracing();

    info!("Starting crypto portfolio sync...");

    let ynab_key = get_env_var::<String>("YNAB_KEY")?;
    let ynab_account_name =
        get_env_var_or_default::<String>("YNAB_ACCOUNT_NAME", "Crypto".to_string());

    info!("Getting YNAB account...");

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

    let values = get_wallet_values().await?;

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

async fn get_wallet_values() -> Result<HashMap<String, f64>, SyncError> {
    let evm_wallets = get_env_var::<String>("EVM_WALLETS")?
        .split(',')
        .map(ToString::to_string)
        .collect::<Vec<String>>();

    let bitcoin_wallets = get_env_var::<String>("BTC_WALLETS")?
        .split(',')
        .map(ToString::to_string)
        .collect::<Vec<String>>();

    info!("Getting wallet values...");

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
    info!("Looking for {} txn...", wallet);

    let txn = if let Some(txn) = txns
        .data
        .transactions
        .iter()
        .find(|t| t.payee_name == Some(Some(wallet.to_string())))
    {
        info!("Found txn: {:?}", txn.payee_name);
        Box::new(txn.clone())
    } else {
        info!("No txn found for {}. Creating a new one...", wallet);

        let response = create_transaction(
            config,
            budget_id,
            PostTransactionsWrapper {
                transaction: Some(Box::new(NewTransaction {
                    account_id: Some(account_id.clone()),
                    date: Some(Utc::now().date_naive().format("%Y-%m-%d").to_string()),
                    amount: Some(0),
                    payee_id: None,
                    payee_name: Some(Some(wallet.to_string())),
                    cleared: Some(TransactionClearedStatus::Cleared),
                    ..Default::default()
                })),
                transactions: None,
            },
        )
        .await
        .map_err(|e| SyncError::YnabApi(e.to_string()))?;

        response.data.transaction.unwrap()
    };

    let total = if rate != 1.0 { total / rate } else { total };
    let total = (total * 1000.0).ceil() as i64;

    let put_txn: PutTransactionWrapper = PutTransactionWrapper {
        transaction: Box::new(ExistingTransaction {
            amount: Some(total),
            date: Some(Utc::now().date_naive().format("%Y-%m-%d").to_string()),
            cleared: Some(TransactionClearedStatus::Cleared),
            ..Default::default()
        }),
    };

    update_transaction(config, budget_id, &txn.id.to_string(), put_txn)
        .await
        .map_err(|e| SyncError::YnabApi(e.to_string()))?;

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

fn get_env_var_or_default<T: From<String>>(key: &str, default: T) -> T {
    env::var(key).map(Into::into).unwrap_or(default)
}

fn setup_tracing() {
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
