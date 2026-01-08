use serde::{Deserialize, Serialize};
use pest::Parser;
use pest_derive::Parser;
use crate::aftn::categories::MessageCategory;
use crate::aftn::error::AftnError;
use crate::aftn::submessages::SubMessage;

#[derive(Parser)]
#[grammar = "aftn/categories/meteorological/meteorological.pest"]
struct AirmetParser;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AirmetMessage {
    /// Contenu du AIRMET
    pub content: String,
    
    /// Corps brut du message
    pub raw: String,
}

impl SubMessage for AirmetMessage {
    fn parse(body: &str) -> Result<Self, AftnError> {
        let mut pairs = AirmetParser::parse(Rule::airmet, body)
            .map_err(|e| AftnError::ParseError(format!("AIRMET parse error: {}", e)))?;
        
        let airmet_pair = pairs.next().ok_or_else(|| {
            AftnError::ParseError("Empty AIRMET parse result".to_string())
        })?;
        
        Self::parse_airmet_pair(airmet_pair, body)
    }
    
    fn validate(&self) -> Result<(), AftnError> {
        if self.raw.trim().is_empty() {
            return Err(AftnError::InvalidFormat("AIRMET cannot be empty".to_string()));
        }
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::Airmet
    }
}

impl AirmetMessage {
    fn parse_airmet_pair(pair: pest::iterators::Pair<Rule>, raw: &str) -> Result<Self, AftnError> {
        let mut content = String::new();
        
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::airmet_content => {
                    content = inner_pair.as_str().trim().to_string();
                }
                _ => {}
            }
        }
        
        if content.is_empty() {
            content = raw.to_string();
        }
        
        Ok(AirmetMessage {
            content,
            raw: raw.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_airmet() {
        let input = "AIRMET VALID 151230/152030 LFPG";
        let result = AirmetMessage::parse(input);
        assert!(result.is_ok());
    }
}

