use serde::{Deserialize, Serialize};
use crate::categories::MessageCategory;
use crate::error::AftnError;
use crate::submessages::SubMessage;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OperationalMessage {
    /// Type opérationnel
    pub op_type: String,
    
    /// Contenu du message
    pub content: String,
    
    /// Corps brut du message
    pub raw: String,
}

impl SubMessage for OperationalMessage {
    fn parse(body: &str) -> Result<Self, AftnError> {
        let op_type = if body.len() >= 3 {
            body[..3].to_uppercase()
        } else {
            "UNK".to_string()
        };
        
        Ok(OperationalMessage {
            op_type,
            content: body.to_string(),
            raw: body.to_string(),
        })
    }
    
    fn validate(&self) -> Result<(), AftnError> {
        if self.raw.trim().is_empty() {
            return Err(AftnError::InvalidFormat("Operational message cannot be empty".to_string()));
        }
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::Operational(self.op_type.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_operational() {
        let input = "DEP ABC123 LFPG 151230";
        let result = OperationalMessage::parse(input);
        assert!(result.is_ok());
    }
}

