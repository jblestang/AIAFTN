//! Parser pour les messages DEP (Departure - départ)

use serde::{Deserialize, Serialize};
use crate::categories::MessageCategory;
use crate::error::AftnError;
use crate::submessages::SubMessage;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DepMessage {
    /// Identifiant du vol (callsign)
    pub callsign: Option<String>,
    
    /// Aérodrome de départ
    pub departure: Option<String>,
    
    /// Heure de départ
    pub departure_time: Option<String>,
    
    /// Corps brut du message
    pub raw: String,
}

impl SubMessage for DepMessage {
    fn parse(body: &str) -> Result<Self, AftnError> {
        let parts: Vec<&str> = body.split_whitespace().collect();
        
        let callsign = parts.get(0).map(|s| s.to_string());
        let departure = parts.get(1).map(|s| s.to_string());
        let departure_time = parts.get(2).map(|s| s.to_string());
        
        Ok(DepMessage {
            callsign,
            departure,
            departure_time,
            raw: body.to_string(),
        })
    }
    
    fn validate(&self) -> Result<(), AftnError> {
        if self.raw.trim().is_empty() {
            return Err(AftnError::InvalidFormat("DEP message cannot be empty".to_string()));
        }
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::Departure
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dep() {
        let input = "DEP ABC123 LFPG 1200";
        let result = DepMessage::parse(input);
        assert!(result.is_ok());
    }
}

