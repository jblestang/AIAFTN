//! Parser pour les messages CNL (Cancel - annulation de plan de vol)

use serde::{Deserialize, Serialize};
use pest_derive::Parser;
use crate::aftn::categories::MessageCategory;
use crate::aftn::error::AftnError;
use crate::aftn::submessages::SubMessage;
use crate::aftn::validation;

#[derive(Parser)]
#[grammar = "aftn/categories/flight_plan/flight_plan.pest"]
struct CnlParser;

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
        use pest::Parser;
        
        let result = CnlParser::parse(Rule::cnl, body);
        
        if let Ok(mut pairs) = result {
            let cnl_pair = pairs.next().ok_or_else(|| {
                AftnError::ParseError("Empty CNL parse result".to_string())
            })?;
            return Self::parse_cnl_pair(cnl_pair, body);
        }
        
        // Fallback: parsing manuel simple
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
        
        // Valider le callsign s'il est présent
        if let Some(ref callsign) = self.callsign {
            validation::validate_callsign(callsign)?;
        }
        
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::Cancel
    }
}

impl CnlMessage {
    fn parse_cnl_pair(pair: pest::iterators::Pair<Rule>, raw: &str) -> Result<Self, AftnError> {
        let mut callsign = None;
        let mut reason = None;
        
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::callsign => {
                    callsign = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::reason => {
                    reason = Some(inner_pair.as_str().trim().to_string());
                }
                _ => {}
            }
        }
        
        Ok(CnlMessage {
            callsign,
            reason,
            raw: raw.to_string(),
        })
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
    
    #[test]
    fn test_validate_callsign_valid() {
        let msg = CnlMessage {
            callsign: Some("ABC123".to_string()),
            reason: None,
            raw: "CNL ABC123".to_string(),
        };
        assert!(msg.validate().is_ok());
    }
    
    #[test]
    fn test_validate_callsign_empty() {
        let msg = CnlMessage {
            callsign: Some("".to_string()),
            reason: None,
            raw: "CNL ".to_string(),
        };
        assert!(msg.validate().is_err());
    }
    
    #[test]
    fn test_validate_callsign_too_long() {
        let msg = CnlMessage {
            callsign: Some("ABCDEFGH".to_string()), // 8 caractères
            reason: None,
            raw: "CNL ABCDEFGH".to_string(),
        };
        assert!(msg.validate().is_err());
    }
    
    #[test]
    fn test_validate_callsign_starts_with_digit() {
        let msg = CnlMessage {
            callsign: Some("123ABC".to_string()),
            reason: None,
            raw: "CNL 123ABC".to_string(),
        };
        assert!(msg.validate().is_err());
    }
    
    #[test]
    fn test_validate_callsign_invalid_characters() {
        let msg = CnlMessage {
            callsign: Some("AB-C123".to_string()),
            reason: None,
            raw: "CNL AB-C123".to_string(),
        };
        assert!(msg.validate().is_err());
    }
    
    #[test]
    fn test_validate_callsign_max_length() {
        let msg = CnlMessage {
            callsign: Some("ABCDEFG".to_string()), // 7 caractères (max)
            reason: None,
            raw: "CNL ABCDEFG".to_string(),
        };
        assert!(msg.validate().is_ok());
    }
    
    #[test]
    fn test_validate_callsign_single_letter() {
        let msg = CnlMessage {
            callsign: Some("A".to_string()), // 1 caractère (min)
            reason: None,
            raw: "CNL A".to_string(),
        };
        assert!(msg.validate().is_ok());
    }
}

