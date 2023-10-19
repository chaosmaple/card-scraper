use crate::ws_card_scraper::ws_card::*;
use std::error::Error;
use thirtyfour::{By, WebDriver, WebElement};

pub(crate) async fn scrape_card_props(driver: &WebDriver) -> Result<Card, Box<dyn Error>> {
    let mut card = Card::default();
    scrape_card_values(driver, &mut card).await?;
    Ok(card)
}

async fn scrape_card_values(driver: &WebDriver, card: &mut Card) -> Result<(), Box<dyn Error>> {
    let card_details = driver.find(By::Css("#cardDetail .card-detail-table")).await?;
    let table_rows = card_details
        .find_all(By::Css("tr:not(.first) th, tr:not(.first) td"))
        .await?;
    let table_rows_chunks = table_rows.chunks(2);
    for chunk in table_rows_chunks {
        if chunk.len() != 2 {
            println!("partial chunk: {}", &chunk[0].text().await?);
        }
        match_by_pair(&chunk[0], &chunk[1], card).await?;
    }
    card.image = scrape_image(&card_details).await?;
    scrape_card_name(card, &card_details).await?;
    Ok(())
}

async fn match_by_pair(
    th: &WebElement,
    td: &WebElement,
    card: &mut Card,
) -> Result<(), Box<dyn Error>> {
    let label = th.text().await?;
    match label.as_str() {
        "カード番号" => card.card_no = parse_text(td).await?,
        "商品名" => card.product = parse_text(td).await?,
        "ネオスタンダード区分" => card.expansion = parse_text(td).await?,
        "作品番号" => card.expansion_id = parse_text(td).await?,
        "レアリティ" => card.rarity = parse_text(td).await?,
        "サイド" => card.side = parse_side(td).await?,
        "種類" => card.card_type = parse_card_type(td).await?,
        "色" => card.color = parse_color(td).await?,
        "レベル" => card.level = parse_number(td).await?,
        "コスト" => card.cost = parse_number(td).await?,
        "パワー" => card.power = parse_number(td).await?,
        "ソウル" => card.soul = parse_soul(td).await?,
        "トリガー" => card.trigger = parse_trigger(td).await?,
        "特徴" => card.special_attribute = parse_special_attribute(td).await?,
        // todo: parse the trigger img as well
        "テキスト" => card.text = parse_text(td).await?,
        "フレーバー" => card.flavor_text = parse_text(td).await?,
        "イラストレーター" => card.illustrator = parse_text(td).await?,
        _ => (),
    }
    Ok(())
}

async fn parse_text(td: &WebElement) -> Result<String, Box<dyn Error>> {
    let text = td.text().await?;
    Ok(trim_and_replace(&text))
}

async fn parse_number(td: &WebElement) -> Result<u16, Box<dyn Error>> {
    let td_text = td.text().await?.trim().to_string();
    if td_text.chars().all(|c| c.is_digit(10)) {
        Ok(td_text.parse()?)
    } else {
        Ok(0)
    }
}

async fn parse_side(td: &WebElement) -> Result<WSCardSide, Box<dyn Error>> {
    let image = td.find(By::Css("img")).await?;
    let side = match image.attr("src").await? {
        Some(str) => match str.as_str() {
            "/wordpress/wp-content/images/cardlist/_partimages/w.gif" => WSCardSide::Weiß,
            "/wordpress/wp-content/images/cardlist/_partimages/s.gif" => WSCardSide::Schwarz,
            _ => unreachable!("Unknown side"),
        },
        None => return Err("No src attribute found for image".into()),
    };
    Ok(side)
}

async fn parse_card_type(td: &WebElement) -> Result<WSCardType, Box<dyn Error>> {
    let card_type = match parse_text(td).await?.as_str() {
        "キャラ" => WSCardType::Character,
        "イベント" => WSCardType::Event,
        "クライマックス" => WSCardType::Climax,
        _ => unreachable!("Unknown card type"),
    };
    Ok(card_type)
}

async fn parse_color(td: &WebElement) -> Result<WSCardColor, Box<dyn Error>> {
    let image = td.find(By::Css("img")).await?;
    let side = match image.attr("src").await? {
        Some(str) => match str.as_str() {
            "/wordpress/wp-content/images/cardlist/_partimages/red.gif" => WSCardColor::Red,
            "/wordpress/wp-content/images/cardlist/_partimages/yellow.gif" => WSCardColor::Yellow,
            "/wordpress/wp-content/images/cardlist/_partimages/blue.gif" => WSCardColor::Blue,
            "/wordpress/wp-content/images/cardlist/_partimages/green.gif" => WSCardColor::Green,
            "/wordpress/wp-content/images/cardlist/_partimages/purple.gif" => WSCardColor::Purple,
            _ => unreachable!("Unknown color"),
        },
        None => return Err("No src attribute found for image".into()),
    };
    Ok(side)
}

async fn parse_soul(td: &WebElement) -> Result<u8, Box<dyn Error>> {
    let souls = td.find_all(By::Css("img")).await?;
    Ok(souls.len() as u8)
}

async fn parse_special_attribute(td: &WebElement) -> Result<Vec<String>, Box<dyn Error>> {
    let attrs = td.text().await?;
    Ok(attrs.split("・").map(|s| s.to_string()).collect())
}

async fn parse_trigger(td: &WebElement) -> Result<WSCardTrigger, Box<dyn Error>> {
    let triggers = td.find_all(By::Css("img")).await?;
    if triggers.is_empty() {
        return Ok(WSCardTrigger::None);
    };
    let trigger = match triggers
        .last()
        .unwrap()
        .attr("src")
        .await?
        .unwrap()
        .split("/")
        .last()
        .unwrap()
    {
        "soul.gif" => {
            if triggers.len() == 2 {
                WSCardTrigger::DoubleSoul
            } else {
                WSCardTrigger::Soul
            }
        }
        "stock.gif" => WSCardTrigger::Pool,
        "salvage.gif" => WSCardTrigger::Comeback,
        "bounce.gif" => WSCardTrigger::Return,
        "draw.gif" => WSCardTrigger::Draw,
        "treasure.gif" => WSCardTrigger::Treasure,
        "shot.gif" => WSCardTrigger::Shot,
        "gate.gif" => WSCardTrigger::Gate,
        "standby.gif" => WSCardTrigger::Standby,
        "choice.gif" => WSCardTrigger::Choice,
        _ => WSCardTrigger::None,
    };
    Ok(trigger)
}

async fn scrape_image(table: &WebElement) -> Result<String, Box<dyn Error>> {
    let image = table
        .find(By::Css(".card-detail-table .graphic img"))
        .await?;
    let src = match image.attr("src").await? {
        Some(src) => src,
        None => return Err("No src attribute found for image".into()),
    };
    Ok(src)
}

async fn scrape_card_name(card: &mut Card, table: &WebElement) -> Result<(), Box<dyn Error>> {
    let names = table
        .find(By::Css("tr.first td:last-child"))
        .await?
        .text()
        .await?;
    let spilted = names.split("\n").collect::<Vec<&str>>();
    card.card_name = trim_and_replace(spilted[0]);
    card.card_name_kana = trim_and_replace(spilted[1]);
    Ok(())
}

fn trim_and_replace(text: &str) -> String {
    text.trim().replace("\u{3000}", " ")
}
