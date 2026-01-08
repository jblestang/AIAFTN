//! Parser pour les messages ALR (Alerting)

use serde::{Deserialize, Serialize};
use crate::categories::MessageCategory;
use crate::error::AftnError;
use crate::submessages::SubMessage;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlrMessage {
    /// Identifiant du vol (callsign)
    pub callsign: Option<String>,
    
    /// Type d'alerte
    pub alert_type: Option<String>,
    
    /// Informations d'alerte
    pub alert_info: Option<String>,
    
    /// Corps brut du message
    pub raw: String,
}

impl SubMessage for AlrMessage {
    fn parse(body: &str) -> Result<Self, AftnError> {
        let parts: Vec<&str> = body.split_whitespace().collect();
        
        let callsign = parts.get(0).map(|s| s.to_string());
        let alert_type = parts.get(1).map(|s| s.to_string());
        let alert_info = if parts.len() > 2 {
            Some(parts[2..].join(" "))
        } else {
            None
        };
        
        Ok(AlrMessage {
            callsign,
            alert_type,
            alert_info,
            raw: body.to_string(),
        })
    }
    
    fn validate(&self) -> Result<(), AftnError> {
        if self.raw.trim().is_empty() {
            return Err(AftnError::InvalidFormat("ALR message cannot be empty".to_string()));
        }
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::Alerting
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_alr() {
        let input = "ALR ABC123 EMERGENCY INFO";
        let result = AlrMessage::parse(input);
        assert!(result.is_ok());
    }
}

