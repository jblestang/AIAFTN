use serde::{Deserialize, Serialize};
use crate::categories::MessageCategory;
use crate::error::AftnError;
use crate::submessages::SubMessage;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PosMessage {
    /// Contenu du rapport de position
    pub content: String,
    
    /// Corps brut du message
    pub raw: String,
}

impl SubMessage for PosMessage {
    fn parse(body: &str) -> Result<Self, AftnError> {
        Ok(PosMessage {
            content: body.to_string(),
            raw: body.to_string(),
        })
    }
    
    fn validate(&self) -> Result<(), AftnError> {
        if self.raw.trim().is_empty() {
            return Err(AftnError::InvalidFormat("Position report cannot be empty".to_string()));
        }
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::PositionReport
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pos() {
        let input = "POS ABC123 151230 N48.5 E2.5 FL350";
        let result = PosMessage::parse(input);
        assert!(result.is_ok());
    }
}

