use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
struct ExchangeRateResponse {
    base: String,
    date: NaiveDate,
    rates: HashMap<String, f64>,
}

const FRANKFURTER_URI: &str = "https://api.frankfurter.app/latest";

const EXCHANGE_FILE_PATH: &str = "exchange_rates.json";

pub async fn get_exchange_rate(base: &str, to: &str) -> Result<f64, Box<dyn Error>> {
    if base == "USD" {
        return Ok(1.0);
    }

    let file_path = Path::new(EXCHANGE_FILE_PATH);

    let exchange_rate: Option<ExchangeRateResponse> =
        if let Ok(file) = std::fs::File::open(file_path) {
            serde_json::from_reader(file).ok()
        } else {
            std::fs::File::create_new(file_path)?;
            None
        };

    if let Some(exchange_rate) = exchange_rate {
        if Utc::now().date_naive().eq(&exchange_rate.date)
            && exchange_rate.base == base
            && exchange_rate.rates.get(to).is_some()
        {
            return Ok(exchange_rate.rates.get(to).unwrap().clone());
        }
    }

    let client = reqwest::Client::new();

    let response = client
        .get(FRANKFURTER_URI)
        .query(&[("base", &base)])
        .query(&[("symbols", &to)])
        .send()
        .await?;

    let response = response.json::<ExchangeRateResponse>().await?;

    info!("Exchange rate updated");

    std::fs::write(file_path, serde_json::to_string_pretty(&response)?)?;

    let rate = response
        .rates
        .get(to)
        .expect(&format!("No rate found for {to}"));

    Ok(rate.clone())
}
