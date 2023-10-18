use thirtyfour::{prelude::WebDriverError, DesiredCapabilities, WebDriver};

pub(crate) async fn initialize_driver() -> Result<WebDriver, WebDriverError> {
    let driver = WebDriver::new("http://localhost:9515", DesiredCapabilities::chrome()).await?;
    driver.goto("https://ws-tcg.com/cardlist/").await?;
    Ok(driver)
}
