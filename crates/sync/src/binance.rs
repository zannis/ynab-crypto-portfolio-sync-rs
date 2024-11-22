use crate::bitcoin::get_bitcoin_price_usd;
use binance_api::apis::configuration::{ApiKey, Configuration};
use binance_api::apis::wallet_api::sapi_v1_asset_wallet_balance_get;
use std::error::Error;

fn config() -> Configuration {
    let mut config = Configuration::default();

    config.api_key = Some(ApiKey {
        key: std::env::var("BINANCE_API_KEY").expect("Binance API key should be set"),
        prefix: None,
    });

    config
}

pub async fn get_binance_wallet_value() -> Result<f64, Box<dyn Error + Send + Sync>> {
    let secret_key = std::env::var("BINANCE_SECRET_KEY").expect("Binance secret key should be set");

    let balance: f64 = {
        let balances =
            sapi_v1_asset_wallet_balance_get(&config(), secret_key.as_str(), None).await?;

        balances
            .iter()
            .map(|b| b.balance.parse::<f64>().unwrap())
            .sum()
    };

    let bitcoin_price = get_bitcoin_price_usd().await?;

    Ok(balance * bitcoin_price)
}
