mod api;
mod models;
mod vendoo;
mod terminal;
mod local_db;

use dotenv::dotenv;
use std::env;
use std::error::Error;
use crate::terminal::terminal_interface;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let wc_api_url = env::var("WC_API_URL").expect("WC_API_URL not set");
    let wc_consumer_key = env::var("WC_CONSUMER_KEY").expect("WC_CONSUMER_KEY not set");
    let wc_consumer_secret = env::var("WC_CONSUMER_SECRET").expect("WC_CONSUMER_SECRET not set");

    terminal_interface(&wc_api_url, &wc_consumer_key, &wc_consumer_secret).await?;

    Ok(())
}
