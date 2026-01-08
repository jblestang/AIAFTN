//! Parser pour les messages ALR (Alerting)

use serde::{Deserialize, Serialize};
use pest_derive::Parser;
use crate::aftn::categories::MessageCategory;
use crate::aftn::error::AftnError;
use crate::aftn::submessages::SubMessage;
use crate::aftn::validation;

#[derive(Parser)]
#[grammar = "aftn/categories/alerting/alerting.pest"]
struct AlrParser;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlrMessage {
    /// Identifiant du vol (callsign)
    pub callsign: Option<String>,
    
    /// Type d'alerte
    pub alert_type: Option<String>,
    
    /// Informations d'alerte
    pub alert_info: Option<String>,
    
    /// Corps brut du message
    pub raw: String,
}

impl SubMessage for AlrMessage {
    fn parse(body: &str) -> Result<Self, AftnError> {
        use pest::Parser;
        
        let result = AlrParser::parse(Rule::alr, body);
        
        if let Ok(mut pairs) = result {
            let alr_pair = pairs.next().ok_or_else(|| {
                AftnError::ParseError("Empty ALR parse result".to_string())
            })?;
            return Self::parse_alr_pair(alr_pair, body);
        }
        
        // Fallback: parsing manuel simple
        let parts: Vec<&str> = body.split_whitespace().collect();
        
        let callsign = parts.get(0).map(|s| s.to_string());
        let alert_type = parts.get(1).map(|s| s.to_string());
        let alert_info = if parts.len() > 2 {
            Some(parts[2..].join(" "))
        } else {
            None
        };
        
        Ok(AlrMessage {
            callsign,
            alert_type,
            alert_info,
            raw: body.to_string(),
        })
    }
    
    fn validate(&self) -> Result<(), AftnError> {
        if self.raw.trim().is_empty() {
            return Err(AftnError::InvalidFormat("ALR message cannot be empty".to_string()));
        }
        
        // Valider le callsign s'il est présent
        if let Some(ref callsign) = self.callsign {
            validation::validate_callsign(callsign)?;
        }
        
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::Alerting
    }
}

impl AlrMessage {
    fn parse_alr_pair(pair: pest::iterators::Pair<Rule>, raw: &str) -> Result<Self, AftnError> {
        let mut callsign = None;
        let mut alert_type = None;
        let mut alert_info = None;
        
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::callsign => {
                    callsign = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::alert_type => {
                    alert_type = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::alert_info => {
                    alert_info = Some(inner_pair.as_str().trim().to_string());
                }
                _ => {}
            }
        }
        
        Ok(AlrMessage {
            callsign,
            alert_type,
            alert_info,
            raw: raw.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_alr() {
        let input = "ALR ABC123 EMERGENCY INFO";
        let result = AlrMessage::parse(input);
        assert!(result.is_ok());
    }
}

