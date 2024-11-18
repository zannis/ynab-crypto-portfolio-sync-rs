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
use tokio::join;
use tracing::{info, warn};
use ynab_api::apis::accounts_api::get_accounts;
use ynab_api::apis::budgets_api::get_budgets;
use ynab_api::apis::configuration::Configuration;
use ynab_api::apis::transactions_api::{
    create_transaction, get_transactions_by_account, update_transaction,
};
use ynab_api::models::{
    ExistingTransaction, NewTransaction, PostTransactionsWrapper, PutTransactionWrapper,
    TransactionClearedStatus,
};

#[tokio::main]
async fn main() {
    dotenv().ok();
    setup_tracing();

    let ynab_key = get_env_var::<String>("YNAB_KEY");
    let ynab_account_name = get_env_var::<String>("YNAB_ACCOUNT_NAME");

    info!("Getting YNAB account...");

    let config = ynab_config(&ynab_key);

    let budget = {
        let budgets = get_budgets(&config, Some(true))
            .await
            .expect("Failed to get YNAB budgets");

        info!(
            "Got {} budgets. Using the default or first one...",
            budgets.data.budgets.len()
        );

        budgets
            .data
            .default_budget
            .clone()
            .unwrap_or_else(|| Box::new(budgets.data.budgets.first().unwrap().clone()))
    };

    let currency = budget.currency_format.unwrap().unwrap().iso_code;

    info!("Getting accounts for budget {}...", budget.id);

    let account = {
        let accounts = get_accounts(&config, &budget.id.to_string(), None)
            .await
            .expect("Failed to get YNAB accounts");

        info!("Got {} accounts", accounts.data.accounts.len());

        accounts
            .data
            .accounts
            .into_iter()
            .find(|a| a.name.eq(&ynab_account_name))
            .expect(&format!(
                "No account found that matches: {ynab_account_name}. Make sure you have set the YNAB_ACCOUNT_NAME environment variable to an existing account."
            ))
    };

    let txns = get_transactions_by_account(
        &config,
        &budget.id.to_string(),
        &account.id.to_string(),
        None,
        None,
        None,
    )
    .await
    .expect("Failed to get YNAB transactions");

    info!("Getting exchange rate...");

    let rate = get_exchange_rate(&currency, "USD").await.unwrap();

    let mut values = {
        let evm_wallets = get_env_var::<String>("EVM_WALLETS")
            .split(",")
            .map(Into::into)
            .collect::<Vec<String>>();

        let bitcoin_wallets = get_env_var::<String>("BTC_WALLETS")
            .split(",")
            .map(Into::into)
            .collect::<Vec<String>>();

        info!("Getting wallet values...");

        let evm_values = async {
            let mut values = HashMap::new();
            for wallet in &evm_wallets {
                if let Ok(Some(total)) = get_total_from_debank(wallet).await {
                    values.insert(wallet.clone(), total);
                } else {
                    warn!("Could not get balance for {wallet}.")
                }
            }
            values
        };

        let bitcoin_values = async {
            let mut values = HashMap::new();
            for wallet in &bitcoin_wallets {
                if let Ok(Some(total)) = get_total_from_coinlore(wallet).await {
                    values.insert(wallet.clone(), total);
                } else {
                    warn!("Could not get balance for {wallet}")
                }
            }
            values
        };

        let (evm_results, bitcoin_results) = join!(evm_values, bitcoin_values);

        evm_results
            .into_iter()
            .chain(bitcoin_results)
            .collect::<HashMap<String, f64>>()
    };

    info!("Getting binance wallet value...");

    if env::var("BINANCE_API_KEY").is_ok() {
        let binance_wallet_value = get_binance_wallet_value().await.unwrap();
        values.insert("Binance".to_string(), binance_wallet_value);
    }

    for (wallet, total) in values {
        info!("Looking for {wallet} txn...");

        let txn = {
            let txn = txns
                .data
                .transactions
                .iter()
                .find(|t| t.payee_name == Some(Some(wallet.clone())));

            if txn.is_some() {
                info!("Found txn: {:?}", txn.unwrap().payee_name);
                Box::new(txn.unwrap().clone())
            } else {
                info!("No txn found for {wallet}. Creating a new one...");

                let new_txn = create_transaction(
                    &config,
                    &budget.id.to_string(),
                    PostTransactionsWrapper {
                        transaction: Some(Box::new(NewTransaction {
                            account_id: Some(account.id),
                            date: Some(Utc::now().date_naive().format("%Y-%m-%d").to_string()),
                            amount: Some(0),
                            payee_id: None,
                            payee_name: Some(Some(wallet.clone())),
                            cleared: Some(TransactionClearedStatus::Cleared),
                            ..Default::default()
                        })),
                        transactions: None,
                    },
                )
                .await
                .expect("Failed to create YNAB transaction");

                new_txn.data.transaction.unwrap()
            }
        };

        let total = {
            let total = if currency != "USD" {
                total
            } else {
                total / rate
            };
            let total = total * 1000.0;
            total.ceil() as i64
        };

        let put_txn: PutTransactionWrapper = PutTransactionWrapper {
            transaction: Box::new(ExistingTransaction {
                amount: Some(total),
                date: Some(Utc::now().date_naive().format("%Y-%m-%d").to_string()),
                memo: None,
                cleared: Some(TransactionClearedStatus::Cleared),
                ..Default::default()
            }),
        };

        update_transaction(
            &config,
            &budget.id.to_string(),
            &txn.id.to_string(),
            put_txn,
        )
        .await
        .expect("Failed to update YNAB transaction");
    }
}

fn ynab_config(bearer_access_token: &str) -> Configuration {
    let mut config = Configuration::new();

    config.bearer_access_token = Some(bearer_access_token.to_owned());

    config
}

fn get_env_var<T: From<String>>(key: &str) -> T {
    env::var(key)
        .expect(&format!("Missing environment variable \"{}\"", key))
        .into()
}

fn setup_tracing() {
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
