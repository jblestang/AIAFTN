//! Parser pour les messages REQ (Request)

use serde::{Deserialize, Serialize};
use pest_derive::Parser;
use crate::aftn::categories::MessageCategory;
use crate::aftn::error::AftnError;
use crate::aftn::submessages::SubMessage;

#[derive(Parser)]
#[grammar = "aftn/categories/coordination/coordination.pest"]
struct ReqParser;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReqMessage {
    /// Type de demande
    pub request_type: Option<String>,
    
    /// Contenu de la demande
    pub request_content: Option<String>,
    
    /// Corps brut du message
    pub raw: String,
}

impl SubMessage for ReqMessage {
    fn parse(body: &str) -> Result<Self, AftnError> {
        use pest::Parser;
        
        let result = ReqParser::parse(Rule::req, body);
        
        if let Ok(mut pairs) = result {
            let req_pair = pairs.next().ok_or_else(|| {
                AftnError::ParseError("Empty REQ parse result".to_string())
            })?;
            return Self::parse_req_pair(req_pair, body);
        }
        
        // Fallback: parsing manuel simple
        let parts: Vec<&str> = body.split_whitespace().collect();
        
        let request_type = parts.get(0).map(|s| s.to_string());
        let request_content = if parts.len() > 1 {
            Some(parts[1..].join(" "))
        } else {
            None
        };
        
        Ok(ReqMessage {
            request_type,
            request_content,
            raw: body.to_string(),
        })
    }
    
    fn validate(&self) -> Result<(), AftnError> {
        if self.raw.trim().is_empty() {
            return Err(AftnError::InvalidFormat("REQ message cannot be empty".to_string()));
        }
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::Request
    }
}

impl ReqMessage {
    fn parse_req_pair(pair: pest::iterators::Pair<Rule>, raw: &str) -> Result<Self, AftnError> {
        let mut request_type = None;
        let mut request_content = None;
        
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::request_type => {
                    request_type = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::request_content => {
                    request_content = Some(inner_pair.as_str().trim().to_string());
                }
                _ => {}
            }
        }
        
        Ok(ReqMessage {
            request_type,
            request_content,
            raw: raw.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_req() {
        let input = "REQ FPL ABC123";
        let result = ReqMessage::parse(input);
        assert!(result.is_ok());
    }
}

