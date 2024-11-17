mod binance;
mod bitcoin;
mod evm;

use crate::binance::get_binance_wallet_value;
use crate::bitcoin::get_total_from_coinlore;
use crate::evm::get_total_from_debank;
use chrono::{NaiveDate, Utc};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::path::Path;
use tokio::task::spawn_blocking;
use tracing::info;
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

#[derive(Debug, Serialize, Deserialize)]
struct ExchangeRateResponse {
    base: String,
    date: NaiveDate,
    rates: HashMap<String, f64>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    setup_tracing();

    info!("Getting YNAB account...");

    let ynab_key = get_env_var::<String>("YNAB_KEY");

    let config = ynab_config(&ynab_key);

    let budget = {
        let budgets = get_budgets(&config, Some(true))
            .await
            .expect("Failed to get YNAB budgets");

        info!(
            "Got {} budgets. Using the first one...",
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
        let accounts = get_accounts(&ynab_config(&ynab_key), &budget.id.to_string(), None)
            .await
            .expect("Failed to get YNAB accounts");

        info!("Got {} accounts", accounts.data.accounts.len());

        accounts
            .data
            .accounts
            .into_iter()
            .find(|a| a.name == "Crypto")
            .expect("No crypto account found")
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

    let rate = get_exchange_rate(&currency, "USD").await.unwrap();

    info!("Got exchange rate: {}", rate);

    let mut values = spawn_blocking(|| {
        let evm_wallets = get_env_var::<String>("EVM_WALLETS")
            .split(",")
            .map(Into::into)
            .collect::<Vec<String>>();

        let bitcoin_wallets = get_env_var::<String>("BITCOIN_WALLETS")
            .split(",")
            .map(Into::into)
            .collect::<Vec<String>>();

        info!("Getting EVM wallet values...");

        let evm_values = evm_wallets
            .iter()
            .filter_map(|wallet| {
                get_total_from_debank(&wallet)
                    .unwrap()
                    .map(|t| (wallet.clone(), t))
            })
            .collect::<HashMap<String, f64>>();

        info!("Getting bitcoin wallet values...");

        let bitcoin_values = bitcoin_wallets
            .iter()
            .filter_map(|wallet| {
                get_total_from_coinlore(&wallet)
                    .unwrap()
                    .map(|t| (wallet.clone(), t))
            })
            .collect::<HashMap<String, f64>>();

        evm_values
            .into_iter()
            .chain(bitcoin_values)
            .collect::<HashMap<String, f64>>()
    })
    .await
    .expect("Failed to get wallet value in USD");

    info!("Getting binance wallet value...");

    if env::var("BINANCE_API_KEY").is_ok() {
        values.insert(
            "Binance".to_string(),
            get_binance_wallet_value().await.unwrap(),
        );
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

async fn get_exchange_rate(base: &str, to: &str) -> Result<f64, Box<dyn Error>> {
    if base == "USD" {
        return Ok(1.0);
    }

    let file_path = Path::new("exchange_rates.json");

    let exchange_rate: Option<ExchangeRateResponse> =
        if let Ok(file) = std::fs::File::open(file_path) {
            serde_json::from_reader(file).ok()
        } else {
            std::fs::File::create_new(file_path)?;
            None
        };

    if let Some(exchange_rate) = exchange_rate {
        if Utc::now().date_naive().eq(&exchange_rate.date) && exchange_rate.base == base {
            return Ok(exchange_rate.rates.get(base).unwrap().clone());
        }
    }

    let client = reqwest::Client::new();

    let req = client
        .get("https://api.frankfurter.app/latest")
        .query(&[("base", &base)])
        .query(&[("symbols", &to)]);

    let response = req.send().await?;

    let response = response.json::<ExchangeRateResponse>().await?;

    info!("Exchange rate updated");

    std::fs::write(file_path, serde_json::to_string_pretty(&response)?)?;

    Ok(response.rates.get(to).unwrap().clone())
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
