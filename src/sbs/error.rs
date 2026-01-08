use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum SbsError {
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("PEST parse error: {0}")]
    PestParseError(String),
    
    #[error("Invalid message format: {0}")]
    InvalidFormat(String),
    
    #[error("Invalid message type: {0}")]
    InvalidMessageType(String),
    
    #[error("Invalid field value: {field} = {value} ({reason})")]
    InvalidFieldValue { field: String, value: String, reason: String },
    
    #[error("Missing required field: {0}")]
    MissingField(String),
    
    #[error("Invalid ICAO address: {0}")]
    InvalidIcaoAddress(String),
    
    #[error("Invalid timestamp: {0}")]
    InvalidTimestamp(String),
    
    #[error("Invalid altitude: {0}")]
    InvalidAltitude(String),
    
    #[error("Invalid speed: {0}")]
    InvalidSpeed(String),
    
    #[error("Invalid heading: {0}")]
    InvalidHeading(String),
}

