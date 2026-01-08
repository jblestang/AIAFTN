//! Parser pour les messages NMEA 0183

use pest::Parser;
use pest_derive::Parser;
use crate::nmea::message::NmeaMessage;
use crate::nmea::types::NmeaMessageType;
use crate::nmea::error::NmeaError;

#[derive(Parser)]
#[grammar = "nmea/nmea.pest"]
pub struct NmeaParser;

impl NmeaParser {
    /// Parse un message NMEA 0183 complet
    pub fn parse_message(input: &str) -> Result<NmeaMessage, NmeaError> {
        // Nettoyer l'input (supprimer les espaces en début/fin et les retours à la ligne)
        let trimmed = input.trim();
        
        // Vérifier le format de base
        if !trimmed.starts_with('$') && !trimmed.starts_with('!') {
            return Err(NmeaError::InvalidFormat(
                "Message must start with $ or !".to_string()
            ));
        }
        
        // Trouver le checksum
        let checksum_pos = trimmed.rfind('*')
            .ok_or_else(|| NmeaError::MissingChecksum)?;
        
        let checksum_provided = trimmed[checksum_pos + 1..]
            .chars()
            .take(2)
            .collect::<String>()
            .to_uppercase();
        
        // Calculer le checksum attendu
        let checksum_expected = Self::calculate_checksum(&trimmed[1..checksum_pos]);
        
        // Valider le checksum
        if checksum_provided != checksum_expected {
            return Err(NmeaError::InvalidChecksum {
                expected: checksum_expected,
                got: checksum_provided,
            });
        }
        
        // Parser avec PEST
        let mut pairs = NmeaParser::parse(Rule::nmea_message, trimmed)
            .map_err(|e| NmeaError::ParseError(format!("{}", e)))?;
        
        let message_pair = pairs.next().ok_or_else(|| {
            NmeaError::ParseError("Empty parse result".to_string())
        })?;
        
        Self::parse_message_pair(message_pair, trimmed)
    }
    
    /// Calcule le checksum NMEA (XOR de tous les caractères entre $ et *)
    fn calculate_checksum(data: &str) -> String {
        let mut checksum: u8 = 0;
        for byte in data.bytes() {
            checksum ^= byte;
        }
        format!("{:02X}", checksum)
    }
    
    /// Parse une paire PEST en NmeaMessage
    fn parse_message_pair(pair: pest::iterators::Pair<Rule>, raw: &str) -> Result<NmeaMessage, NmeaError> {
        let mut message_type_str = String::new();
        let mut fields = Vec::new();
        let mut checksum = String::new();
        
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::message_type => {
                    message_type_str = inner_pair.as_str().to_string();
                }
                Rule::fields => {
                    // Les champs sont parsés manuellement depuis la chaîne brute
                    // car PEST ne gère pas bien les champs optionnels répétés
                }
                Rule::checksum => {
                    for checksum_pair in inner_pair.into_inner() {
                        if checksum_pair.as_rule() == Rule::hex_digit {
                            checksum.push_str(checksum_pair.as_str());
                        }
                    }
                }
                _ => {}
            }
        }
        
        // Parser les champs manuellement (plus fiable que PEST pour ce cas)
        let message_type = NmeaMessageType::from_identifier(&message_type_str)?;
        
        // Extraire les champs depuis la chaîne brute
        // Format: $MESSAGE_TYPE,field1,field2,...,fieldN*CHECKSUM
        let start_pos = raw.find(',').unwrap_or(raw.find('*').unwrap_or(raw.len()));
        let end_pos = raw.find('*').unwrap_or(raw.len());
        
        if start_pos < end_pos {
            let fields_str = &raw[start_pos + 1..end_pos];
            fields = fields_str
                .split(',')
                .map(|s| s.to_string())
                .collect();
        }
        
        Ok(NmeaMessage::new(
            message_type,
            fields,
            checksum,
            raw.to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_checksum() {
        // Exemple: $GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47
        // Le checksum est calculé sur: GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,
        let data = "GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,";
        let checksum = NmeaParser::calculate_checksum(data);
        assert_eq!(checksum, "47");
    }

    #[test]
    fn test_parse_simple_gga() {
        let input = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47";
        let result = NmeaParser::parse_message(input);
        assert!(result.is_ok(), "Should parse successfully");
        
        let message = result.unwrap();
        assert_eq!(message.message_type, NmeaMessageType::GPGGA);
        assert_eq!(message.checksum, "47");
        assert!(message.fields.len() > 0);
    }

    #[test]
    fn test_parse_invalid_checksum() {
        let input = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*00";
        let result = NmeaParser::parse_message(input);
        assert!(result.is_err(), "Should reject invalid checksum");
        
        if let Err(NmeaError::InvalidChecksum { expected, got }) = result {
            assert_eq!(expected, "47");
            assert_eq!(got, "00");
        } else {
            panic!("Expected InvalidChecksum error");
        }
    }

    #[test]
    fn test_parse_missing_checksum() {
        let input = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,";
        let result = NmeaParser::parse_message(input);
        assert!(result.is_err(), "Should reject missing checksum");
        
        assert!(matches!(result, Err(NmeaError::MissingChecksum)));
    }
}

