use serde::{Deserialize, Serialize};
use pest::Parser;
use pest_derive::Parser;
use crate::aftn::categories::MessageCategory;
use crate::aftn::error::AftnError;
use crate::aftn::submessages::SubMessage;

#[derive(Parser)]
#[grammar = "aftn/categories/meteorological/meteorological.pest"]
struct NotamParser;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotamMessage {
    /// Numéro du NOTAM (format: A1234/24)
    pub number: Option<String>,
    
    /// Aérodrome concerné
    pub aerodrome: Option<String>,
    
    /// Type de NOTAM (NEW, REPLACE, CANCEL, etc.)
    pub notam_type: Option<String>,
    
    /// Contenu du NOTAM
    pub content: String,
    
    /// Corps brut du message
    pub raw: String,
}

impl SubMessage for NotamMessage {
    fn parse(body: &str) -> Result<Self, AftnError> {
        let mut pairs = NotamParser::parse(Rule::notam, body)
            .map_err(|e| AftnError::ParseError(format!("NOTAM parse error: {}", e)))?;
        
        let notam_pair = pairs.next().ok_or_else(|| {
            AftnError::ParseError("Empty NOTAM parse result".to_string())
        })?;
        
        Self::parse_notam_pair(notam_pair, body)
    }
    
    fn validate(&self) -> Result<(), AftnError> {
        // Validation basique : le contenu ne doit pas être vide
        if self.content.trim().is_empty() {
            return Err(AftnError::InvalidFormat("NOTAM content cannot be empty".to_string()));
        }
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::Notam
    }
}

impl NotamMessage {
    fn parse_notam_pair(pair: pest::iterators::Pair<Rule>, raw: &str) -> Result<Self, AftnError> {
        let mut number = None;
        let mut aerodrome = None;
        let mut notam_type = None;
        let mut content = String::new();
        
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::notam_number => {
                    number = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::aerodrome => {
                    aerodrome = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::notam_type => {
                    notam_type = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::content => {
                    content = inner_pair.as_str().trim().to_string();
                }
                _ => {}
            }
        }
        
        // Si aucun contenu spécifique n'a été trouvé, utiliser le raw
        if content.is_empty() {
            content = raw.to_string();
        }
        
        Ok(NotamMessage {
            number,
            aerodrome,
            notam_type,
            content,
            raw: raw.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_notam() {
        let input = "NOTAM A1234/24 LFPG RWY 09/27 CLOSED DUE TO MAINTENANCE";
        let result = NotamMessage::parse(input);
        assert!(result.is_ok());
        
        let notam = result.unwrap();
        // Le contenu devrait contenir le message original ou au moins une partie
        assert!(!notam.raw.is_empty());
    }

    #[test]
    fn test_parse_notam_with_number() {
        let input = "NOTAM A1234/24 LFPG";
        let result = NotamMessage::parse(input);
        assert!(result.is_ok());
    }
}

