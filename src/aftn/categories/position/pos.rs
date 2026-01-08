use serde::{Deserialize, Serialize};
use pest::Parser;
use pest_derive::Parser;
use crate::aftn::categories::MessageCategory;
use crate::aftn::error::AftnError;
use crate::aftn::submessages::SubMessage;

#[derive(Parser)]
#[grammar = "aftn/categories/position/pos.pest"]
struct PosParser;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PosMessage {
    /// Contenu du rapport de position
    pub content: String,
    
    /// Corps brut du message
    pub raw: String,
}

impl SubMessage for PosMessage {
    fn parse(body: &str) -> Result<Self, AftnError> {
        let mut pairs = PosParser::parse(Rule::pos, body)
            .map_err(|e| AftnError::ParseError(format!("POS parse error: {}", e)))?;
        
        let pos_pair = pairs.next().ok_or_else(|| {
            AftnError::ParseError("Empty POS parse result".to_string())
        })?;
        
        Self::parse_pos_pair(pos_pair, body)
    }
    
    fn validate(&self) -> Result<(), AftnError> {
        if self.raw.trim().is_empty() {
            return Err(AftnError::InvalidFormat("Position report cannot be empty".to_string()));
        }
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::PositionReport
    }
}

impl PosMessage {
    fn parse_pos_pair(pair: pest::iterators::Pair<Rule>, raw: &str) -> Result<Self, AftnError> {
        let mut content = String::new();
        
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::pos_content => {
                    content = inner_pair.as_str().trim().to_string();
                }
                _ => {}
            }
        }
        
        if content.is_empty() {
            content = raw.to_string();
        }
        
        Ok(PosMessage {
            content,
            raw: raw.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pos() {
        let input = "POS ABC123 151230 N48.5 E2.5 FL350";
        let result = PosMessage::parse(input);
        assert!(result.is_ok());
    }
}

