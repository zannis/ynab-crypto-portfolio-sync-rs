use serde::Deserialize;
use std::error::Error;

pub async fn get_total_from_coinlore(
    wallet: &str,
) -> Result<Option<f64>, Box<dyn Error + Send + Sync>> {
    let balance_satoshi = reqwest::Client::new()
        .get(&format!(
            "https://blockchain.info/q/addressbalance/{wallet}"
        ))
        .send()
        .await?
        .json::<i32>()
        .await?;

    let price = get_bitcoin_price_usd().await?;

    Ok(Some((balance_satoshi as f64 / 100_000_000.0) * price))
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct CoinloreTickerResponse {
    id: String,
    symbol: String,
    name: String,
    nameid: String,
    rank: i32,
    price_usd: String,
    percent_change_24h: String,
    percent_change_1h: String,
    percent_change_7d: String,
    price_btc: String,
    market_cap_usd: String,
    volume24: f32,
    volume24a: f32,
    csupply: String,
    tsupply: String,
    msupply: String,
}

pub async fn get_bitcoin_price_usd() -> Result<f64, Box<dyn Error + Send + Sync>> {
    let client = reqwest::Client::new();

    let response = client
        .get("https://api.coinlore.net/api/ticker/?id=90")
        .send()
        .await?;

    let data = response.json::<Vec<CoinloreTickerResponse>>().await?;

    Ok(data[0].price_usd.parse::<f64>()?)
}
