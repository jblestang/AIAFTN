use serde::{Deserialize, Serialize};
use crate::categories::MessageCategory;
use crate::error::AftnError;
use crate::submessages::SubMessage;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SigmetMessage {
    /// Contenu du SIGMET
    pub content: String,
    
    /// Corps brut du message
    pub raw: String,
}

impl SubMessage for SigmetMessage {
    fn parse(body: &str) -> Result<Self, AftnError> {
        Ok(SigmetMessage {
            content: body.to_string(),
            raw: body.to_string(),
        })
    }
    
    fn validate(&self) -> Result<(), AftnError> {
        if self.raw.trim().is_empty() {
            return Err(AftnError::InvalidFormat("SIGMET cannot be empty".to_string()));
        }
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::Sigmet
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sigmet() {
        let input = "SIGMET VALID 151230/152030 LFPG";
        let result = SigmetMessage::parse(input);
        assert!(result.is_ok());
    }
}

