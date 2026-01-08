//! Parser pour les messages CHG (Change - modification de plan de vol)

use serde::{Deserialize, Serialize};
use pest_derive::Parser;
use crate::aftn::categories::MessageCategory;
use crate::aftn::error::AftnError;
use crate::aftn::submessages::SubMessage;
use crate::aftn::validation;

#[derive(Parser)]
#[grammar = "aftn/categories/flight_plan/flight_plan.pest"]
struct ChgParser;

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
        use pest::Parser;
        
        let result = ChgParser::parse(Rule::chg, body);
        
        if let Ok(mut pairs) = result {
            let chg_pair = pairs.next().ok_or_else(|| {
                AftnError::ParseError("Empty CHG parse result".to_string())
            })?;
            return Self::parse_chg_pair(chg_pair, body);
        }
        
        // Fallback: parsing manuel simple
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
        
        // Valider le callsign s'il est présent
        if let Some(ref callsign) = self.callsign {
            validation::validate_callsign(callsign)?;
        }
        
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::Change
    }
}

impl ChgMessage {
    fn parse_chg_pair(pair: pest::iterators::Pair<Rule>, raw: &str) -> Result<Self, AftnError> {
        let mut callsign = None;
        let mut changed_field = None;
        let mut new_value = None;
        
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::callsign => {
                    callsign = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::changed_field => {
                    changed_field = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::new_value => {
                    new_value = Some(inner_pair.as_str().trim().to_string());
                }
                _ => {}
            }
        }
        
        Ok(ChgMessage {
            callsign,
            changed_field,
            new_value,
            raw: raw.to_string(),
        })
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
    
    #[test]
    fn test_validate_chg() {
        let mut msg = ChgMessage {
            callsign: Some("ABC123".to_string()),
            changed_field: Some("ADEP".to_string()),
            new_value: Some("LFPB".to_string()),
            raw: "CHG ABC123 ADEP LFPB".to_string(),
        };
        assert!(msg.validate().is_ok());
        
        // Callsign invalide
        msg.callsign = Some("123ABC".to_string());
        assert!(msg.validate().is_err());
    }
}

