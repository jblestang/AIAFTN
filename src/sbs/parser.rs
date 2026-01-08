//! Parser pour les messages SBS (Mode-S/ADS-B)
//!
//! Format SBS typique:
//! MSG,3,111,11111,AAAAAA,111111,111111,111111,111111,111111,11111111,111111,111111,111111,1111
//! ou
//! MSG,3,145,29315,4CA2E6,27215,2015/02/05,14:53:22.734,2015/02/05,14:53:22.734,BAW1425,,,,,,,,,,,0

use pest::Parser;
use pest_derive::Parser;
use crate::sbs::message::SbsMessage;
use crate::sbs::types::SbsMessageType;
use crate::sbs::error::SbsError;

#[derive(Parser)]
#[grammar = "sbs/sbs.pest"]
pub struct SbsParser;

impl SbsParser {
    /// Parse un message SBS (Mode-S/ADS-B) complet.
    /// 
    /// Valide la structure du message (doit commencer par "MSG,type") et extrait
    /// tous les champs selon le format SBS standard. Utilise une approche hybride:
    /// PEST pour valider la structure de base, puis extraction manuelle des champs
    /// pour gérer les champs optionnels vides.
    /// 
    /// Format: MSG,type,transmission_type,session_id,aircraft_id,hex_ident,flight_id,
    ///         date_gen,time_gen,date_log,time_log,callsign,altitude,speed,track,
    ///         lat,lon,vr,squawk,alert,emergency,spi,is_on_ground
    /// 
    /// # Arguments
    /// * `input` - Message SBS brut (format CSV avec virgules comme séparateurs)
    /// 
    /// # Returns
    /// * `Ok(SbsMessage)` - Message parsé avec succès
    /// * `Err(SbsError)` - Erreur de format, type de message invalide, ou parsing échoué
    /// 
    /// # Exemples
    /// ```
    /// use aftn::SbsParser;
    /// let input = "MSG,1,145,29315,4CA2E6,27215,2015/02/05,14:53:22.734,2015/02/05,14:53:22.734,BAW1425,,,,,,,,,,,0";
    /// let message = SbsParser::parse_message(input)?;
    /// ```
    pub fn parse_message(input: &str) -> Result<SbsMessage, SbsError> {
        let trimmed = input.trim();
        
        // Parser avec PEST
        let mut pairs = SbsParser::parse(Rule::sbs_message, trimmed)
            .map_err(|e| SbsError::PestParseError(format!("{}", e)))?;
        
        let message_pair = pairs.next().ok_or_else(|| {
            SbsError::ParseError("Empty parse result".to_string())
        })?;
        
        Self::parse_sbs_pair(message_pair, trimmed)
    }
    
