use headless_chrome::Browser;
use std::error::Error;

pub async fn get_total_from_debank(wallet: &str) -> Result<Option<f64>, Box<dyn Error>> {
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
            .parse::<f64>()
            .ok()
    });

    Ok(first_line)
}
