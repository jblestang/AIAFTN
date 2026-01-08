use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum AdexpError {
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Invalid message format: {0}")]
    InvalidFormat(String),
    
    #[error("Invalid section: {0}")]
    InvalidSection(String),
    
    #[error("Invalid field: {0}")]
    InvalidField(String),
    
    #[error("Missing required field: {0}")]
    MissingField(String),
    
    #[error("Invalid field value: {0}")]
    InvalidFieldValue(String),
    
    #[error("Invalid message type: {0}")]
    InvalidMessageType(String),
    
    #[error("Section not found: {0}")]
    SectionNotFound(String),
    
    #[error("Field not found in section: {section}.{field}")]
    FieldNotFound { section: String, field: String },
    
    #[error("Invalid date/time format: {0}")]
    InvalidDateTime(String),
    
    #[error("Message too long: max {max} characters, got {got}")]
    MessageTooLong { max: usize, got: usize },
}

