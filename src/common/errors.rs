use reqwest::header::InvalidHeaderValue;
use thiserror::Error;
use tokio_tungstenite::tungstenite::{
    Error as TungsteniteError, http::Error as TungsteniteHttpError,
};

#[derive(Error, Debug)]
pub enum SignError {
    #[error("Signing operation failed: {0}")]
    Crypto(String),
    #[error("Unknown signing error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum ExchangeError {
    #[error("Custom error: {0}")]
    Custom(String),
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Header not valid: {0}")]
    InvalidHeaderValue(#[from] InvalidHeaderValue),
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Signing error: {0}")]
    Sign(#[from] SignError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Ledger signing failed: {0}")]
    LedgerSigningFailed(String),
    #[error("No signature found in CLI output")]
    NoSignature,
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Not initialized error: {0}")]
    NotInitialized(String),
    #[error("Invalid AggLevel value: {0}")]
    InvalidAggLevel(u32),
    #[error("WebSocket Error: {0}")]
    WebSocketBox(Box<dyn std::error::Error + Send + Sync>),
    #[error("WebSocket Send Message error: {0}")]
    WebSocket(Box<TungsteniteError>),
    #[error("WebSocket HTTP error: {0}")]
    WebSocketHttp(TungsteniteHttpError),
    #[error("WebSocket lost connection.")]
    WebSocketLostConnection,
    #[error("Send Request error: {0}")]
    WebSocketSendRequest(String),
    #[error("Invalid Amount Value: {0}")]
    InvalidAmountValue(String),
    #[error("Invalid Price Value: {0}")]
    InvalidPriceValue(String),
    #[error("Symbol not found in hashmap: {0}")]
    SymbolNotFound(String),
}

impl From<TungsteniteError> for ExchangeError {
    fn from(err: TungsteniteError) -> Self {
        ExchangeError::WebSocket(Box::new(err))
    }
}

impl From<TungsteniteHttpError> for ExchangeError {
    fn from(err: TungsteniteHttpError) -> Self {
        ExchangeError::WebSocketHttp(err)
    }
}

impl From<&str> for ExchangeError {
    fn from(s: &str) -> Self {
        ExchangeError::Custom(s.to_string())
    }
}

impl From<String> for ExchangeError {
    fn from(s: String) -> Self {
        ExchangeError::Custom(s)
    }
}
