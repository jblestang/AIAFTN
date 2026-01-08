//! Parser pour les messages CHG (Change - modification de plan de vol)

use serde::{Deserialize, Serialize};
use crate::categories::MessageCategory;
use crate::error::AftnError;
use crate::submessages::SubMessage;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChgMessage {
    /// Identifiant du vol (callsign)
    pub callsign: Option<String>,
    
    /// Champ modifié
    pub changed_field: Option<String>,
    
    /// Nouvelle valeur
    pub new_value: Option<String>,
    
    /// Corps brut du message
    pub raw: String,
}

impl SubMessage for ChgMessage {
    fn parse(body: &str) -> Result<Self, AftnError> {
        let parts: Vec<&str> = body.split_whitespace().collect();
        
        let callsign = parts.get(0).map(|s| s.to_string());
        let changed_field = parts.get(1).map(|s| s.to_string());
        let new_value = parts.get(2..).map(|v| v.join(" "));
        
        Ok(ChgMessage {
            callsign,
            changed_field,
            new_value,
            raw: body.to_string(),
        })
    }
    
    fn validate(&self) -> Result<(), AftnError> {
        if self.raw.trim().is_empty() {
            return Err(AftnError::InvalidFormat("CHG message cannot be empty".to_string()));
        }
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::Change
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_chg() {
        let input = "CHG ABC123 ADEP LFPB";
        let result = ChgMessage::parse(input);
        assert!(result.is_ok());
    }
}

