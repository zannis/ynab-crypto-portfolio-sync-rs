use crate::bitcoin::get_bitcoin_price_usd;
use crate::get_env_var;
use binance_api::apis::configuration::{ApiKey, Configuration};
use binance_api::apis::wallet_api::sapi_v1_asset_wallet_balance_get;
use std::error::Error;
use tokio::task::spawn_blocking;

fn config() -> Configuration {
    let mut config = Configuration::default();

    config.api_key = Some(ApiKey {
        key: get_env_var("BINANCE_API_KEY"),
        prefix: None,
    });

    config
}

pub async fn get_binance_wallet_value() -> Result<f64, Box<dyn Error + Send + Sync>> {
    let secret_key = get_env_var::<String>("BINANCE_SECRET_KEY");

    let balance: f64 = {
        let balances =
            sapi_v1_asset_wallet_balance_get(&config(), secret_key.as_str(), None).await?;

        balances
            .iter()
            .map(|b| b.balance.parse::<f64>().unwrap())
            .sum()
    };

    let bitcoin_price = spawn_blocking(|| get_bitcoin_price_usd()).await??;

    Ok(balance * bitcoin_price)
}
