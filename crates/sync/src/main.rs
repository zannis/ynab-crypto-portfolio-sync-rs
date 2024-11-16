use chrono::{NaiveDate, Utc};
use dotenv::dotenv;
use headless_chrome::Browser;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::path::Path;
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
struct ExchangeRateResponse {
    base: String,
    date: NaiveDate,
    rates: HashMap<String, f32>,
}

struct YnabTransaction {}

fn main() {
    dotenv().ok();
    setup_tracing();

    let wallets = get_env_var::<String>("WALLETS")
        .split(",")
        .map(Into::into)
        .collect::<Vec<String>>();

    info!("Syncing...");

    let rate = get_exchange_rate("EUR", "USD").unwrap();

    info!("{rate}");

    for wallet in wallets {
        info!("{wallet}");

        // let total = get_total_from_debank(wallet).unwrap();
        //
        // if let Some(total) = total {
        //     println!("{total}");
        // } else {
        //     println!("No total found in debank");
        //     std::process::exit(1);
        // }
        //
        // let txn = get_ynab_transaction(env::var("YNAB_TRANSACTION").unwrap().as_str()).unwrap();
        //
        // update_ynab_crypto_txn(&txn, total.unwrap()).unwrap();
    }
}

fn get_total_from_debank(wallet: &str) -> Result<Option<i32>, Box<dyn Error>> {
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
            .parse::<i32>()
            .ok()
    });

    Ok(first_line)
}

fn get_exchange_rate(base: &str, to: &str) -> Result<f32, Box<dyn Error>> {
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

    let client = reqwest::blocking::Client::new();

    let req = client
        .get("https://api.frankfurter.app/latest")
        .query(&[("access_key", &env::var("EXCHANGE_RATES_IO_KEY")?)])
        .query(&[("base", &base)])
        .query(&[("symbols", &to)]);

    let response = req.send()?;

    let response = response.json::<ExchangeRateResponse>()?;

    info!("{response:?}");

    info!("Exchange rate updated");

    std::fs::write(file_path, serde_json::to_string_pretty(&response)?)?;

    Ok(response.rates.get(to).unwrap().clone())
}

fn get_ynab_transactions() -> Result<Value, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();

    let budget_id = env::var("YNAB_BUDGET")?;

    let account_id = env::var("YNAB_ACCOUNT")?;

    let response = client
        .get(&format!(
            "https://api.ynab.com/v1/budgets/{budget_id}/accounts/{account_id}/transactions"
        ))
        .header("Authorization", format!("Bearer {}", env::var("YNAB_KEY")?))
        .send()?;

    let response = response.json()?;

    println!("{}", serde_json::to_string_pretty(&response).unwrap());

    Ok(response)
}

fn get_ynab_transaction(txn_id: &str) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();

    let budget_id = env::var("YNAB_BUDGET")?;

    let account_id = env::var("YNAB_ACCOUNT")?;

    let response = client.get(&format!("https://api.ynab.com/v1/budgets/{budget_id}/accounts/{account_id}/transactions/{txn_id}"))
        .header("Authorization", format!("Bearer {}", env::var("YNAB_KEY")?))
        .send()?;

    let response = response.json::<Value>()?;

    let txn = response
        .get("data")
        .and_then(|d| d.get("transaction"))
        .ok_or("No transaction found")?
        .clone();

    Ok(txn)
}

fn update_ynab_crypto_txn(txn: &Value, new_total: i32) -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();

    let budget_id = env::var("YNAB_BUDGET")?;

    let transaction_id = env::var("YNAB_TRANSACTION")?;

    let exchange_rate = get_exchange_rate("EUR", "USD")?;

    let amount = new_total as f32 / exchange_rate * 1000_f32;

    println!("exchange_rate: {}", exchange_rate);

    let new_txn = {
        let mut txn = txn.clone();
        txn["amount"] = serde_json::Value::from(amount.ceil() as i32);
        txn["date"] =
            serde_json::Value::from(Utc::now().date_naive().format("%Y-%m-%d").to_string());
        txn["cleared"] = serde_json::Value::from("cleared");
        txn
    };

    let body = serde_json::json!({
        "transaction": new_txn
    });

    println!("new_txn: {}", serde_json::to_string_pretty(&body).unwrap());

    let response = client
        .put(&format!(
            "https://api.ynab.com/v1/budgets/{budget_id}/transactions/{transaction_id}"
        ))
        .header("Authorization", format!("Bearer {}", env::var("YNAB_KEY")?))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&body).unwrap())
        .send()?;

    let response: Value = response.json()?;

    println!("put: {}", serde_json::to_string_pretty(&response).unwrap());

    Ok(())
}

fn get_env_var<T: From<String>>(key: &str) -> T {
    env::var(key)
        .expect(&format!("Missing environment variable \"{}\"", key))
        .into()
}

fn get_env_var_or_default<T: From<String>>(key: &str, default: T) -> T {
    env::var(key).ok().map_or(default, Into::into)
}

fn setup_tracing() {
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
