use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),
    #[error("Serialization error: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("Big Error: {0:?}")]
    BigError(Box<BigError>),
    #[error("Custom error:{0}")]
    Custom(String),
}

#[allow(unused)]
#[derive(Debug)]
pub struct BigError {
    a: String,
    b: Vec<String>,
    c: [u8; 8],
    d: u64,
}
