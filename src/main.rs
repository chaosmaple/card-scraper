pub mod utils;
pub mod ws_card_scraper;

use std::error::Error;
use ws_card_scraper::scrape_runner::{scrape_ws_cards_from_title, get_single_card};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    scrape_ws_cards_from_title("幻影ヲ駆ケル太陽", 10).await?;
    get_single_card("https://ws-tcg.com/cardlist/?cardno=IM/S07-100&l").await?;
    Ok(())
}
