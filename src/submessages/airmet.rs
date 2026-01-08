use serde::{Deserialize, Serialize};
use crate::categories::MessageCategory;
use crate::error::AftnError;
use crate::submessages::SubMessage;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AirmetMessage {
    /// Contenu du AIRMET
    pub content: String,
    
    /// Corps brut du message
    pub raw: String,
}

impl SubMessage for AirmetMessage {
    fn parse(body: &str) -> Result<Self, AftnError> {
        Ok(AirmetMessage {
            content: body.to_string(),
            raw: body.to_string(),
        })
    }
    
    fn validate(&self) -> Result<(), AftnError> {
        if self.raw.trim().is_empty() {
            return Err(AftnError::InvalidFormat("AIRMET cannot be empty".to_string()));
        }
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::Airmet
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_airmet() {
        let input = "AIRMET VALID 151230/152030 LFPG";
        let result = AirmetMessage::parse(input);
        assert!(result.is_ok());
    }
}

