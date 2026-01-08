//! Parser pour les messages COF/CDN (Coordination)

use serde::{Deserialize, Serialize};
use pest_derive::Parser;
use crate::aftn::categories::MessageCategory;
use crate::aftn::error::AftnError;
use crate::aftn::submessages::SubMessage;
use crate::aftn::validation;

#[derive(Parser)]
#[grammar = "aftn/categories/coordination/coordination.pest"]
struct CofParser;

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
        use pest::Parser;
        
        let result = CofParser::parse(Rule::cof, body);
        
        if let Ok(mut pairs) = result {
            let cof_pair = pairs.next().ok_or_else(|| {
                AftnError::ParseError("Empty COF parse result".to_string())
            })?;
            return Self::parse_cof_pair(cof_pair, body);
        }
        
        // Fallback: parsing manuel simple
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
        
        // Valider le callsign s'il est présent
        if let Some(ref callsign) = self.callsign {
            validation::validate_callsign(callsign)?;
        }
        
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::Coordination
    }
}

impl CofMessage {
    fn parse_cof_pair(pair: pest::iterators::Pair<Rule>, raw: &str) -> Result<Self, AftnError> {
        let mut callsign = None;
        let mut coordination_data = None;
        
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::callsign => {
                    callsign = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::coordination_data => {
                    coordination_data = Some(inner_pair.as_str().trim().to_string());
                }
                _ => {}
            }
        }
        
        Ok(CofMessage {
            callsign,
            coordination_data,
            raw: raw.to_string(),
        })
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

