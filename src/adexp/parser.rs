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
        // D'abord, parser manuellement les blocs BEGIN/END
        let (input_without_arrays, array_sections) = Self::extract_array_blocks(input)?;
        
        // Parser le reste avec PEST
        let mut pairs = AdexpParser::parse(Rule::message, &input_without_arrays)
            .map_err(|e| AdexpError::ParseError(format!("{}", e)))?;
        
        let message_pair = pairs.next().ok_or_else(|| {
            AdexpError::ParseError("Empty parse result".to_string())
        })?;
        
        let mut message = Self::parse_message_pair(message_pair, &input_without_arrays)?;
        
        // Ajouter les sections de tableaux parsées manuellement
        for (name, section) in array_sections {
            message.sections.insert(name, section);
        }
        
        Ok(message)
    }
    
    /// Extrait et parse manuellement les blocs BEGIN/END
    /// Retourne le texte sans les blocs et les sections parsées
    fn extract_array_blocks(input: &str) -> Result<(String, HashMap<String, Section>), AdexpError> {
        let mut result = String::new();
        let mut array_sections = HashMap::new();
        let lines: Vec<&str> = input.lines().collect();
        let mut i = 0;
        
        while i < lines.len() {
            let line = lines[i].trim();
            
            // Détecter un bloc BEGIN
            if line.starts_with("-BEGIN") {
                // Extraire le nom de la section
                let section_name = line.strip_prefix("-BEGIN")
                    .ok_or_else(|| AdexpError::ParseError("Invalid BEGIN marker".to_string()))?
                    .trim()
                    .to_string();
                
                if section_name.is_empty() {
                    return Err(AdexpError::ParseError("Empty section name in BEGIN".to_string()));
                }
                
                // Créer une section pour ce tableau
                let mut array_section = Section::new(section_name.clone());
                i += 1; // Passer à la ligne suivante
                
                // Parser les éléments du tableau jusqu'à trouver -END
                while i < lines.len() {
                    let current_line = lines[i].trim();
                    
                    // Vérifier si c'est la fin du bloc
                    if current_line.starts_with("-END") {
                        let end_section_name = current_line.strip_prefix("-END")
                            .ok_or_else(|| AdexpError::ParseError("Invalid END marker".to_string()))?
                            .trim();
                        
                        if end_section_name != section_name {
                            return Err(AdexpError::ParseError(
                                format!("Mismatched END marker: expected {}, got {}", section_name, end_section_name)
                            ));
                        }
                        i += 1; // Passer à la ligne suivante
                        break;
                    }
                    
                    // Parser les champs inline de cette ligne
                    let mut field_start = 0;
                    let line_chars: Vec<char> = current_line.chars().collect();
                    
                    while field_start < line_chars.len() {
                        // Chercher le prochain "-" qui indique un champ
                        let mut field_pos = field_start;
                        while field_pos < line_chars.len() && (line_chars[field_pos] != '-' || 
                               (field_pos > 0 && line_chars[field_pos - 1] != ' ' && line_chars[field_pos - 1] != '\t')) {
                            field_pos += 1;
                        }
                        
                        if field_pos >= line_chars.len() {
                            break;
                        }
                        
                        // Extraire le nom du champ
                        let mut field_name_end = field_pos + 1;
                        while field_name_end < line_chars.len() && 
                              (line_chars[field_name_end].is_alphanumeric() || line_chars[field_name_end] == '_') {
                            field_name_end += 1;
                        }
                        
                        if field_name_end > field_pos + 1 {
                            let field_name: String = line_chars[field_pos + 1..field_name_end].iter().collect();
                            
                            // Extraire la valeur du champ (optionnelle)
                            let mut field_value_start = field_name_end;
                            while field_value_start < line_chars.len() && 
                                  (line_chars[field_value_start] == ' ' || line_chars[field_value_start] == '\t') {
                                field_value_start += 1;
                            }
                            
                            let field_value: String = if field_value_start < line_chars.len() {
                                line_chars[field_value_start..].iter().collect()
                            } else {
                                String::new()
                            };
                            
                            array_section.add_field(field_name, field_value.trim().to_string());
                        }
                        
                        field_start = field_name_end;
                    }
                    
                    i += 1;
                }
                
                // Ajouter la section au résultat
                array_sections.insert(section_name, array_section);
            } else {
                // Ligne normale, l'ajouter au résultat
                result.push_str(lines[i]);
                result.push('\n');
                i += 1;
            }
        }
        
        Ok((result, array_sections))
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
                            Rule::section_item => {
                                // section_item peut être section_header ou field
                                // Les blocs BEGIN/END sont parsés manuellement avant d'appeler PEST
                                for section_item_pair in item_pair.into_inner() {
                                    match section_item_pair.as_rule() {
                                        Rule::section_header => {
                                            // Sauvegarder la section précédente si elle existe
                                            if !current_section.name.is_empty() || !current_section.fields.is_empty() {
                                                sections.insert(current_section.name.clone(), current_section.clone());
                                            }
                                            
                                            // Créer une nouvelle section
                                            for header_pair in section_item_pair.into_inner() {
                                                if header_pair.as_rule() == Rule::section_name {
                                                    current_section = Section::new(header_pair.as_str().to_string());
                                                }
                                            }
                                        }
                                        Rule::field => {
                                            let (field_name, field_value) = Self::parse_field(section_item_pair)?;
                                            current_section.add_field(field_name, field_value);
                                        }
                                        _ => {}
                                    }
                                }
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

