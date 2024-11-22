use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
struct ExchangeRateResponse {
    base: String,
    date: NaiveDate,
    rates: HashMap<String, f64>,
}

const FRANKFURTER_URI: &str = "https://api.frankfurter.app/latest";

pub async fn get_exchange_rate(base: &str, to: &str) -> Result<f64, Box<dyn Error>> {
    if base == "USD" {
        return Ok(1.0);
    }

    let response = reqwest::Client::new()
        .get(FRANKFURTER_URI)
        .query(&[("base", &base)])
        .query(&[("symbols", &to)])
        .send()
        .await?;

    let response = response.json::<ExchangeRateResponse>().await?;

    info!("Exchange rate updated");

    let rate = response
        .rates
        .get(to)
        .expect(&format!("No rate found for {to}"));

    Ok(rate.clone())
}
