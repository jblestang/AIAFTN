use serde::{Deserialize, Serialize};
use crate::categories::MessageCategory;
use crate::error::AftnError;
use crate::submessages::SubMessage;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenericMessage {
    /// Contenu du message générique
    pub content: String,
    
    /// Corps brut du message
    pub raw: String,
}

impl SubMessage for GenericMessage {
    fn parse(body: &str) -> Result<Self, AftnError> {
        Ok(GenericMessage {
            content: body.to_string(),
            raw: body.to_string(),
        })
    }
    
    fn validate(&self) -> Result<(), AftnError> {
        // Les messages génériques peuvent être vides
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::Generic
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_generic() {
        let input = "GEN MESSAGE CONTENT";
        let result = GenericMessage::parse(input);
        assert!(result.is_ok());
    }
}

