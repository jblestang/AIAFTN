//! Parser pour les messages DLA (Delay - retard)

use serde::{Deserialize, Serialize};
use pest_derive::Parser;
use crate::aftn::categories::MessageCategory;
use crate::aftn::error::AftnError;
use crate::aftn::submessages::SubMessage;
use crate::aftn::validation;

#[derive(Parser)]
#[grammar = "aftn/categories/flight_plan/flight_plan.pest"]
struct DlaParser;

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
        use pest::Parser;
        
        let result = DlaParser::parse(Rule::dla, body);
        
        if let Ok(mut pairs) = result {
            let dla_pair = pairs.next().ok_or_else(|| {
                AftnError::ParseError("Empty DLA parse result".to_string())
            })?;
            return Self::parse_dla_pair(dla_pair, body);
        }
        
        // Fallback: parsing manuel simple
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
        
        // Valider le callsign s'il est présent
        if let Some(ref callsign) = self.callsign {
            validation::validate_callsign(callsign)?;
        }
        
        // Valider la nouvelle heure de départ si elle est présente
        if let Some(ref new_departure_time) = self.new_departure_time {
            validation::validate_time_hhmm(new_departure_time)?;
        }
        
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::Delay
    }
}

impl DlaMessage {
    fn parse_dla_pair(pair: pest::iterators::Pair<Rule>, raw: &str) -> Result<Self, AftnError> {
        let mut callsign = None;
        let mut new_departure_time = None;
        let mut reason = None;
        
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::callsign => {
                    callsign = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::new_time => {
                    new_departure_time = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::reason => {
                    reason = Some(inner_pair.as_str().trim().to_string());
                }
                _ => {}
            }
        }
        
        Ok(DlaMessage {
            callsign,
            new_departure_time,
            reason,
            raw: raw.to_string(),
        })
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

