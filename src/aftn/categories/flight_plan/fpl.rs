use serde::{Deserialize, Serialize};
use pest_derive::Parser;
use crate::aftn::categories::MessageCategory;
use crate::aftn::error::AftnError;
use crate::aftn::submessages::SubMessage;
use crate::aftn::validation;

#[derive(Parser)]
#[grammar = "aftn/categories/flight_plan/flight_plan.pest"]
pub struct FplParser;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FplMessage {
    /// Identifiant du vol (callsign)
    pub callsign: Option<String>,
    
    /// Type de vol (V, I, etc.)
    pub flight_type: Option<String>,
    
    /// Aérodrome de départ
    pub departure: Option<String>,
    
    /// Aérodrome de destination
    pub destination: Option<String>,
    
    /// Route
    pub route: Option<String>,
    
    /// Corps brut du message
    pub raw: String,
}

impl SubMessage for FplMessage {
    fn parse(body: &str) -> Result<Self, AftnError> {
        use pest::Parser;
        // Essayer de parser avec la grammaire, sinon parser manuellement
        let result = FplParser::parse(Rule::fpl, body);
        
        if let Ok(mut pairs) = result {
            let fpl_pair = pairs.next().ok_or_else(|| {
                AftnError::ParseError("Empty FPL parse result".to_string())
            })?;
            return Self::parse_fpl_pair(fpl_pair, body);
        }
        
        // Fallback: parsing manuel simple
        let parts: Vec<&str> = body.split_whitespace().collect();
        if parts.len() < 7 {
            return Err(AftnError::ParseError("FPL: not enough fields".to_string()));
        }
        
        if parts[0] != "FPL" {
            return Err(AftnError::ParseError("FPL: missing FPL prefix".to_string()));
        }
        
        Ok(FplMessage {
            callsign: Some(parts.get(1).unwrap_or(&"").to_string()),
            flight_type: Some(parts.get(2).unwrap_or(&"").to_string()),
            departure: Some(parts.get(3).unwrap_or(&"").to_string()),
            destination: parts.get(5).map(|s| s.to_string()),
            route: if parts.len() > 6 {
                Some(parts[6..].join(" "))
            } else {
                None
            },
            raw: body.to_string(),
        })
    }
    
    fn validate(&self) -> Result<(), AftnError> {
        if self.raw.trim().is_empty() {
            return Err(AftnError::InvalidFormat("FPL cannot be empty".to_string()));
        }
        
        // Valider le callsign s'il est présent
        if let Some(ref callsign) = self.callsign {
            validation::validate_callsign(callsign)?;
        }
        
        // Valider le type de vol s'il est présent
        if let Some(ref flight_type) = self.flight_type {
            validation::validate_flight_type(flight_type)?;
        }
        
        // Valider l'aérodrome de départ s'il est présent
        if let Some(ref departure) = self.departure {
            validation::validate_aerodrome_code(departure)?;
        }
        
        // Valider l'aérodrome de destination s'il est présent
        if let Some(ref destination) = self.destination {
            validation::validate_aerodrome_code(destination)?;
        }
        
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::FlightPlan
    }
}

impl FplMessage {
    fn parse_fpl_pair(pair: pest::iterators::Pair<Rule>, raw: &str) -> Result<Self, AftnError> {
        let mut callsign = None;
        let mut flight_type = None;
        let mut departure = None;
        let mut destination = None;
        let mut route = None;
        
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::callsign => {
                    callsign = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::flight_type => {
                    flight_type = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::departure => {
                    departure = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::destination => {
                    destination = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::route => {
                    route = Some(inner_pair.as_str().trim().to_string());
                }
                _ => {}
            }
        }
        
        Ok(FplMessage {
            callsign,
            flight_type,
            departure,
            destination,
            route,
            raw: raw.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_fpl() {
        let input = "FPL ABC123 V LFPG 151200 LFPB 1800";
        let result = FplMessage::parse(input);
        assert!(result.is_ok(), "FPL parsing failed: {:?}", result);
        
        let fpl = result.unwrap();
        assert_eq!(fpl.callsign, Some("ABC123".to_string()));
        assert_eq!(fpl.departure, Some("LFPG".to_string()));
        assert_eq!(fpl.destination, Some("LFPB".to_string()));
    }
    
    #[test]
    fn test_validate_fpl_callsign() {
        let mut msg = FplMessage {
            callsign: Some("ABC123".to_string()),
            flight_type: Some("V".to_string()),
            departure: Some("LFPG".to_string()),
            destination: Some("LFPB".to_string()),
            route: None,
            raw: "FPL ABC123 V LFPG 151200 LFPB 1800".to_string(),
        };
        assert!(msg.validate().is_ok());
        
        // Callsign invalide
        msg.callsign = Some("123ABC".to_string());
        assert!(msg.validate().is_err());
    }
    
    #[test]
    fn test_validate_fpl_aerodrome_codes() {
        let mut msg = FplMessage {
            callsign: Some("ABC123".to_string()),
            flight_type: Some("V".to_string()),
            departure: Some("LFPG".to_string()),
            destination: Some("LFPB".to_string()),
            route: None,
            raw: "FPL ABC123 V LFPG 151200 LFPB 1800".to_string(),
        };
        assert!(msg.validate().is_ok());
        
        // Aérodrome invalide
        msg.departure = Some("LF".to_string());
        assert!(msg.validate().is_err());
    }
    
    #[test]
    fn test_validate_fpl_flight_type() {
        let mut msg = FplMessage {
            callsign: Some("ABC123".to_string()),
            flight_type: Some("V".to_string()),
            departure: Some("LFPG".to_string()),
            destination: Some("LFPB".to_string()),
            route: None,
            raw: "FPL ABC123 V LFPG 151200 LFPB 1800".to_string(),
        };
        assert!(msg.validate().is_ok());
        
        // Type de vol invalide
        msg.flight_type = Some("X".to_string());
        assert!(msg.validate().is_err());
    }
}

