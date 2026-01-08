//! Parser pour les messages DEP (Departure - départ)

use serde::{Deserialize, Serialize};
use pest_derive::Parser;
use crate::aftn::categories::MessageCategory;
use crate::aftn::error::AftnError;
use crate::aftn::submessages::SubMessage;
use crate::aftn::validation;

#[derive(Parser)]
#[grammar = "aftn/categories/flight_plan/flight_plan.pest"]
struct DepParser;

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
        use pest::Parser;
        
        let result = DepParser::parse(Rule::dep, body);
        
        if let Ok(mut pairs) = result {
            let dep_pair = pairs.next().ok_or_else(|| {
                AftnError::ParseError("Empty DEP parse result".to_string())
            })?;
            return Self::parse_dep_pair(dep_pair, body);
        }
        
        // Fallback: parsing manuel simple
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
        
        // Valider le callsign s'il est présent
        if let Some(ref callsign) = self.callsign {
            validation::validate_callsign(callsign)?;
        }
        
        // Valider l'aérodrome de départ s'il est présent
        if let Some(ref departure) = self.departure {
            validation::validate_aerodrome_code(departure)?;
        }
        
        // Valider l'heure de départ s'elle est présente
        if let Some(ref departure_time) = self.departure_time {
            validation::validate_time_hhmm(departure_time)?;
        }
        
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::Departure
    }
}

impl DepMessage {
    fn parse_dep_pair(pair: pest::iterators::Pair<Rule>, raw: &str) -> Result<Self, AftnError> {
        let mut callsign = None;
        let mut departure = None;
        let mut departure_time = None;
        
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::callsign => {
                    callsign = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::aerodrome => {
                    departure = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::time => {
                    departure_time = Some(inner_pair.as_str().trim().to_string());
                }
                _ => {}
            }
        }
        
        Ok(DepMessage {
            callsign,
            departure,
            departure_time,
            raw: raw.to_string(),
        })
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
    
    #[test]
    fn test_validate_dep() {
        let mut msg = DepMessage {
            callsign: Some("ABC123".to_string()),
            departure: Some("LFPG".to_string()),
            departure_time: Some("1200".to_string()),
            raw: "DEP ABC123 LFPG 1200".to_string(),
        };
        assert!(msg.validate().is_ok());
        
        // Callsign invalide
        msg.callsign = Some("123ABC".to_string());
        assert!(msg.validate().is_err());
        
        // Aérodrome invalide
        msg.callsign = Some("ABC123".to_string());
        msg.departure = Some("LF".to_string());
        assert!(msg.validate().is_err());
        
        // Temps invalide
        msg.departure = Some("LFPG".to_string());
        msg.departure_time = Some("2400".to_string());
        assert!(msg.validate().is_err());
    }
}

