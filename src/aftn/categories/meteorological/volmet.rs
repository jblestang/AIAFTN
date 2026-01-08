use serde::{Deserialize, Serialize};
use pest::Parser;
use pest_derive::Parser;
use crate::aftn::categories::MessageCategory;
use crate::aftn::error::AftnError;
use crate::aftn::submessages::SubMessage;

#[derive(Parser)]
#[grammar = "aftn/categories/meteorological/meteorological.pest"]
struct VolmetParser;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VolmetMessage {
    /// Contenu du VOLMET
    pub content: String,
    
    /// Corps brut du message
    pub raw: String,
}

impl SubMessage for VolmetMessage {
    fn parse(body: &str) -> Result<Self, AftnError> {
        let mut pairs = VolmetParser::parse(Rule::volmet, body)
            .map_err(|e| AftnError::ParseError(format!("VOLMET parse error: {}", e)))?;
        
        let volmet_pair = pairs.next().ok_or_else(|| {
            AftnError::ParseError("Empty VOLMET parse result".to_string())
        })?;
        
        Self::parse_volmet_pair(volmet_pair, body)
    }
    
    fn validate(&self) -> Result<(), AftnError> {
        if self.raw.trim().is_empty() {
            return Err(AftnError::InvalidFormat("VOLMET cannot be empty".to_string()));
        }
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::Volmet
    }
}

impl VolmetMessage {
    fn parse_volmet_pair(pair: pest::iterators::Pair<Rule>, raw: &str) -> Result<Self, AftnError> {
        let mut content = String::new();
        
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::volmet_content => {
                    content = inner_pair.as_str().trim().to_string();
                }
                _ => {}
            }
        }
        
        if content.is_empty() {
            content = raw.to_string();
        }
        
        Ok(VolmetMessage {
            content,
            raw: raw.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_volmet() {
        let input = "VOLMET LFPG METAR";
        let result = VolmetMessage::parse(input);
        assert!(result.is_ok());
    }
}