    /// Parse une paire PEST en SbsMessage
    /// PEST valide la structure de base (MSG,type,...), puis on extrait les champs manuellement
    fn parse_sbs_pair(pair: pest::iterators::Pair<Rule>, raw: &str) -> Result<SbsMessage, SbsError> {
        let mut message_type_num = None;
        
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::message_type => {
                    message_type_num = Some(inner_pair.as_str().parse::<u8>()
                        .map_err(|_| SbsError::InvalidMessageType(
                            format!("Invalid message type: {}", inner_pair.as_str())
                        ))?);
                }
                Rule::rest_of_message => {
                    // Les champs sont parsés manuellement depuis la chaîne brute
                    // car PEST a du mal avec les champs vides entre virgules
                }
                _ => {}
            }
        }
        
        let msg_number = message_type_num.ok_or_else(|| {
            SbsError::InvalidFormat("Missing message type".to_string())
        })?;
        
        // Extraire les champs manuellement depuis la chaîne brute
        // Format: MSG,type,field1,field2,...
        let fields: Vec<&str> = raw.split(',').collect();
        
        if fields.len() < 2 {
            return Err(SbsError::InvalidFormat(
                "Message must have at least 2 fields (MSG,type)".to_string()
            ));
        }
        
        let message_type = SbsMessageType::from_msg_number(msg_number)?;
        let mut message = SbsMessage::new(message_type, raw.to_string());
        
        // Parser les champs selon le format SBS standard
        // Format: MSG,type,transmission_type,session_id,aircraft_id,hex_ident,flight_id,date_gen,time_gen,date_log,time_log,callsign,altitude,speed,track,lat,lon,vr,squawk,alert,emergency,spi,is_on_ground
        // Index:  0   1    2               3          4           5        6         7         8         9         10         11      12      13    14    15  16  17 18     19    20        21   22
        
        if fields.len() >= 3 {
            if !fields[2].is_empty() {
                message.transmission_type = fields[2].parse::<u8>()
                    .unwrap_or(msg_number);
            }
        }
        
        if fields.len() >= 4 {
            message.session_id = if fields[3].is_empty() { None } else { Some(fields[3].to_string()) };
        }
        
        if fields.len() >= 5 {
            message.aircraft_id = if fields[4].is_empty() { None } else { Some(fields[4].to_string()) };
        }
        
        if fields.len() >= 6 {
            message.hex_ident = if fields[5].is_empty() { None } else { Some(fields[5].to_string()) };
        }
        
        if fields.len() >= 7 {
            message.flight_id = if fields[6].is_empty() { None } else { Some(fields[6].to_string()) };
        }
        
        if fields.len() >= 8 {
            message.date_message_generated = if fields[7].is_empty() { None } else { Some(fields[7].to_string()) };
        }
        
        if fields.len() >= 9 {
            message.time_message_generated = if fields[8].is_empty() { None } else { Some(fields[8].to_string()) };
        }
        
        if fields.len() >= 10 {
            message.date_message_logged = if fields[9].is_empty() { None } else { Some(fields[9].to_string()) };
        }
        
        if fields.len() >= 11 {
            message.time_message_logged = if fields[10].is_empty() { None } else { Some(fields[10].to_string()) };
        }
        
        // Détecter automatiquement où se trouve callsign
        // Normalement à l'index 11, mais peut être à l'index 10 si time_log est manquant
        // Callsign est généralement une chaîne alphanumérique (pas un nombre)
        let callsign_index = if fields.len() > 11 && !fields[11].is_empty() && !fields[11].chars().next().map_or(false, |c| c.is_ascii_digit()) {
            11 // Format standard
        } else if fields.len() > 10 && !fields[10].is_empty() && !fields[10].chars().next().map_or(false, |c| c.is_ascii_digit()) {
            10 // Format avec time_log manquant
        } else {
            11 // Par défaut
        };
        
        if fields.len() > callsign_index {
            message.callsign = if fields[callsign_index].is_empty() { None } else { Some(fields[callsign_index].to_string()) };
        }
        
        // Altitude est toujours après callsign
        let altitude_index = callsign_index + 1;
        if fields.len() > altitude_index {
            message.altitude = if fields[altitude_index].is_empty() {
                None
            } else {
                fields[altitude_index].parse::<i32>().ok()
            };
        }
        
        // Les autres champs suivent après altitude
        let speed_index = altitude_index + 1;
        if fields.len() > speed_index {
            message.ground_speed = if fields[speed_index].is_empty() {
                None
            } else {
                fields[speed_index].parse::<f64>().ok()
            };
        }
        
        let track_index = speed_index + 1;
        if fields.len() > track_index {
            message.track = if fields[track_index].is_empty() {
                None
            } else {
                fields[track_index].parse::<f64>().ok()
            };
        }
        
        let lat_index = track_index + 1;
        if fields.len() > lat_index {
            message.latitude = if fields[lat_index].is_empty() {
                None
            } else {
                fields[lat_index].parse::<f64>().ok()
            };
        }
        
        let lon_index = lat_index + 1;
        if fields.len() > lon_index {
            message.longitude = if fields[lon_index].is_empty() {
                None
            } else {
                fields[lon_index].parse::<f64>().ok()
            };
        }
        
        let vr_index = lon_index + 1;
        if fields.len() > vr_index {
            message.vertical_rate = if fields[vr_index].is_empty() {
                None
            } else {
                fields[vr_index].parse::<i32>().ok()
            };
        }
        
        let squawk_index = vr_index + 1;
        if fields.len() > squawk_index {
            message.squawk = if fields[squawk_index].is_empty() { None } else { Some(fields[squawk_index].to_string()) };
        }
        
        let alert_index = squawk_index + 1;
        if fields.len() > alert_index {
            message.alert = if fields[alert_index].is_empty() {
                None
            } else {
                Some(fields[alert_index] == "1" || fields[alert_index].to_lowercase() == "true")
            };
        }
        
        let emergency_index = alert_index + 1;
        if fields.len() > emergency_index {
            message.emergency = if fields[emergency_index].is_empty() {
                None
            } else {
                Some(fields[emergency_index] == "1" || fields[emergency_index].to_lowercase() == "true")
            };
        }
        
        let spi_index = emergency_index + 1;
        if fields.len() > spi_index {
            message.spi = if fields[spi_index].is_empty() {
                None
            } else {
                Some(fields[spi_index] == "1" || fields[spi_index].to_lowercase() == "true")
            };
        }
        
        let ground_index = spi_index + 1;
        if fields.len() > ground_index {
            message.is_on_ground = if fields[ground_index].is_empty() {
                None
            } else {
                Some(fields[ground_index] == "1" || fields[ground_index].to_lowercase() == "true")
            };
        }
        
        Ok(message)
    }
    
    /// Méthode de fallback pour le parsing manuel (si PEST échoue)
    fn parse_message_manual(input: &str) -> Result<SbsMessage, SbsError> {
        let trimmed = input.trim();
        
        // Vérifier que le message commence par MSG
        if !trimmed.starts_with("MSG,") {
            return Err(SbsError::InvalidFormat(
                "Message must start with MSG,".to_string()
            ));
        }
        
        // Parser les champs séparés par des virgules
        let fields: Vec<&str> = trimmed.split(',').collect();
        
        if fields.len() < 2 {
            return Err(SbsError::InvalidFormat(
                "Message must have at least 2 fields (MSG,type)".to_string()
            ));
        }
        
        // Field 0: "MSG"
        // PANIC: fields[0] peut panic si fields est vide, mais la grammaire PEST garantit
        // qu'un message SBS commence par "MSG", donc fields contient au moins 1 élément
        if fields[0] != "MSG" {
            return Err(SbsError::InvalidFormat(
                format!("Expected 'MSG', got '{}'", fields[0])
            ));
        }
        
        // Field 1: Message type (1-8)
        // PANIC: fields[1] peut panic si fields.len() < 2, mais la grammaire PEST garantit
        // qu'un message SBS a au moins "MSG,type", donc fields contient au moins 2 éléments
        let msg_number = fields[1].parse::<u8>()
            .map_err(|_| SbsError::InvalidMessageType(
                format!("Invalid message type: {}", fields[1])
            ))?;
        
        let message_type = SbsMessageType::from_msg_number(msg_number)?;
        let mut message = SbsMessage::new(message_type, trimmed.to_string());
        
        // Parser les champs selon le format SBS
        // Format standard: MSG,type,transmission_type,session_id,aircraft_id,hex_ident,flight_id,...
        if fields.len() >= 3 {
            // Field 2: Transmission type (optionnel)
            if !fields[2].is_empty() {
                message.transmission_type = fields[2].parse::<u8>()
                    .unwrap_or(msg_number);
            }
        }
        
        if fields.len() >= 4 {
            message.session_id = if fields[3].is_empty() { None } else { Some(fields[3].to_string()) };
        }
        
        if fields.len() >= 5 {
            message.aircraft_id = if fields[4].is_empty() { None } else { Some(fields[4].to_string()) };
        }
        
        if fields.len() >= 6 {
            message.hex_ident = if fields[5].is_empty() { None } else { Some(fields[5].to_string()) };
        }
        
        if fields.len() >= 7 {
            message.flight_id = if fields[6].is_empty() { None } else { Some(fields[6].to_string()) };
        }
        
        // Format avec dates/heures (format étendu)
        if fields.len() >= 8 {
            message.date_message_generated = if fields[7].is_empty() { None } else { Some(fields[7].to_string()) };
        }
        
        if fields.len() >= 9 {
            message.time_message_generated = if fields[8].is_empty() { None } else { Some(fields[8].to_string()) };
        }
        
        if fields.len() >= 10 {
            message.date_message_logged = if fields[9].is_empty() { None } else { Some(fields[9].to_string()) };
        }
        
        if fields.len() >= 11 {
            message.time_message_logged = if fields[10].is_empty() { None } else { Some(fields[10].to_string()) };
        }
        
        // Format SBS standard: MSG,type,trans,ses,ac,hex,flt,date_gen,time_gen,date_log,time_log,callsign,alt,speed,track,lat,lon,vr,squawk,alert,emerg,spi,ground
        // Index:               0   1    2    3   4  5   6   7         8         9         10         11      12  13    14    15  16  17 18     19    20    21   22
        // Les positions sont fixes selon le format SBS standard
        
        if fields.len() >= 12 {
            message.callsign = if fields[11].is_empty() { None } else { Some(fields[11].to_string()) };
        }
        
        if fields.len() >= 13 {
            message.altitude = if fields[12].is_empty() {
                None
            } else {
                fields[12].parse::<i32>().ok()
            };
        }
        
        // Les autres champs sont toujours aux positions fixes
        let speed_index = 13;
        if fields.len() > speed_index {
            message.ground_speed = if fields[speed_index].is_empty() {
                None
            } else {
                fields[speed_index].parse::<f64>().ok()
            };
        }
        
        let track_index = speed_index + 1;
        if fields.len() > track_index {
            message.track = if fields[track_index].is_empty() {
                None
            } else {
                fields[track_index].parse::<f64>().ok()
            };
        }
        
        let lat_index = track_index + 1;
        if fields.len() > lat_index {
            message.latitude = if fields[lat_index].is_empty() {
                None
            } else {
                fields[lat_index].parse::<f64>().ok()
            };
        }
        
        let lon_index = lat_index + 1;
        if fields.len() > lon_index {
            message.longitude = if fields[lon_index].is_empty() {
                None
            } else {
                fields[lon_index].parse::<f64>().ok()
            };
        }
        
        let vr_index = lon_index + 1;
        if fields.len() > vr_index {
            message.vertical_rate = if fields[vr_index].is_empty() {
                None
            } else {
                fields[vr_index].parse::<i32>().ok()
            };
        }
        
        let squawk_index = vr_index + 1;
        if fields.len() > squawk_index {
            message.squawk = if fields[squawk_index].is_empty() { None } else { Some(fields[squawk_index].to_string()) };
        }
        
        let alert_index = squawk_index + 1;
        if fields.len() > alert_index {
            message.alert = if fields[alert_index].is_empty() {
                None
            } else {
                Some(fields[alert_index] == "1" || fields[alert_index].to_lowercase() == "true")
            };
        }
        
        let emergency_index = alert_index + 1;
        if fields.len() > emergency_index {
            message.emergency = if fields[emergency_index].is_empty() {
                None
            } else {
                Some(fields[emergency_index] == "1" || fields[emergency_index].to_lowercase() == "true")
            };
        }
        
        let spi_index = emergency_index + 1;
        if fields.len() > spi_index {
            message.spi = if fields[spi_index].is_empty() {
                None
            } else {
                Some(fields[spi_index] == "1" || fields[spi_index].to_lowercase() == "true")
            };
        }
        
        let ground_index = spi_index + 1;
        if fields.len() > ground_index {
            message.is_on_ground = if fields[ground_index].is_empty() {
                None
            } else {
                Some(fields[ground_index] == "1" || fields[ground_index].to_lowercase() == "true")
            };
        }
        
        Ok(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_identification_message() {
        let input = "MSG,1,145,29315,4CA2E6,27215,2015/02/05,14:53:22.734,2015/02/05,14:53:22.734,BAW1425,,,,,,,,,,,0";
        let result = SbsParser::parse_message(input);
        assert!(result.is_ok(), "Should parse successfully");
        
        let message = result.unwrap();
        assert_eq!(message.message_type, SbsMessageType::Identification);
        assert_eq!(message.callsign, Some("BAW1425".to_string()));
        assert_eq!(message.hex_ident, Some("4CA2E6".to_string()));
    }

    #[test]
    fn test_parse_airborne_position_message() {
        let input = "MSG,3,145,29315,4CA2E6,27215,2015/02/05,14:53:22.734,2015/02/05,14:53:22.734,,37025,1035.0,295.6,51.4703,-0.4543,,,,,0";
        let result = SbsParser::parse_message(input);
        assert!(result.is_ok(), "Should parse successfully");
        
        let message = result.unwrap();
        assert_eq!(message.message_type, SbsMessageType::AirbornePosition);
        assert_eq!(message.altitude, Some(37025));
        assert_eq!(message.ground_speed, Some(1035.0));
        assert_eq!(message.track, Some(295.6));
        assert_eq!(message.latitude, Some(51.4703));
        assert_eq!(message.longitude, Some(-0.4543));
    }

    #[test]
    fn test_parse_airborne_velocity_message() {
        let input = "MSG,4,145,29315,4CA2E6,27215,2015/02/05,14:53:22.734,2015/02/05,14:53:22.734,,,1035.0,295.6,,,-3200,,,,,0";
        let result = SbsParser::parse_message(input);
        assert!(result.is_ok(), "Should parse successfully");
        
        let message = result.unwrap();
        assert_eq!(message.message_type, SbsMessageType::AirborneVelocity);
        assert_eq!(message.ground_speed, Some(1035.0));
        assert_eq!(message.track, Some(295.6));
        assert_eq!(message.vertical_rate, Some(-3200));
    }

    #[test]
    fn test_parse_invalid_format() {
        let input = "INVALID,1,2,3";
        let result = SbsParser::parse_message(input);
        assert!(result.is_err(), "Should reject invalid format");
    }
}

