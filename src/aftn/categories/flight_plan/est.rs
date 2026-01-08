//! Parser pour les messages EST (Estimate - estimation)

use serde::{Deserialize, Serialize};
use pest_derive::Parser;
use crate::aftn::categories::MessageCategory;
use crate::aftn::error::AftnError;
use crate::aftn::submessages::SubMessage;
use crate::aftn::validation;

#[derive(Parser)]
#[grammar = "aftn/categories/flight_plan/flight_plan.pest"]
struct EstParser;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EstMessage {
    /// Identifiant du vol (callsign)
    pub callsign: Option<String>,
    
    /// Point d'estimation
    pub estimate_point: Option<String>,
    
    /// Heure estimée
    pub estimate_time: Option<String>,
    
    /// Niveau de vol estimé
    pub estimate_level: Option<String>,
    
    /// Corps brut du message
    pub raw: String,
}

impl SubMessage for EstMessage {
    fn parse(body: &str) -> Result<Self, AftnError> {
        use pest::Parser;
        
        let result = EstParser::parse(Rule::est, body);
        
        if let Ok(mut pairs) = result {
            let est_pair = pairs.next().ok_or_else(|| {
                AftnError::ParseError("Empty EST parse result".to_string())
            })?;
            return Self::parse_est_pair(est_pair, body);
        }
        
        // Fallback: parsing manuel simple
        let parts: Vec<&str> = body.split_whitespace().collect();
        
        let callsign = parts.get(0).map(|s| s.to_string());
        let estimate_point = parts.get(1).map(|s| s.to_string());
        let estimate_time = parts.get(2).map(|s| s.to_string());
        let estimate_level = parts.get(3).map(|s| s.to_string());
        
        Ok(EstMessage {
            callsign,
            estimate_point,
            estimate_time,
            estimate_level,
            raw: body.to_string(),
        })
    }
    
    fn validate(&self) -> Result<(), AftnError> {
        if self.raw.trim().is_empty() {
            return Err(AftnError::InvalidFormat("EST message cannot be empty".to_string()));
        }
        
        // Valider le callsign s'il est présent
        if let Some(ref callsign) = self.callsign {
            validation::validate_callsign(callsign)?;
        }
        
        // Valider le point d'estimation s'il est présent
        if let Some(ref estimate_point) = self.estimate_point {
            validation::validate_waypoint(estimate_point)?;
        }
        
        // Valider l'heure estimée si elle est présente
        if let Some(ref estimate_time) = self.estimate_time {
            validation::validate_time_hhmm(estimate_time)?;
        }
        
        // Valider le niveau de vol estimé s'il est présent
        if let Some(ref estimate_level) = self.estimate_level {
            validation::validate_flight_level(estimate_level)?;
        }
        
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::Estimate
    }
}

impl EstMessage {
    fn parse_est_pair(pair: pest::iterators::Pair<Rule>, raw: &str) -> Result<Self, AftnError> {
        let mut callsign = None;
        let mut estimate_point = None;
        let mut estimate_time = None;
        let mut estimate_level = None;
        
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::callsign => {
                    callsign = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::waypoint => {
                    estimate_point = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::time => {
                    estimate_time = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::flight_level => {
                    estimate_level = Some(inner_pair.as_str().trim().to_string());
                }
                _ => {}
            }
        }
        
        Ok(EstMessage {
            callsign,
            estimate_point,
            estimate_time,
            estimate_level,
            raw: raw.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_est() {
        let input = "EST ABC123 LFPG 1300 F350";
        let result = EstMessage::parse(input);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_validate_est() {
        let mut msg = EstMessage {
            callsign: Some("ABC123".to_string()),
            estimate_point: Some("LFPG".to_string()),
            estimate_time: Some("1300".to_string()),
            estimate_level: Some("F350".to_string()),
            raw: "EST ABC123 LFPG 1300 F350".to_string(),
        };
        assert!(msg.validate().is_ok());
        
        // Callsign invalide
        msg.callsign = Some("123ABC".to_string());
        assert!(msg.validate().is_err());
        
        // Point d'estimation invalide
        msg.callsign = Some("ABC123".to_string());
        msg.estimate_point = Some("A".to_string());
        assert!(msg.validate().is_err());
        
        // Temps invalide
        msg.estimate_point = Some("LFPG".to_string());
        msg.estimate_time = Some("2400".to_string());
        assert!(msg.validate().is_err());
        
        // Niveau de vol invalide
        msg.estimate_time = Some("1300".to_string());
        msg.estimate_level = Some("F35".to_string());
        assert!(msg.validate().is_err());
    }
}

