use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum AftnError {
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Invalid message format: {0}")]
    InvalidFormat(String),
    
    #[error("Invalid priority: {0}")]
    InvalidPriority(String),
    
    #[error("Invalid address: {0}")]
    InvalidAddress(String),
    
    #[error("Invalid category: {0}")]
    InvalidCategory(String),
    
    #[error("Invalid date/time: {0}")]
    InvalidDateTime(String),
    
    #[error("Message too long: max {max} characters, got {got}")]
    MessageTooLong { max: usize, got: usize },
    
    #[error("Message too short: min {min} characters, got {got}")]
    MessageTooShort { min: usize, got: usize },
}

