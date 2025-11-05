//! WebSocket client for Polymarket Market Channel

use crate::types::{AuthPayload, MarketEvent, SubscribeMessage};
use crate::{Error, Result};
use futures_util::{SinkExt, StreamExt};
use serde_json;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite::Message};

/// WebSocket URL for Polymarket Market Channel
const WSS_URL: &str = "wss://ws-subscriptions-clob.polymarket.com/ws/market";

/// Polymarket WebSocket client
pub struct PolymarketWss {
    /// WebSocket connection
    ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl PolymarketWss {
    /// Connect to Polymarket WebSocket and authenticate
    ///
    /// # Arguments
    /// * `api_key` - Your API key
    /// * `api_secret` - Your API secret
    /// * `passphrase` - Your API passphrase
    /// * `asset_ids` - List of token IDs to subscribe to
    ///
    /// # Example
    /// ```no_run
    /// use polymarket_wss_rs::PolymarketWss;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = PolymarketWss::connect(
    ///         "api-key".to_string(),
    ///         "api-secret".to_string(),
    ///         "passphrase".to_string(),
    ///         vec!["token-id".to_string()],
    ///     ).await.unwrap();
    /// }
    /// ```
    pub async fn connect(
        api_key: String,
        api_secret: String,
        passphrase: String,
        asset_ids: Vec<String>,
    ) -> Result<Self> {
        // Connect to WebSocket
        let (ws_stream, _) = connect_async(WSS_URL)
            .await
            .map_err(|e| Error::WebSocketError(format!("Connection failed: {}", e)))?;

        let mut client = Self { ws_stream };

        // Send subscription message
        let subscribe_msg = SubscribeMessage {
            auth: AuthPayload {
                api_key,
                secret: api_secret,
                passphrase,
            },
            markets: vec![],
            assets_ids: asset_ids,
        };

        let json = serde_json::to_string(&subscribe_msg)?;
        client
            .ws_stream
            .send(Message::Text(json))
            .await
            .map_err(|e| Error::WebSocketError(format!("Failed to subscribe: {}", e)))?;

        Ok(client)
    }

    /// Receive next market event from the WebSocket
    ///
    /// Returns `None` when the connection is closed
    pub async fn next_event(&mut self) -> Result<Option<MarketEvent>> {
        loop {
            match self.ws_stream.next().await {
                Some(Ok(Message::Text(text))) => {
                    // Try to parse as MarketEvent
                    match serde_json::from_str::<MarketEvent>(&text) {
                        Ok(event) => return Ok(Some(event)),
                        Err(e) => {
                            // Log and skip unparseable messages
                            eprintln!("Failed to parse message: {}", e);
                            eprintln!("Raw message: {}", text);
                            continue;
                        }
                    }
                }
                Some(Ok(Message::Close(_))) => return Ok(None),
                Some(Ok(_)) => continue, // Ignore ping/pong/binary
                Some(Err(e)) => return Err(Error::WebSocketError(format!("Stream error: {}", e))),
                None => return Ok(None), // Connection closed
            }
        }
    }

    /// Close the WebSocket connection
    pub async fn close(mut self) -> Result<()> {
        self.ws_stream
            .close(None)
            .await
            .map_err(|e| Error::WebSocketError(format!("Close failed: {}", e)))?;
        Ok(())
    }
}
