use crate::{utils::initialize_driver, ws_card_scraper::scraper::scrape_card_props};
use std::error::Error;
use std::thread;
use std::time::Duration;
use thirtyfour::{By, WebDriver};

use super::ws_card::WSCard;

pub(crate) async fn get_single_card(url: &str) -> Result<WSCard, Box<dyn Error>> {
    let driver = initialize_driver().await?;
    driver.goto(url).await?;

    scrape_card_props(&driver).await
}

pub(crate) async fn scrape_ws_cards_from_title(
    title: &str,
    count: u16,
) -> Result<(), Box<dyn Error>> {
    let driver = initialize_driver().await?;
    driver.goto("https://ws-tcg.com/cardlist/").await?;

    click_into_title(&driver, title).await?;
    click_into_by_css(
        &driver,
        "#searchResults > div > table > tbody > tr:nth-child(1) > td > h4 > a",
    )
    .await?;

    thread::sleep(Duration::from_secs(2));

    let timestamp = chrono::Local::now().format("%Y%m%d%H%M%S");
    let file_name = format!("cards_{}.csv", timestamp);
    let mut wtr = csv::Writer::from_path(file_name)?;

    for _ in 0..count {
        let link = driver.find_all(By::Css(".card-detail-neighbor *")).await?;
        if link[1].tag_name().await? == "span" {
            break;
        }
        let card = scrape_card_props(&driver).await?;
        click_into_by_css(&driver, ".card-detail-neighbor *:last-child").await?;
        wtr.serialize(&card)?;
    }
    wtr.flush()?;
    Ok(())
}

async fn click_into_title(driver: &WebDriver, title: &str) -> Result<(), Box<dyn Error>> {
    let title_container = driver.find(By::Id("titleNumberList")).await?;
    title_container
        .find(By::LinkText(title))
        .await?
        .click()
        .await?;
    Ok(())
}

async fn click_into_by_css(driver: &WebDriver, css: &str) -> Result<(), Box<dyn Error>> {
    driver.find(By::Css(css)).await?.click().await?;
    Ok(())
}
