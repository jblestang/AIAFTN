use serde::{Deserialize, Serialize};
use pest::Parser;
use pest_derive::Parser;
use crate::categories::MessageCategory;
use crate::error::AftnError;
use crate::submessages::SubMessage;

#[derive(Parser)]
#[grammar = "submessages/fpl.pest"]
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
}

