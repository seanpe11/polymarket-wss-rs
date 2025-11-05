//! Example: Listen to Polymarket market data
//!
//! Usage:
//! export POLYMARKET_API_KEY="your-key"
//! export POLYMARKET_API_SECRET="your-secret"
//! export POLYMARKET_PASSPHRASE="your-passphrase"
//! cargo run --example listen

use polymarket_wss_rs::{MarketEvent, client::PolymarketWss};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load credentials from environment
    let api_key = env::var("POLYMARKET_API_KEY").expect("POLYMARKET_API_KEY not set");
    let api_secret = env::var("POLYMARKET_API_SECRET").expect("POLYMARKET_API_SECRET not set");
    let passphrase = env::var("POLYMARKET_PASSPHRASE").expect("POLYMARKET_PASSPHRASE not set");

    // Example token ID - replace with real one
    let asset_ids = vec![
        "71321045679252212594626385532706912750332728571942532289631379312455583992563".to_string(),
    ];

    println!("Connecting to Polymarket WebSocket...");
    let mut client = PolymarketWss::connect(api_key, api_secret, passphrase, asset_ids).await?;

    println!("Connected! Listening for events...\n");

    // Listen for events
    let mut event_count = 0;
    while let Ok(Some(event)) = client.next_event().await {
        event_count += 1;

        match event {
            MarketEvent::Book(book) => {
                println!("📖 Book Update #{}", event_count);
                println!("   Market: {}", book.market);
                println!("   Asset:  {}", book.asset_id);
                println!("   Bids:   {} levels", book.buys.len());
                println!("   Asks:   {} levels", book.sells.len());
                if !book.buys.is_empty() {
                    println!(
                        "   Best Bid: {} @ {}",
                        book.buys[0].size, book.buys[0].price
                    );
                }
                if !book.sells.is_empty() {
                    println!(
                        "   Best Ask: {} @ {}",
                        book.sells[0].size, book.sells[0].price
                    );
                }
            }
            MarketEvent::LastTradePrice(trade) => {
                println!("💰 Trade #{}", event_count);
                println!("   Market: {}", trade.market);
                println!("   Price:  {}", trade.price);
                println!("   Size:   {}", trade.size);
                println!("   Side:   {}", trade.side);
            }
            MarketEvent::PriceChange(change) => {
                println!("📊 Price Change #{}", event_count);
                println!("   Price: {}", change.price);
                println!("   Size:  {}", change.size);
                println!("   Side:  {}", change.side);
            }
            MarketEvent::TickSizeChange(tick) => {
                println!("🎚️  Tick Size Change #{}", event_count);
                println!("   Old: {}", tick.old_tick_size);
                println!("   New: {}", tick.new_tick_size);
            }
        }
        println!();

        // Stop after 10 events for demo
        if event_count >= 10 {
            println!("Received 10 events, closing connection...");
            break;
        }
    }

    client.close().await?;
    println!("Disconnected.");
    Ok(())
}
