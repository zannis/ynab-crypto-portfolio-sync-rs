use chrono::{NaiveDate, Utc};
use dotenv::dotenv;
use headless_chrome::Browser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::path::Path;
use tracing::info;
use ynab_api::apis::accounts_api::get_accounts;
use ynab_api::apis::budgets_api::get_budgets;
use ynab_api::apis::configuration::Configuration;
use ynab_api::apis::transactions_api::{get_transactions_by_account, update_transaction};
use ynab_api::models::{ExistingTransaction, PutTransactionWrapper, TransactionClearedStatus};

#[derive(Debug, Serialize, Deserialize)]
struct ExchangeRateResponse {
    base: String,
    date: NaiveDate,
    rates: HashMap<String, f32>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    setup_tracing();

    let wallets = get_env_var::<String>("WALLETS")
        .split(",")
        .map(Into::into)
        .collect::<Vec<String>>();

    info!("Syncing...");

    let rate = get_exchange_rate("EUR", "USD").await.unwrap();

    info!("Got exchange rate: {}", rate);

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

    for wallet in wallets {
        info!("Looking for {wallet} txn...");

        let txn = {
            let txn = txns
                .data
                .transactions
                .iter()
                .find(|t| t.payee_name == Some(Some(wallet.clone())));

            if txn.is_none() {
                info!("No txn found for {wallet}");
                continue;
            }

            let txn = txn.unwrap();

            info!("Found txn: {:?}", txn.payee_name);

            txn
        };

        let total_in_usd = {
            let total_in_usd = get_total_from_debank(&wallet).unwrap();

            if total_in_usd.is_none() {
                info!("No total found in debank. Skipping...");
                continue;
            }

            total_in_usd.unwrap()
        };

        let total = total_in_usd as f32 / rate * 1000_f32;

        let put_txn: PutTransactionWrapper = PutTransactionWrapper {
            transaction: Box::new(ExistingTransaction {
                amount: Some(total as i64),
                date: Some(Utc::now().date_naive().format("%Y-%m-%d").to_string()),
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

fn get_total_from_debank(wallet: &str) -> Result<Option<i64>, Box<dyn Error>> {
    let browser = Browser::default()?;

    let tab = browser.new_tab()?;

    tab.navigate_to(&format!("https://debank.com/profile/{wallet}"))?;

    tab.wait_until_navigated()?;

    let element = tab.find_element("[class^='HeaderInfo_totalAssetInner__']")?;

    let text = element.get_inner_text()?;

    // debank returns the total in USD, without decimals
    let first_line = text.lines().next().map(Into::into).and_then(|s: String| {
        s.trim_start_matches("$")
            .replace(",", "")
            .parse::<i64>()
            .ok()
    });

    Ok(first_line)
}

async fn get_exchange_rate(base: &str, to: &str) -> Result<f32, Box<dyn Error>> {
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

    info!("{response:?}");

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
