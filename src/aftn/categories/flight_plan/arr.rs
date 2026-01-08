//! Parser pour les messages ARR (Arrival - arrivée)

use serde::{Deserialize, Serialize};
use pest_derive::Parser;
use crate::aftn::categories::MessageCategory;
use crate::aftn::error::AftnError;
use crate::aftn::submessages::SubMessage;
use crate::aftn::validation;

#[derive(Parser)]
#[grammar = "aftn/categories/flight_plan/flight_plan.pest"]
struct ArrParser;

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
        use pest::Parser;
        
        let result = ArrParser::parse(Rule::arr, body);
        
        if let Ok(mut pairs) = result {
            let arr_pair = pairs.next().ok_or_else(|| {
                AftnError::ParseError("Empty ARR parse result".to_string())
            })?;
            return Self::parse_arr_pair(arr_pair, body);
        }
        
        // Fallback: parsing manuel simple
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
        
        // Valider le callsign s'il est présent
        if let Some(ref callsign) = self.callsign {
            validation::validate_callsign(callsign)?;
        }
        
        // Valider l'aérodrome d'arrivée s'il est présent
        if let Some(ref arrival) = self.arrival {
            validation::validate_aerodrome_code(arrival)?;
        }
        
        // Valider l'heure d'arrivée si elle est présente
        if let Some(ref arrival_time) = self.arrival_time {
            validation::validate_time_hhmm(arrival_time)?;
        }
        
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::Arrival
    }
}

impl ArrMessage {
    fn parse_arr_pair(pair: pest::iterators::Pair<Rule>, raw: &str) -> Result<Self, AftnError> {
        let mut callsign = None;
        let mut arrival = None;
        let mut arrival_time = None;
        
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::callsign => {
                    callsign = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::aerodrome => {
                    arrival = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::time => {
                    arrival_time = Some(inner_pair.as_str().trim().to_string());
                }
                _ => {}
            }
        }
        
        Ok(ArrMessage {
            callsign,
            arrival,
            arrival_time,
            raw: raw.to_string(),
        })
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
    
    #[test]
    fn test_validate_arr() {
        let mut msg = ArrMessage {
            callsign: Some("ABC123".to_string()),
            arrival: Some("LFPB".to_string()),
            arrival_time: Some("1400".to_string()),
            raw: "ARR ABC123 LFPB 1400".to_string(),
        };
        assert!(msg.validate().is_ok());
        
        // Callsign invalide
        msg.callsign = Some("123ABC".to_string());
        assert!(msg.validate().is_err());
        
        // Aérodrome invalide
        msg.callsign = Some("ABC123".to_string());
        msg.arrival = Some("LF".to_string());
        assert!(msg.validate().is_err());
        
        // Temps invalide
        msg.arrival = Some("LFPB".to_string());
        msg.arrival_time = Some("1260".to_string());
        assert!(msg.validate().is_err());
    }
}

