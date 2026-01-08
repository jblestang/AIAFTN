use serde::{Deserialize, Serialize};
use crate::categories::MessageCategory;
use crate::error::AftnError;
use crate::submessages::SubMessage;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AtisMessage {
    /// Contenu de l'ATIS
    pub content: String,
    
    /// Corps brut du message
    pub raw: String,
}

impl SubMessage for AtisMessage {
    fn parse(body: &str) -> Result<Self, AftnError> {
        Ok(AtisMessage {
            content: body.to_string(),
            raw: body.to_string(),
        })
    }
    
    fn validate(&self) -> Result<(), AftnError> {
        if self.raw.trim().is_empty() {
            return Err(AftnError::InvalidFormat("ATIS cannot be empty".to_string()));
        }
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::Atis
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_atis() {
        let input = "ATIS LFPG INFORMATION ALPHA";
        let result = AtisMessage::parse(input);
        assert!(result.is_ok());
    }
}

