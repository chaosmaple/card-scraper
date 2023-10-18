pub mod utils;
pub mod card_scraper;

use std::error::Error;

use card_scraper::card_scraper::scrape_ws_cards_from_title;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    scrape_ws_cards_from_title("アイドルマスター").await.unwrap();
    Ok(())
}
