//! Type definitions for Polymarket WebSocket Market Channel messages
//!
//! Based on official Polymarket CLOB API documentation:
//! https://docs.polymarket.com/developers/CLOB/websocket/market-channel

use serde::{Deserialize, Serialize};

/// WebSocket message from the Market Channel
///
/// The server sends different event types tagged by the `event_type` field.
/// We use serde's untagged deserialization to handle all variants.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "event_type")]
pub enum MarketEvent {
    /// Full order book snapshot or update
    #[serde(rename = "book")]
    Book(BookEvent),

    /// Single price level change
    #[serde(rename = "price_change")]
    PriceChange(PriceChangeEvent),

    /// Tick size change (happens when price > 0.96 or < 0.04)
    #[serde(rename = "tick_size_change")]
    TickSizeChange(TickSizeChangeEvent),

    /// Last trade price (when maker/taker matched)
    #[serde(rename = "last_trade_price")]
    LastTradePrice(LastTradePriceEvent),
}

/// Full order book event (event_type: "book")
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BookEvent {
    /// Asset ID (token ID) - this is the conditional token
    pub asset_id: String,

    /// Condition ID of the market (the market identifier)
    pub market: String,

    /// Unix timestamp in milliseconds
    pub timestamp: String,

    /// Hash summary of the orderbook content
    pub hash: String,

    /// Buy side (bids) - list of (price, size) levels
    #[serde(default)]
    pub buys: Vec<OrderSummary>,

    /// Sell side (asks) - list of (price, size) levels
    #[serde(default)]
    pub sells: Vec<OrderSummary>,
}

/// Single price level in the order book
///
/// Note: The docs say "size available at that price level" for `price`
/// and "price of the orderbook level" for `size`, which seems backwards,
/// but we're matching their exact field names here.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrderSummary {
    /// Price level (e.g., "0.52")
    pub price: String,

    /// Aggregate size available at this price (e.g., "100.5")
    pub size: String,
}

/// Price change event (event_type: "price_change")
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PriceChangeEvent {
    /// Asset ID (token ID)
    pub asset_id: String,

    /// Condition ID of market
    pub market: String,

    /// Price level affected
    pub price: String,

    /// New aggregate size for this price level
    pub size: String,

    /// "BUY" or "SELL"
    pub side: String,

    /// Unix timestamp in milliseconds
    pub timestamp: String,
}

/// Tick size change event (event_type: "tick_size_change")
///
/// Happens when price reaches limits: price > 0.96 or price < 0.04
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TickSizeChangeEvent {
    /// Asset ID (token ID)
    pub asset_id: String,

    /// Condition ID of market
    pub market: String,

    /// Previous minimum tick size (e.g., "0.01")
    pub old_tick_size: String,

    /// Current minimum tick size (e.g., "0.001")
    pub new_tick_size: String,

    /// Unix timestamp in milliseconds
    pub timestamp: String,
}

/// Last trade price event (event_type: "last_trade_price")
///
/// Sent when a maker and taker order match, creating a trade
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LastTradePriceEvent {
    /// Asset ID (token ID)
    pub asset_id: String,

    /// Condition ID of market
    pub market: String,

    /// Trade price (e.g., "0.456")
    pub price: String,

    /// Trade size (e.g., "219.217767")
    pub size: String,

    /// "BUY" or "SELL"
    pub side: String,

    /// Fee rate in basis points (e.g., "0" means 0 bps)
    pub fee_rate_bps: String,

    /// Unix timestamp in milliseconds
    pub timestamp: String,
}

/// Subscription message to send to the WebSocket server
///
/// Example:
/// ```json
/// {
///   "auth": {
///     "apiKey": "your-key",
///     "secret": "your-secret",
///     "passphrase": "your-passphrase"
///   },
///   "markets": [],
///   "assets_ids": ["token-id-1", "token-id-2"]
/// }
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct SubscribeMessage {
    /// Authentication credentials
    pub auth: AuthPayload,

    /// Market IDs to subscribe to (can be empty)
    #[serde(default)]
    pub markets: Vec<String>,

    /// Asset IDs (token IDs) to subscribe to
    pub assets_ids: Vec<String>,
}

/// Authentication payload for WebSocket subscription
#[derive(Debug, Clone, Serialize)]
pub struct AuthPayload {
    /// API key
    #[serde(rename = "apiKey")]
    pub api_key: String,

    /// API secret
    pub secret: String,

    /// API passphrase
    pub passphrase: String,
}
