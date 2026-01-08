//! Parser pour les messages CNL (Cancel - annulation de plan de vol)

use serde::{Deserialize, Serialize};
use crate::categories::MessageCategory;
use crate::error::AftnError;
use crate::submessages::SubMessage;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CnlMessage {
    /// Identifiant du vol (callsign)
    pub callsign: Option<String>,
    
    /// Raison de l'annulation (optionnel)
    pub reason: Option<String>,
    
    /// Corps brut du message
    pub raw: String,
}

impl SubMessage for CnlMessage {
    fn parse(body: &str) -> Result<Self, AftnError> {
        let parts: Vec<&str> = body.split_whitespace().collect();
        
        let callsign = parts.get(0).map(|s| s.to_string());
        let reason = if parts.len() > 1 {
            Some(parts[1..].join(" "))
        } else {
            None
        };
        
        Ok(CnlMessage {
            callsign,
            reason,
            raw: body.to_string(),
        })
    }
    
    fn validate(&self) -> Result<(), AftnError> {
        if self.raw.trim().is_empty() {
            return Err(AftnError::InvalidFormat("CNL message cannot be empty".to_string()));
        }
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::Cancel
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cnl() {
        let input = "CNL ABC123";
        let result = CnlMessage::parse(input);
        assert!(result.is_ok());
    }
}

