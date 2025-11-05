//! Polymarket WebSocket Client
//!
//! This crate provides authentication and WebSocket connectivity
//! for Polymarket's CLOB (Central Limit Order Book) API.

pub mod auth;
pub mod error;
pub mod types;

pub use error::Error;
pub use types::{
    BookEvent, LastTradePriceEvent, MarketEvent, OrderSummary, PriceChangeEvent, SubscribeMessage,
    TickSizeChangeEvent,
};

// Type alias for Results in this crate
pub type Result<T> = std::result::Result<T, Error>;
