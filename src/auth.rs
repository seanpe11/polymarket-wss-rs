//! Authentication module for Polymarket CLOB API

use crate::{Error, Result};
use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// API credentials for L2 (HMAC-based) authentication
#[derive(Debug, Clone)]
pub struct Credentials {
    pub api_key: String,
    pub api_secret: String,
    pub passphrase: String,
}

impl Credentials {
    pub fn new(api_key: String, api_secret: String, passphrase: String) -> Self {
        Self {
            api_key,
            api_secret,
            passphrase,
        }
    }

    pub fn build_signature(
        &self,
        timestamp: u64,
        method: &str,
        path: &str,
        body: Option<&str>,
    ) -> Result<String> {
        let secret_bytes = BASE64
            .decode(&self.api_secret)
            .map_err(|e| Error::SignatureError(format!("Failed to decode secret: {}", e)))?;

        let mut message = format!("{}{}{}", timestamp, method, path);
        if let Some(body_str) = body {
            message.push_str(body_str);
        }

        let mut mac = HmacSha256::new_from_slice(&secret_bytes)
            .map_err(|e| Error::SignatureError(format!("Invalid key length: {}", e)))?;
        mac.update(message.as_bytes());
        let signature_bytes = mac.finalize().into_bytes();

        let mut signature = BASE64.encode(&signature_bytes);
        signature = signature.replace('+', "-").replace('/', "_");

        Ok(signature)
    }
}
