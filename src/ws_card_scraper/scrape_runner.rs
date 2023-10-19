use crate::{utils::initialize_driver, ws_card_scraper::scraper::scrape_card_props};
use std::error::Error;
use thirtyfour::{By, WebDriver};

pub(crate) async fn scrap_single_card(url: &str) -> Result<(), Box<dyn Error>> {
    let driver = initialize_driver().await?;
    driver.goto(url).await?;

    let card = scrape_card_props(&driver).await?;
    println!("Card: {:?}", card);
    Ok(())
}

pub(crate) async fn scrape_ws_cards_from_title(title: &str) -> Result<(), Box<dyn Error>> {
    let driver = initialize_driver().await?;
    driver.goto("https://ws-tcg.com/cardlist/").await?;

    click_into_title(&driver, title).await?;
    click_into_by_css(
        &driver,
        "#searchResults > div > table > tbody > tr:nth-child(1) > td > h4 > a",
    )
    .await?;

    loop {
        let link = driver.find_all(By::Css(".card-detail-neighbor *")).await?;
        if link[1].tag_name().await? == "span" {
            break;
        }
        let card = scrape_card_props(&driver).await?;
        click_into_by_css(&driver, ".card-detail-neighbor *:last-child").await?;
        println!("Card: {:?}", card);
    }
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
