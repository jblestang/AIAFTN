//! Parser pour les messages SPL (Supplementary Flight Plan)

use serde::{Deserialize, Serialize};
use pest_derive::Parser;
use crate::aftn::categories::MessageCategory;
use crate::aftn::error::AftnError;
use crate::aftn::submessages::SubMessage;
use crate::aftn::validation;

#[derive(Parser)]
#[grammar = "aftn/categories/flight_plan/flight_plan.pest"]
struct SplParser;

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
        use pest::Parser;
        
        let result = SplParser::parse(Rule::spl, body);
        
        if let Ok(mut pairs) = result {
            let spl_pair = pairs.next().ok_or_else(|| {
                AftnError::ParseError("Empty SPL parse result".to_string())
            })?;
            return Self::parse_spl_pair(spl_pair, body);
        }
        
        // Fallback: parsing manuel simple
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
        
        // Valider le callsign s'il est présent
        if let Some(ref callsign) = self.callsign {
            validation::validate_callsign(callsign)?;
        }
        
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::SupplementaryFlightPlan
    }
}

impl SplMessage {
    fn parse_spl_pair(pair: pest::iterators::Pair<Rule>, raw: &str) -> Result<Self, AftnError> {
        let mut callsign = None;
        let mut supplementary_data = None;
        
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::callsign => {
                    callsign = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::supplementary_data => {
                    supplementary_data = Some(inner_pair.as_str().trim().to_string());
                }
                _ => {}
            }
        }
        
        Ok(SplMessage {
            callsign,
            supplementary_data,
            raw: raw.to_string(),
        })
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

