use serde::{Deserialize, Serialize};
use pest::Parser;
use pest_derive::Parser;
use crate::aftn::categories::MessageCategory;
use crate::aftn::error::AftnError;
use crate::aftn::submessages::SubMessage;

#[derive(Parser)]
#[grammar = "aftn/categories/meteorological/meteorological.pest"]
struct SigmetParser;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SigmetMessage {
    /// Contenu du SIGMET
    pub content: String,
    
    /// Corps brut du message
    pub raw: String,
}

impl SubMessage for SigmetMessage {
    fn parse(body: &str) -> Result<Self, AftnError> {
        let mut pairs = SigmetParser::parse(Rule::sigmet, body)
            .map_err(|e| AftnError::ParseError(format!("SIGMET parse error: {}", e)))?;
        
        let sigmet_pair = pairs.next().ok_or_else(|| {
            AftnError::ParseError("Empty SIGMET parse result".to_string())
        })?;
        
        Self::parse_sigmet_pair(sigmet_pair, body)
    }
    
    fn validate(&self) -> Result<(), AftnError> {
        if self.raw.trim().is_empty() {
            return Err(AftnError::InvalidFormat("SIGMET cannot be empty".to_string()));
        }
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::Sigmet
    }
}

impl SigmetMessage {
    fn parse_sigmet_pair(pair: pest::iterators::Pair<Rule>, raw: &str) -> Result<Self, AftnError> {
        let mut content = String::new();
        
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::sigmet_content => {
                    content = inner_pair.as_str().trim().to_string();
                }
                _ => {}
            }
        }
        
        if content.is_empty() {
            content = raw.to_string();
        }
        
        Ok(SigmetMessage {
            content,
            raw: raw.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sigmet() {
        let input = "SIGMET VALID 151230/152030 LFPG";
        let result = SigmetMessage::parse(input);
        assert!(result.is_ok());
    }
}

