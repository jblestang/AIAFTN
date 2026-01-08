use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum NmeaError {
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Invalid message format: {0}")]
    InvalidFormat(String),
    
    #[error("Invalid checksum: expected {expected}, got {got}")]
    InvalidChecksum { expected: String, got: String },
    
    #[error("Missing checksum")]
    MissingChecksum,
    
    #[error("Invalid message type: {0}")]
    InvalidMessageType(String),
    
    #[error("Invalid field value: {field} = {value} ({reason})")]
    InvalidFieldValue { field: String, value: String, reason: String },
    
    #[error("Missing required field: {0}")]
    MissingField(String),
    
    #[error("Invalid coordinate: {0}")]
    InvalidCoordinate(String),
    
    #[error("Invalid time format: {0}")]
    InvalidTime(String),
    
    #[error("Invalid date format: {0}")]
    InvalidDate(String),
}

