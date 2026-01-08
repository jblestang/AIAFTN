use serde::{Deserialize, Serialize};
use crate::categories::MessageCategory;
use crate::error::AftnError;
use crate::submessages::SubMessage;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VolmetMessage {
    /// Contenu du VOLMET
    pub content: String,
    
    /// Corps brut du message
    pub raw: String,
}

impl SubMessage for VolmetMessage {
    fn parse(body: &str) -> Result<Self, AftnError> {
        Ok(VolmetMessage {
            content: body.to_string(),
            raw: body.to_string(),
        })
    }
    
    fn validate(&self) -> Result<(), AftnError> {
        if self.raw.trim().is_empty() {
            return Err(AftnError::InvalidFormat("VOLMET cannot be empty".to_string()));
        }
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::Volmet
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_volmet() {
        let input = "VOLMET LFPG METAR";
        let result = VolmetMessage::parse(input);
        assert!(result.is_ok());
    }
}

