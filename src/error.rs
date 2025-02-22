use thiserror::Error;

#[derive(Error, Debug)]
pub enum ZebroError {
    #[error("Failed to connect to the printer at {0}")]
    ConnectionError(String),
    #[error("Failed to send ZPL code: {0}")]
    SendError(#[from] std::io::Error),
    #[error("ZPL code is required")]
    MissingZplCode,
    #[error("Either address or IP must be provided")]
    MissingAddress,
    #[error("Invalid address format")]
    InvalidAddress,
    #[error("Invalid ZPL code")]
    InvalidZplCode
}