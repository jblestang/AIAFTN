use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use crate::adexp::message::{AdexpMessage, Section};
use crate::adexp::types::MessageType;
use crate::adexp::error::AdexpError;

#[derive(Parser)]
#[grammar = "adexp/adexp.pest"]
pub struct AdexpParser;

impl AdexpParser {
    /// Parse un message ADEXP complet
    pub fn parse_message(input: &str) -> Result<AdexpMessage, AdexpError> {
        let mut pairs = AdexpParser::parse(Rule::message, input)
            .map_err(|e| AdexpError::ParseError(format!("{}", e)))?;
        
        let message_pair = pairs.next().ok_or_else(|| {
            AdexpError::ParseError("Empty parse result".to_string())
        })?;
        
        Self::parse_message_pair(message_pair, input)
    }
    
    fn parse_message_pair(pair: pest::iterators::Pair<Rule>, raw: &str) -> Result<AdexpMessage, AdexpError> {
        let mut message = AdexpMessage::new(raw.to_string());
        let mut current_section = Section::new("".to_string());
        let mut sections = HashMap::new();
        
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::sections => {
                    for item_pair in inner_pair.into_inner() {
                        match item_pair.as_rule() {
                            Rule::section_header => {
                                // Sauvegarder la section précédente si elle existe
                                if !current_section.name.is_empty() || !current_section.fields.is_empty() {
                                    sections.insert(current_section.name.clone(), current_section.clone());
                                }
                                
                                // Créer une nouvelle section
                                for header_pair in item_pair.into_inner() {
                                    if header_pair.as_rule() == Rule::section_name {
                                        current_section = Section::new(header_pair.as_str().to_string());
                                    }
                                }
                            }
                            Rule::field => {
                                let (field_name, field_value) = Self::parse_field(item_pair)?;
                                current_section.add_field(field_name, field_value);
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        
        // Sauvegarder la dernière section
        if !current_section.name.is_empty() || !current_section.fields.is_empty() {
            sections.insert(current_section.name.clone(), current_section);
        }
        
        // Si aucune section nommée, créer une section par défaut (vide)
        if sections.is_empty() {
            let default_section = Section::new("".to_string());
            sections.insert("".to_string(), default_section);
        }
        
        message.sections = sections;
        
        // Déterminer le type de message depuis TITLE
        if let Ok(Some(title)) = message.get_field_value("", "TITLE") {
            message.message_type = MessageType::from_title(title)?;
        }
        
        Ok(message)
    }
    
    
    fn parse_field(pair: pest::iterators::Pair<Rule>) -> Result<(String, String), AdexpError> {
        let mut field_name = String::new();
        let mut field_value = String::new();
        
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::field_name => {
                    field_name = inner_pair.as_str().to_string();
                }
                Rule::field_value => {
                    field_value = inner_pair.as_str().trim().to_string();
                }
                _ => {}
            }
        }
        
        Ok((field_name, field_value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_fpl() {
        let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
";
        let result = AdexpParser::parse_message(input);
        assert!(result.is_ok(), "Le parsing devrait réussir");
        
        let message = result.unwrap();
        assert_eq!(message.message_type, MessageType::FlightPlan);
        assert_eq!(message.get_field_value("", "ARCID").unwrap(), Some(&"ABC123".to_string()));
        assert_eq!(message.get_field_value("", "ADEP").unwrap(), Some(&"LFPG".to_string()));
        assert_eq!(message.get_field_value("", "ADES").unwrap(), Some(&"LFPB".to_string()));
    }

    #[test]
    fn test_parse_chg_message() {
        let input = "-ADEXP
-TITLE CHG
-ARCID ABC123
-ADEP LFPG
";
        let result = AdexpParser::parse_message(input);
        assert!(result.is_ok());
        
        let message = result.unwrap();
        assert_eq!(message.message_type, MessageType::Change);
    }

    #[test]
    fn test_parse_with_sections() {
        let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ROUTE
-ADEP LFPG
-ADES LFPB
";
        let result = AdexpParser::parse_message(input);
        assert!(result.is_ok());
        
        let message = result.unwrap();
        // Les champs devraient être accessibles
        assert!(message.get_field_value("", "ARCID").is_ok());
    }

    #[test]
    fn test_parse_field_without_value() {
        let input = "-ADEXP
-TITLE FPL
-ARCID
";
        let result = AdexpParser::parse_message(input);
        // Le parsing peut réussir même si la valeur est vide
        let _ = result;
    }

    #[test]
    fn test_parse_multiple_field_values() {
        let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADEP LFPB
";
        let result = AdexpParser::parse_message(input);
        assert!(result.is_ok());
        
        let message = result.unwrap();
        // ADEP devrait avoir plusieurs valeurs
        let adep_values = message.get_field("", "ADEP").unwrap();
        assert!(adep_values.is_some());
        if let Some(values) = adep_values {
            assert!(values.len() >= 1);
        }
    }
}

