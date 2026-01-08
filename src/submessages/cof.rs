//! Parser pour les messages COF/CDN (Coordination)

use serde::{Deserialize, Serialize};
use crate::categories::MessageCategory;
use crate::error::AftnError;
use crate::submessages::SubMessage;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CofMessage {
    /// Identifiant du vol (callsign)
    pub callsign: Option<String>,
    
    /// Données de coordination
    pub coordination_data: Option<String>,
    
    /// Corps brut du message
    pub raw: String,
}

impl SubMessage for CofMessage {
    fn parse(body: &str) -> Result<Self, AftnError> {
        let parts: Vec<&str> = body.split_whitespace().collect();
        
        let callsign = parts.get(0).map(|s| s.to_string());
        let coordination_data = if parts.len() > 1 {
            Some(parts[1..].join(" "))
        } else {
            None
        };
        
        Ok(CofMessage {
            callsign,
            coordination_data,
            raw: body.to_string(),
        })
    }
    
    fn validate(&self) -> Result<(), AftnError> {
        if self.raw.trim().is_empty() {
            return Err(AftnError::InvalidFormat("COF message cannot be empty".to_string()));
        }
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::Coordination
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cof() {
        let input = "COF ABC123 COORDINATION DATA";
        let result = CofMessage::parse(input);
        assert!(result.is_ok());
    }
}

