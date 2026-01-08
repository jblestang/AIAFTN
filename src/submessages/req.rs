//! Parser pour les messages REQ (Request)

use serde::{Deserialize, Serialize};
use crate::categories::MessageCategory;
use crate::error::AftnError;
use crate::submessages::SubMessage;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReqMessage {
    /// Type de demande
    pub request_type: Option<String>,
    
    /// Contenu de la demande
    pub request_content: Option<String>,
    
    /// Corps brut du message
    pub raw: String,
}

impl SubMessage for ReqMessage {
    fn parse(body: &str) -> Result<Self, AftnError> {
        let parts: Vec<&str> = body.split_whitespace().collect();
        
        let request_type = parts.get(0).map(|s| s.to_string());
        let request_content = if parts.len() > 1 {
            Some(parts[1..].join(" "))
        } else {
            None
        };
        
        Ok(ReqMessage {
            request_type,
            request_content,
            raw: body.to_string(),
        })
    }
    
    fn validate(&self) -> Result<(), AftnError> {
        if self.raw.trim().is_empty() {
            return Err(AftnError::InvalidFormat("REQ message cannot be empty".to_string()));
        }
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::Request
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_req() {
        let input = "REQ FPL ABC123";
        let result = ReqMessage::parse(input);
        assert!(result.is_ok());
    }
}

