use pest::Parser;
use pest_derive::Parser;
use crate::aftn::message::{AftnMessage, Addresses, TransmissionTime};
use crate::aftn::categories::MessageCategory;
use crate::aftn::error::AftnError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_message() {
        let input = "GG LFPGYYYX LFPOYYYX 151230 NOTAM A1234/24 LFPG RWY 09/27 CLOSED";
        let result = AftnParser::parse_message(input);
        assert!(result.is_ok(), "Le parsing devrait réussir");
        
        let message = result.unwrap();
        assert_eq!(message.priority, "GG");
        assert_eq!(message.addresses.origin, "LFPGYYYX");
        assert_eq!(message.addresses.destinations.len(), 1);
        assert_eq!(message.addresses.destinations[0], "LFPOYYYX");
        assert_eq!(message.transmission_time.day, 15);
        assert_eq!(message.transmission_time.hour, 12);
        assert_eq!(message.transmission_time.minute, 30);
    }

    #[test]
    fn test_parse_message_with_multiple_destinations() {
        let input = "DD LFPGYYYX LFPOYYYX LFPBYYYX 201530 METAR LFPG 201530Z 28015KT 9999 FEW030 12/08 Q1013";
        let result = AftnParser::parse_message(input);
        assert!(result.is_ok());
        
        let message = result.unwrap();
        assert_eq!(message.priority, "DD");
        assert_eq!(message.addresses.destinations.len(), 2);
        assert_eq!(message.addresses.destinations[0], "LFPOYYYX");
        assert_eq!(message.addresses.destinations[1], "LFPBYYYX");
    }

    #[test]
    fn test_parse_message_with_sequence_number() {
        let input = "FF LFPGYYYX LFPOYYYX 151230 NOTAM A1234/24 /SEQ 001";
        let result = AftnParser::parse_message(input);
        assert!(result.is_ok());
        
        let message = result.unwrap();
        assert!(message.sequence_number.is_some());
        assert_eq!(message.sequence_number.unwrap(), "001");
    }

    #[test]
    fn test_parse_metar_message() {
        let input = "GG LFPGYYYX LFPOYYYX 151230 METAR LFPG 151230Z 28015KT 9999 FEW030 12/08 Q1013";
        let result = AftnParser::parse_message(input);
        assert!(result.is_ok());
        
        let message = result.unwrap();
        assert_eq!(message.category, MessageCategory::Metar);
    }

    #[test]
    fn test_parse_taf_message() {
        let input = "DD LFPGYYYX LFPOYYYX 151230 TAF LFPG 151200Z 1512/1612 28015KT 9999 FEW030";
        let result = AftnParser::parse_message(input);
        assert!(result.is_ok());
        
        let message = result.unwrap();
        assert_eq!(message.category, MessageCategory::Taf);
    }

    #[test]
    fn test_parse_notam_message() {
        let input = "GG LFPGYYYX LFPOYYYX 151230 NOTAM A1234/24 LFPG RWY 09/27 CLOSED";
        let result = AftnParser::parse_message(input);
        assert!(result.is_ok());
        
        let message = result.unwrap();
        assert_eq!(message.category, MessageCategory::Notam);
    }

    #[test]
    fn test_all_priorities() {
        let priorities = vec!["GG", "DD", "FF", "SS", "KK", "LL"];
        
        for priority in priorities {
            let input = format!("{} LFPGYYYX LFPOYYYX 151230 TEST MESSAGE", priority);
            let result = AftnParser::parse_message(&input);
            assert!(result.is_ok(), "Le parsing devrait réussir pour la priorité {}", priority);
            
            let message = result.unwrap();
            assert_eq!(message.priority, priority);
            assert!(message.validate().is_ok(), "La validation devrait réussir pour la priorité {}", priority);
        }
    }
}

#[derive(Parser)]
#[grammar = "aftn/aftn.pest"]
pub struct AftnParser;

impl AftnParser {
    /// Parse un message AFTN complet
    pub fn parse_message(input: &str) -> Result<AftnMessage, AftnError> {
        let mut pairs = AftnParser::parse(Rule::message, input)
            .map_err(|e| AftnError::ParseError(format!("{}", e)))?;
        
        let message_pair = pairs.next().ok_or_else(|| {
            AftnError::ParseError("Empty parse result".to_string())
        })?;
        
        Self::parse_message_pair(message_pair)
    }
    
    fn parse_message_pair(pair: pest::iterators::Pair<Rule>) -> Result<AftnMessage, AftnError> {
        let mut priority = String::new();
        let mut origin = String::new();
        let mut destinations = Vec::new();
        let mut day = 0u8;
        let mut hour = 0u8;
        let mut minute = 0u8;
        let mut body = String::new();
        let mut sequence_number = None;
        
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::priority => {
                    priority = inner_pair.as_str().to_string();
                }
                Rule::addresses => {
                    let mut addr_pairs = inner_pair.into_inner();
                    if let Some(origin_pair) = addr_pairs.next() {
                        origin = origin_pair.as_str().trim().to_string();
                    }
                    for dest_pair in addr_pairs {
                        let dest = dest_pair.as_str().trim().to_string();
                        if !dest.is_empty() {
                            destinations.push(dest);
                        }
                    }
                }
                Rule::transmission_time => {
                    let mut time_pairs = inner_pair.into_inner();
                    if let Some(day_pair) = time_pairs.next() {
                        day = day_pair.as_str().parse::<u8>()
                            .map_err(|e| AftnError::InvalidDateTime(format!("Invalid day: {}", e)))?;
                    }
                    if let Some(hour_pair) = time_pairs.next() {
                        hour = hour_pair.as_str().parse::<u8>()
                            .map_err(|e| AftnError::InvalidDateTime(format!("Invalid hour: {}", e)))?;
                    }
                    if let Some(minute_pair) = time_pairs.next() {
                        minute = minute_pair.as_str().parse::<u8>()
                            .map_err(|e| AftnError::InvalidDateTime(format!("Invalid minute: {}", e)))?;
                    }
                }
                Rule::body => {
                    body = inner_pair.as_str().trim().to_string();
                }
                Rule::sequence_number => {
                    // Extraire juste le numéro de séquence, pas le "/SEQ"
                    let seq_str = inner_pair.as_str().trim();
                    if let Some(seq) = seq_str.strip_prefix("/SEQ") {
                        sequence_number = Some(seq.trim().to_string());
                    } else {
                        sequence_number = Some(seq_str.to_string());
                    }
                }
                _ => {}
            }
        }
        
        // Détecter la catégorie depuis le corps du message
        let category = if body.len() >= 3 {
            MessageCategory::from_message_id(&body[..3])?
        } else {
            MessageCategory::Generic
        };
        
        // Parser le sous-message selon la catégorie (pour validation)
        let _submessage = crate::aftn::submessages::parse_submessage(&category, &body).ok();
        
        let message = AftnMessage {
            priority,
            addresses: Addresses {
                origin,
                destinations,
            },
            category,
            transmission_time: TransmissionTime {
                day,
                hour,
                minute,
            },
            body,
            sequence_number,
        };
        
        message.validate()?;
        Ok(message)
    }
}

