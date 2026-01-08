//! Parser pour les messages EST (Estimate - estimation)

use serde::{Deserialize, Serialize};
use crate::categories::MessageCategory;
use crate::error::AftnError;
use crate::submessages::SubMessage;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EstMessage {
    /// Identifiant du vol (callsign)
    pub callsign: Option<String>,
    
    /// Point d'estimation
    pub estimate_point: Option<String>,
    
    /// Heure estimée
    pub estimate_time: Option<String>,
    
    /// Niveau de vol estimé
    pub estimate_level: Option<String>,
    
    /// Corps brut du message
    pub raw: String,
}

impl SubMessage for EstMessage {
    fn parse(body: &str) -> Result<Self, AftnError> {
        let parts: Vec<&str> = body.split_whitespace().collect();
        
        let callsign = parts.get(0).map(|s| s.to_string());
        let estimate_point = parts.get(1).map(|s| s.to_string());
        let estimate_time = parts.get(2).map(|s| s.to_string());
        let estimate_level = parts.get(3).map(|s| s.to_string());
        
        Ok(EstMessage {
            callsign,
            estimate_point,
            estimate_time,
            estimate_level,
            raw: body.to_string(),
        })
    }
    
    fn validate(&self) -> Result<(), AftnError> {
        if self.raw.trim().is_empty() {
            return Err(AftnError::InvalidFormat("EST message cannot be empty".to_string()));
        }
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::Estimate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_est() {
        let input = "EST ABC123 LFPG 1300 F350";
        let result = EstMessage::parse(input);
        assert!(result.is_ok());
    }
}

