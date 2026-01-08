use serde::{Deserialize, Serialize};
use pest::Parser;
use pest_derive::Parser;
use crate::categories::MessageCategory;
use crate::error::AftnError;
use crate::submessages::SubMessage;

#[derive(Parser)]
#[grammar = "submessages/atis.pest"]
struct AtisParser;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AtisMessage {
    /// Contenu de l'ATIS
    pub content: String,
    
    /// Corps brut du message
    pub raw: String,
}

impl SubMessage for AtisMessage {
    fn parse(body: &str) -> Result<Self, AftnError> {
        let mut pairs = AtisParser::parse(Rule::atis, body)
            .map_err(|e| AftnError::ParseError(format!("ATIS parse error: {}", e)))?;
        
        let atis_pair = pairs.next().ok_or_else(|| {
            AftnError::ParseError("Empty ATIS parse result".to_string())
        })?;
        
        Self::parse_atis_pair(atis_pair, body)
    }
    
    fn validate(&self) -> Result<(), AftnError> {
        if self.raw.trim().is_empty() {
            return Err(AftnError::InvalidFormat("ATIS cannot be empty".to_string()));
        }
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::Atis
    }
}

impl AtisMessage {
    fn parse_atis_pair(pair: pest::iterators::Pair<Rule>, raw: &str) -> Result<Self, AftnError> {
        let mut content = String::new();
        
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::atis_content => {
                    content = inner_pair.as_str().trim().to_string();
                }
                _ => {}
            }
        }
        
        if content.is_empty() {
            content = raw.to_string();
        }
        
        Ok(AtisMessage {
            content,
            raw: raw.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_atis() {
        let input = "ATIS LFPG INFORMATION ALPHA";
        let result = AtisMessage::parse(input);
        assert!(result.is_ok());
    }
}

