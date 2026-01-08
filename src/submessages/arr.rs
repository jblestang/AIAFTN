//! Parser pour les messages ARR (Arrival - arrivée)

use serde::{Deserialize, Serialize};
use crate::categories::MessageCategory;
use crate::error::AftnError;
use crate::submessages::SubMessage;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrMessage {
    /// Identifiant du vol (callsign)
    pub callsign: Option<String>,
    
    /// Aérodrome d'arrivée
    pub arrival: Option<String>,
    
    /// Heure d'arrivée
    pub arrival_time: Option<String>,
    
    /// Corps brut du message
    pub raw: String,
}

impl SubMessage for ArrMessage {
    fn parse(body: &str) -> Result<Self, AftnError> {
        let parts: Vec<&str> = body.split_whitespace().collect();
        
        let callsign = parts.get(0).map(|s| s.to_string());
        let arrival = parts.get(1).map(|s| s.to_string());
        let arrival_time = parts.get(2).map(|s| s.to_string());
        
        Ok(ArrMessage {
            callsign,
            arrival,
            arrival_time,
            raw: body.to_string(),
        })
    }
    
    fn validate(&self) -> Result<(), AftnError> {
        if self.raw.trim().is_empty() {
            return Err(AftnError::InvalidFormat("ARR message cannot be empty".to_string()));
        }
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::Arrival
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_arr() {
        let input = "ARR ABC123 LFPB 1400";
        let result = ArrMessage::parse(input);
        assert!(result.is_ok());
    }
}

