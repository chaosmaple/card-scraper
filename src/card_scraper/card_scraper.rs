use crate::utils::initialize_driver;
use std::error::Error;
use thirtyfour::{By, WebDriver, WebElement};

#[derive(Debug, Clone, Eq, PartialEq)]
enum WSCardType {
    Character,
    Event,
    Climax,
}

impl Default for WSCardType {
    fn default() -> Self {
        WSCardType::Character
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum WSCardSide {
    Weiß,
    Schwarz,
}

impl Default for WSCardSide {
    fn default() -> Self {
        WSCardSide::Weiß
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum WSCardColor {
    Red,
    Blue,
    Green,
    Yellow,
    Purple,
    Colorless,
}

impl Default for WSCardColor {
    fn default() -> Self {
        WSCardColor::Colorless
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum WSCardTrigger {
    None,
    Soul,
    DoubleSoul,
    Pool,
    Comeback,
    Return,
    Draw,
    Treasure,
    Shot,
    Gate,
    Choice,
    Standby,
}

impl Default for WSCardTrigger {
    fn default() -> Self {
        WSCardTrigger::None
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
struct Card {
    image: String,
    card_name: String,
    card_name_kana: String,
    card_no: String,
    product: String,
    expansion: String,
    expansion_id: String,
    rarity: String,
    side: WSCardSide,
    card_type: WSCardType,
    color: WSCardColor,
    level: u16,
    cost: u16,
    power: u16,
    soul: u8,
    trigger: WSCardTrigger,
    special_attribute: Vec<String>,
    text: String,
    flavor_text: String,
    illustrator: String,
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
    let card = scrape_card_props(&driver).await?;
    println!("Card: {:?}", card);
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

async fn scrape_card_props(driver: &WebDriver) -> Result<Card, Box<dyn Error>> {
    let mut card = Card::default();
    scrape_card_values(driver, &mut card).await?;
    Ok(card)
}

async fn scrape_card_values(driver: &WebDriver, card: &mut Card) -> Result<(), Box<dyn Error>> {
    let card_details = driver.find(By::Css("#cardDetail")).await?;
    let table_rows = card_details.find_all(By::Css("tr:not(.first)")).await?;

    for row in table_rows {
        println!("row: {:?}", row.find(By::Css("th")).await?.text().await?.as_str());
        match row.find(By::Css("th")).await?.text().await?.as_str() {
            "カード番号" => card.card_no = parse_text(&row).await?,
            "商品名" => card.product = parse_text(&row).await?,
            "ネオスタンダード区分" => card.expansion = parse_text(&row).await?,
            "作品番号" => card.expansion_id = parse_text(&row).await?,
            "レアリティ" => card.rarity = parse_text(&row).await?,
            "サイド" => card.side = parse_side(&row).await?,
            "種類" => card.card_type = parse_card_type(&row).await?,
            "色" => card.color = parse_color(&row).await?,
            "レベル" => card.level = parse_number(&row).await?,
            "コスト" => card.cost = parse_number(&row).await?,
            "パワー" => card.power = parse_number(&row).await?,
            "ソウル" => card.soul = parse_soul(&row).await?,
            "トリガー" => card.trigger = parse_trigger(&row).await?,
            "特徴" => card.special_attribute = parse_special_attribute(&row).await?,
            "テキスト" => card.text = parse_text(&row).await?,
            "フレーバー" => card.flavor_text = parse_text(&row).await?,
            label => println!("no match label: {}", label),
        }
    }
    card.image = scrape_image(&card_details).await?;
    Ok(())
}

async fn parse_text(row: &WebElement) -> Result<String, Box<dyn Error>> {
    let text = row.find(By::Css("td")).await?.text().await?;
    Ok(text)
}

async fn parse_number(row: &WebElement) -> Result<u16, Box<dyn Error>> {
    let text = row.find(By::Css("td")).await?.text().await?;
    println!("text: {}", text);
    Ok(text.parse()?)
}

async fn parse_side(row: &WebElement) -> Result<WSCardSide, Box<dyn Error>> {
    let image = row.find(By::Css("td img")).await?;
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

async fn parse_card_type(row: &WebElement) -> Result<WSCardType, Box<dyn Error>> {
    let card_type = match parse_text(row).await?.as_str() {
        "キャラクター" => WSCardType::Character,
        "イベント" => WSCardType::Event,
        "クライマックス" => WSCardType::Climax,
        _ => unreachable!("Unknown card type"),
    };
    Ok(card_type)
}

async fn parse_color(row: &WebElement) -> Result<WSCardColor, Box<dyn Error>> {
    let image = row.find(By::Css("td img")).await?;
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

async fn parse_soul(row: &WebElement) -> Result<u8, Box<dyn Error>> {
    let souls = row.find_all(By::Css("td img")).await?;
    println!("souls: {:?}", souls.len());
    Ok(souls.len() as u8)
}

async fn parse_special_attribute(row: &WebElement) -> Result<Vec<String>, Box<dyn Error>> {
    let attrs = row.find(By::Css("td")).await?.text().await?;
    Ok(attrs.split("・").map(|s| s.to_string()).collect())
}

async fn parse_trigger(row: &WebElement) -> Result<WSCardTrigger, Box<dyn Error>> {
    let triggers = row.find_all(By::Css("td img")).await?;
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
