//! Parser pour les messages DLA (Delay - retard)

use serde::{Deserialize, Serialize};
use crate::categories::MessageCategory;
use crate::error::AftnError;
use crate::submessages::SubMessage;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DlaMessage {
    /// Identifiant du vol (callsign)
    pub callsign: Option<String>,
    
    /// Nouvelle heure de départ
    pub new_departure_time: Option<String>,
    
    /// Raison du retard (optionnel)
    pub reason: Option<String>,
    
    /// Corps brut du message
    pub raw: String,
}

impl SubMessage for DlaMessage {
    fn parse(body: &str) -> Result<Self, AftnError> {
        let parts: Vec<&str> = body.split_whitespace().collect();
        
        let callsign = parts.get(0).map(|s| s.to_string());
        let new_departure_time = parts.get(1).map(|s| s.to_string());
        let reason = if parts.len() > 2 {
            Some(parts[2..].join(" "))
        } else {
            None
        };
        
        Ok(DlaMessage {
            callsign,
            new_departure_time,
            reason,
            raw: body.to_string(),
        })
    }
    
    fn validate(&self) -> Result<(), AftnError> {
        if self.raw.trim().is_empty() {
            return Err(AftnError::InvalidFormat("DLA message cannot be empty".to_string()));
        }
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::Delay
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dla() {
        let input = "DLA ABC123 1300";
        let result = DlaMessage::parse(input);
        assert!(result.is_ok());
    }
}

