use serde::{Deserialize, Serialize};
use pest::Parser;
use pest_derive::Parser;
use crate::aftn::categories::MessageCategory;
use crate::aftn::error::AftnError;
use crate::aftn::submessages::SubMessage;

#[derive(Parser)]
#[grammar = "aftn/categories/meteorological/meteorological.pest"]
struct TafParser;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TafMessage {
    /// Identifiant de l'aérodrome
    pub station: Option<String>,
    
    /// Date et heure d'émission
    pub issue_time: Option<String>,
    
    /// Période de validité
    pub valid_period: Option<String>,
    
    /// Conditions météorologiques
    pub forecast: String,
    
    /// Corps brut du message
    pub raw: String,
}

impl SubMessage for TafMessage {
    fn parse(body: &str) -> Result<Self, AftnError> {
        let mut pairs = TafParser::parse(Rule::taf, body)
            .map_err(|e| AftnError::ParseError(format!("TAF parse error: {}", e)))?;
        
        let taf_pair = pairs.next().ok_or_else(|| {
            AftnError::ParseError("Empty TAF parse result".to_string())
        })?;
        
        Self::parse_taf_pair(taf_pair, body)
    }
    
    fn validate(&self) -> Result<(), AftnError> {
        if self.raw.trim().is_empty() {
            return Err(AftnError::InvalidFormat("TAF cannot be empty".to_string()));
        }
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::Taf
    }
}

impl TafMessage {
    fn parse_taf_pair(pair: pest::iterators::Pair<Rule>, raw: &str) -> Result<Self, AftnError> {
        let mut station = None;
        let mut issue_time = None;
        let mut valid_period = None;
        let mut forecast = String::new();
        
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::station => {
                    station = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::issue_time => {
                    issue_time = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::valid_period => {
                    valid_period = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::forecast_element => {
                    if forecast.is_empty() {
                        forecast = inner_pair.as_str().trim().to_string();
                    } else {
                        forecast.push_str(" ");
                        forecast.push_str(inner_pair.as_str().trim());
                    }
                }
                _ => {}
            }
        }
        
        if forecast.is_empty() {
            forecast = raw.to_string();
        }
        
        Ok(TafMessage {
            station,
            issue_time,
            valid_period,
            forecast,
            raw: raw.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_taf() {
        let input = "TAF LFPG 151200Z 1512/1612 28015KT 9999 FEW030";
        let result = TafMessage::parse(input);
        assert!(result.is_ok());
        
        let taf = result.unwrap();
        assert_eq!(taf.station, Some("LFPG".to_string()));
    }
}

