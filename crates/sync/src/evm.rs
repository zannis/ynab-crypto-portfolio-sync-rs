use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;
use tracing::info;

pub async fn get_total_from_debank(wallet: &str) -> Result<Option<f64>, Box<dyn Error>> {
    #[cfg(feature = "headless")]
    return get_total_from_debank_with_headless_chrome(wallet).await;

    #[cfg(feature = "docker")]
    return get_total_from_debank_with_fantoccini(wallet).await;
}

#[cfg(feature = "headless")]
pub async fn get_total_from_debank_with_headless_chrome(
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

    let tab = tab
        .navigate_to(&format!("https://debank.com/profile/{wallet}"))?
        .wait_until_navigated()?;

    info!("Getting balance for {wallet} from Debank...");
    // wait 10 seconds for the async js to finish loading before grabbing the value
    sleep(Duration::from_secs(10)).await;

    let text = tab
        .find_element("[class^='HeaderInfo_totalAssetInner__']")?
        .get_inner_text()?;

    // debank returns the total in USD, without decimals
    let first_line = text.lines().next().map(Into::into).and_then(|s: String| {
        s.trim_start_matches("$")
            .replace(",", "")
            .parse::<f64>()
            .ok()
    });

    Ok(first_line)
}

#[cfg(feature = "docker")]
pub async fn get_total_from_debank_with_fantoccini(
    wallet: &str,
) -> Result<Option<f64>, Box<dyn Error>> {
    use fantoccini::wd::Capabilities;
    use fantoccini::{ClientBuilder, Locator};

    let webdriver_url = std::env::var("WEBDRIVER_URL").unwrap();

    let cap: Capabilities = serde_json::from_value(serde_json::json!({
        "browserName": "chrome",
        "goog:chromeOptions": {
            "args": ["--headless", "--blink-settings=imagesEnabled=false"]
        }
    }))?;

    let c = ClientBuilder::native()
        .capabilities(cap)
        .connect(&webdriver_url)
        .await
        .expect("Failed to connect to WebDriver");

    c.goto(&format!("https://debank.com/profile/{wallet}"))
        .await?;

    info!("Getting balance for {wallet} from Debank...");
    // wait 10 seconds for the async js to finish loading before grabbing the value
    sleep(Duration::from_secs(10)).await;

    let text = c
        .find(Locator::Css("[class^='HeaderInfo_totalAssetInner__']"))
        .await?
        .text()
        .await?;

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
