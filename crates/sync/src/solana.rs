use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;
use tracing::info;

const TIMEOUT_SECONDS: u64 = 30;

pub async fn get_solana_wallet_net_worth(wallet: &str) -> Result<Option<f64>, Box<dyn Error>> {
    #[cfg(feature = "headless")]
    return get_net_worth_from_step_finance_with_headless_chrome(wallet).await;

    #[cfg(feature = "docker")]
    return get_net_worth_from_step_finance_with_fantoccini(wallet).await;
}

#[cfg(feature = "headless")]
pub async fn get_net_worth_from_step_finance_with_headless_chrome(
    wallet: &str,
) -> Result<Option<f64>, Box<dyn Error>> {
    use headless_chrome::{Browser, LaunchOptions};
    let browser = Browser::new(
        LaunchOptions::default_builder()
            .args(vec![
                "--headless".as_ref(),
                "--blink-settings=imagesEnabled=false".as_ref(),
            ])
            .build()?,
    )?;

    let tab = browser.new_tab()?;

    info!("Getting balance for {wallet} from Step Finance...");

    let tab = tab.navigate_to(&format!(
        "https://app.step.finance/en/dashboard?watching={wallet}"
    ))?;

    // wait for the async js to finish loading before grabbing the value
    sleep(Duration::from_secs(TIMEOUT_SECONDS)).await;

    let text = tab.get_title()?;

    // debank returns the total in USD, without decimals
    let first_line = text.lines().next().map(Into::into).and_then(|s: String| {
        s.trim_start_matches("$")
            .trim_end_matches(" USD | Step")
            .replace(",", "")
            .parse::<f64>()
            .ok()
    });

    Ok(first_line)
}

#[cfg(feature = "docker")]
pub async fn get_net_worth_from_step_finance_with_fantoccini(
    wallet: &str,
) -> Result<Option<f64>, Box<dyn Error>> {
    use fantoccini::wd::Capabilities;
    use fantoccini::{ClientBuilder, Locator};

    let webdriver_url = std::env::var("WEBDRIVER_URL").unwrap();

    let cap: Capabilities = serde_json::from_value(serde_json::json!({
        "browserName": "chrome",
        "goog:chromeOptions": {
            "args": [
                "--headless",
                "--blink-settings=imagesEnabled=false"
            ]
        }
    }))?;

    let c = ClientBuilder::native()
        .capabilities(cap)
        .connect(&webdriver_url)
        .await
        .expect("Failed to connect to WebDriver");

    c.goto(&format!(
        "https://app.step.finance/en/dashboard?watching={wallet}"
    ))
    .await?;

    info!("Getting balance for {wallet} from Step Finance...");
    // wait for the async js to finish loading before grabbing the value
    sleep(Duration::from_secs(TIMEOUT_SECONDS)).await;

    let text = c.find(Locator::Css("span.net-worth")).await?.text().await?;

    // debank returns the total in USD, without decimals
    let first_line = text.lines().next().map(Into::into).and_then(|s: String| {
        s.trim_start_matches("$")
            .replace(",", "")
            .parse::<f64>()
            .ok()
    });

    c.close().await?;

    Ok(first_line)
}
