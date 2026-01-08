//! Parser pour les messages SPL (Supplementary Flight Plan)

use serde::{Deserialize, Serialize};
use crate::categories::MessageCategory;
use crate::error::AftnError;
use crate::submessages::SubMessage;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SplMessage {
    /// Identifiant du vol (callsign)
    pub callsign: Option<String>,
    
    /// Données supplémentaires du plan de vol
    pub supplementary_data: Option<String>,
    
    /// Corps brut du message
    pub raw: String,
}

impl SubMessage for SplMessage {
    fn parse(body: &str) -> Result<Self, AftnError> {
        let parts: Vec<&str> = body.split_whitespace().collect();
        
        let callsign = parts.get(0).map(|s| s.to_string());
        let supplementary_data = if parts.len() > 1 {
            Some(parts[1..].join(" "))
        } else {
            None
        };
        
        Ok(SplMessage {
            callsign,
            supplementary_data,
            raw: body.to_string(),
        })
    }
    
    fn validate(&self) -> Result<(), AftnError> {
        if self.raw.trim().is_empty() {
            return Err(AftnError::InvalidFormat("SPL message cannot be empty".to_string()));
        }
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::SupplementaryFlightPlan
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_spl() {
        let input = "SPL ABC123 ADDITIONAL DATA";
        let result = SplMessage::parse(input);
        assert!(result.is_ok());
    }
}

