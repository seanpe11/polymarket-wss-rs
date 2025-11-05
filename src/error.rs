use std::fmt;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to build signature: {0}")]
    SignatureErorr(String),

    #[error("Base64 decode error: {0}")]
    Base64Error(#[from] base64::DecodeError),

    #[error("Websocket error: {0}")]
    WebSocketError(String),

    #[error("HTTP request error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}
