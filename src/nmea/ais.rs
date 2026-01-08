//! Structures de données et décodage pour les messages AIS NMEA 0183
//!
//! AIS (Automatic Identification System) utilise des messages NMEA 0183
//! avec un format spécial: AIVDM (pour les données reçues) et AIVDO (pour les données envoyées)
//!
//! Format: $AIVDM,1,1,,A,13HOI:0P0000VOHLCnHQKwvL05Ip,0*XX
//! Les données AIS sont encodées en 6-bit ASCII

use serde::{Deserialize, Serialize};
use crate::nmea::error::NmeaError;
use crate::nmea::message::NmeaMessage;

/// Message AIS (AIVDM ou AIVDO)
/// Format: $AIVDM,seq_num,frag_num,radio_channel,payload,fill_bits*checksum
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AisMessage {
    /// Type de message AIS (AIVDM ou AIVDO)
    pub message_type: String,
    /// Numéro de séquence (1-9)
    pub sequence_number: Option<u8>,
    /// Numéro de fragment (1-9)
    pub fragment_number: Option<u8>,
    /// Numéro de fragment total (optionnel, pour messages multi-fragments)
    pub total_fragments: Option<u8>,
    /// Canal radio (A ou B)
    pub radio_channel: Option<char>,
    /// Payload encodé en 6-bit ASCII
    pub payload: String,
    /// Nombre de bits de remplissage (0-5)
    pub fill_bits: Option<u8>,
    /// Données décodées (si disponible)
    pub decoded_data: Option<AisDecodedData>,
}

/// Données AIS décodées
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AisDecodedData {
    /// Type de message AIS (1-27)
    pub message_id: u8,
    /// MMSI (Maritime Mobile Service Identity)
    pub mmsi: Option<u32>,
    /// Autres données spécifiques au type de message
    pub data: AisMessageData,
}

/// Données spécifiques selon le type de message AIS
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AisMessageData {
    /// Message de type 1, 2, 3: Position Report Class A
    PositionReport {
        /// Statut de navigation
        navigation_status: Option<u8>,
        /// Taux de rotation (degrés/min)
        rate_of_turn: Option<i8>,
        /// Vitesse sur le fond (noeuds)
        speed_over_ground: Option<u16>,
        /// Précision de position
        position_accuracy: Option<bool>,
        /// Longitude (1/10000 minute)
        longitude: Option<i32>,
        /// Latitude (1/10000 minute)
        latitude: Option<i32>,
        /// Cap sur le fond (degrés)
        course_over_ground: Option<u16>,
        /// Cap vrai (degrés)
        true_heading: Option<u16>,
        /// Time stamp (secondes UTC)
        timestamp: Option<u8>,
    },
    /// Message de type 5: Static and Voyage Related Data
    StaticData {
        /// MMSI
        mmsi: u32,
        /// Indicatif d'appel
        call_sign: Option<String>,
        /// Nom du navire
        vessel_name: Option<String>,
        /// Type de navire
        ship_type: Option<u8>,
        /// Dimensions du navire
        dimensions: Option<AisDimensions>,
        /// Type de système de positionnement
        position_fix_type: Option<u8>,
        /// ETA (Estimated Time of Arrival)
        eta: Option<AisEta>,
        /// Tirant d'eau (mètres)
        draught: Option<u16>,
        /// Destination
        destination: Option<String>,
    },
    /// Message générique (pour types non encore implémentés)
    Generic(Vec<u8>),
}

/// Dimensions d'un navire AIS
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AisDimensions {
    /// Longueur (mètres)
    pub length: u16,
    /// Largeur (mètres)
    pub width: u16,
}

/// ETA (Estimated Time of Arrival) AIS
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AisEta {
    /// Mois (1-12)
    pub month: u8,
    /// Jour (1-31)
    pub day: u8,
    /// Heure (0-23)
    pub hour: u8,
    /// Minute (0-59)
    pub minute: u8,
}

impl AisMessage {
    /// Parse un message AIS depuis un NmeaMessage
    pub fn from_nmea(message: &NmeaMessage) -> Result<Self, NmeaError> {
        if message.fields.len() < 5 {
            return Err(NmeaError::InvalidFormat(
                "AIS message must have at least 5 fields".to_string()
            ));
        }
        
        let message_type = message.message_type.identifier().to_string();
        
        Ok(AisMessage {
            message_type: message_type.clone(),
            sequence_number: message.get_field(0)
                .and_then(|s| if s.is_empty() { None } else { s.parse::<u8>().ok() }),
            fragment_number: message.get_field(1)
                .and_then(|s| if s.is_empty() { None } else { s.parse::<u8>().ok() }),
            total_fragments: message.get_field(2)
                .and_then(|s| if s.is_empty() { None } else { s.parse::<u8>().ok() }),
            radio_channel: message.get_field(3)
                .and_then(|s| if s.is_empty() { None } else { s.chars().next() }),
            payload: message.get_field(4)
                .cloned()
                .unwrap_or_default(),
            fill_bits: message.get_field(5)
                .and_then(|s| if s.is_empty() { None } else { s.parse::<u8>().ok() }),
            decoded_data: None, // Sera décodé si nécessaire
        })
    }
    
    /// Décode le payload AIS en données binaires
    pub fn decode_payload(&self) -> Result<Vec<u8>, NmeaError> {
        decode_ais_6bit(&self.payload, self.fill_bits.unwrap_or(0))
    }
    
    /// Décode et parse les données AIS
    pub fn decode(&mut self) -> Result<(), NmeaError> {
        let binary_data = self.decode_payload()?;
        
        if binary_data.is_empty() {
            return Err(NmeaError::InvalidFormat("Empty AIS payload".to_string()));
        }
        
        let message_id = (binary_data[0] >> 2) & 0x3F;
        let mmsi = extract_mmsi(&binary_data)?;
        
        let data = match message_id {
            1..=3 => {
                // Position Report Class A
                AisMessageData::PositionReport {
                    navigation_status: extract_navigation_status(&binary_data),
                    rate_of_turn: extract_rate_of_turn(&binary_data),
                    speed_over_ground: extract_speed_over_ground(&binary_data),
                    position_accuracy: extract_position_accuracy(&binary_data),
                    longitude: extract_longitude(&binary_data),
                    latitude: extract_latitude(&binary_data),
                    course_over_ground: extract_course_over_ground(&binary_data),
                    true_heading: extract_true_heading(&binary_data),
                    timestamp: extract_timestamp(&binary_data),
                }
            }
            5 => {
                // Static and Voyage Related Data
                AisMessageData::StaticData {
                    mmsi: mmsi.ok_or_else(|| NmeaError::MissingField("MMSI".to_string()))?,
                    call_sign: extract_call_sign(&binary_data),
                    vessel_name: extract_vessel_name(&binary_data),
                    ship_type: extract_ship_type(&binary_data),
                    dimensions: extract_dimensions(&binary_data),
                    position_fix_type: extract_position_fix_type(&binary_data),
                    eta: extract_eta(&binary_data),
                    draught: extract_draught(&binary_data),
                    destination: extract_destination(&binary_data),
                }
            }
            _ => AisMessageData::Generic(binary_data),
        };
        
        self.decoded_data = Some(AisDecodedData {
            message_id,
            mmsi,
            data,
        });
        
        Ok(())
    }
}

/// Décode une chaîne AIS 6-bit en données binaires
fn decode_ais_6bit(encoded: &str, fill_bits: u8) -> Result<Vec<u8>, NmeaError> {
    // Table de conversion 6-bit ASCII vers binaire
    // Les caractères ASCII 0x30-0x77 sont convertis en valeurs 0-63
    let mut result = Vec::new();
    let mut bit_buffer: u32 = 0;
    let mut bit_count = 0;
    
    for ch in encoded.chars() {
        let value = match ch as u8 {
            b @ 0x30..=0x77 => {
                let decoded = if b < 0x40 {
                    b - 0x30
                } else {
                    b - 0x38
                };
                if decoded > 63 {
                    return Err(NmeaError::InvalidFormat(
                        format!("Invalid 6-bit ASCII character: {}", ch)
                    ));
                }
                decoded
            }
            _ => {
                return Err(NmeaError::InvalidFormat(
                    format!("Invalid character in AIS payload: {}", ch)
                ));
            }
        };
        
        bit_buffer = (bit_buffer << 6) | (value as u32);
        bit_count += 6;
        
        while bit_count >= 8 {
            result.push((bit_buffer >> (bit_count - 8)) as u8);
            bit_count -= 8;
        }
    }
    
    // Gérer les bits de remplissage
    if fill_bits > 0 && bit_count > 0 {
        bit_buffer >>= fill_bits;
        bit_count -= fill_bits;
        if bit_count >= 8 {
            result.push((bit_buffer >> (bit_count - 8)) as u8);
        }
    }
    
    Ok(result)
}

/// Extrait un entier non signé depuis les données binaires AIS
fn extract_uint(data: &[u8], start_bit: usize, length: usize) -> u32 {
    let mut result = 0u32;
    for i in 0..length {
        let bit_pos = start_bit + i;
        let byte_pos = bit_pos / 8;
        let bit_in_byte = 7 - (bit_pos % 8);
        
        if byte_pos < data.len() {
            let bit = (data[byte_pos] >> bit_in_byte) & 1;
            result = (result << 1) | (bit as u32);
        }
    }
    result
}

/// Extrait un entier signé depuis les données binaires AIS
fn extract_int(data: &[u8], start_bit: usize, length: usize) -> i32 {
    let unsigned = extract_uint(data, start_bit, length);
    let sign_bit = 1 << (length - 1);
    if (unsigned & sign_bit) != 0 {
        // Nombre négatif
        let mask = (1 << length) - 1;
        -((!unsigned & mask) as i32 + 1)
    } else {
        unsigned as i32
    }
}

/// Extrait une chaîne ASCII depuis les données binaires AIS
fn extract_string(data: &[u8], start_bit: usize, length: usize) -> String {
    let mut result = String::new();
    for i in 0..length {
        let bit_pos = start_bit + (i * 6);
        let value = extract_uint(data, bit_pos, 6) as u8;
        if value == 0 {
            break; // Fin de chaîne
        }
        let ch = match value {
            0..=31 => '@' as u8 + value, // @, A-Z, [, \, ], ^, _
            32..=63 => ' ' as u8 + (value - 32), // Espace et caractères imprimables
            _ => b'?',
        };
        result.push(ch as char);
    }
    result.trim().to_string()
}

// Fonctions d'extraction spécifiques pour les champs AIS

fn extract_mmsi(data: &[u8]) -> Result<Option<u32>, NmeaError> {
    if data.len() < 2 {
        return Ok(None);
    }
    Ok(Some(extract_uint(data, 8, 30)))
}

fn extract_navigation_status(data: &[u8]) -> Option<u8> {
    if data.len() < 2 {
        return None;
    }
    Some(extract_uint(data, 38, 4) as u8)
}

fn extract_rate_of_turn(data: &[u8]) -> Option<i8> {
    if data.len() < 2 {
        return None;
    }
    Some(extract_int(data, 42, 8) as i8)
}

fn extract_speed_over_ground(data: &[u8]) -> Option<u16> {
    if data.len() < 2 {
        return None;
    }
    Some(extract_uint(data, 50, 10) as u16)
}

fn extract_position_accuracy(data: &[u8]) -> Option<bool> {
    if data.len() < 2 {
        return None;
    }
    Some(extract_uint(data, 60, 1) != 0)
}

fn extract_longitude(data: &[u8]) -> Option<i32> {
    if data.len() < 5 {
        return None;
    }
    Some(extract_int(data, 61, 28))
}

fn extract_latitude(data: &[u8]) -> Option<i32> {
    if data.len() < 5 {
        return None;
    }
    Some(extract_int(data, 89, 27))
}

fn extract_course_over_ground(data: &[u8]) -> Option<u16> {
    if data.len() < 5 {
        return None;
    }
    Some(extract_uint(data, 116, 12) as u16)
}

fn extract_true_heading(data: &[u8]) -> Option<u16> {
    if data.len() < 5 {
        return None;
    }
    Some(extract_uint(data, 128, 9) as u16)
}

fn extract_timestamp(data: &[u8]) -> Option<u8> {
    if data.len() < 5 {
        return None;
    }
    Some(extract_uint(data, 137, 6) as u8)
}

fn extract_call_sign(data: &[u8]) -> Option<String> {
    if data.len() < 8 {
        return None;
    }
    Some(extract_string(data, 112, 42))
}

fn extract_vessel_name(data: &[u8]) -> Option<String> {
    if data.len() < 8 {
        return None;
    }
    Some(extract_string(data, 112, 120))
}

fn extract_ship_type(data: &[u8]) -> Option<u8> {
    if data.len() < 8 {
        return None;
    }
    Some(extract_uint(data, 232, 8) as u8)
}

fn extract_dimensions(data: &[u8]) -> Option<AisDimensions> {
    if data.len() < 8 {
        return None;
    }
    Some(AisDimensions {
        length: extract_uint(data, 240, 9) as u16,
        width: extract_uint(data, 249, 9) as u16,
    })
}

fn extract_position_fix_type(data: &[u8]) -> Option<u8> {
    if data.len() < 8 {
        return None;
    }
    Some(extract_uint(data, 258, 4) as u8)
}

fn extract_eta(data: &[u8]) -> Option<AisEta> {
    if data.len() < 8 {
        return None;
    }
    Some(AisEta {
        month: extract_uint(data, 262, 4) as u8,
        day: extract_uint(data, 266, 5) as u8,
        hour: extract_uint(data, 271, 5) as u8,
        minute: extract_uint(data, 276, 6) as u8,
    })
}

fn extract_draught(data: &[u8]) -> Option<u16> {
    if data.len() < 8 {
        return None;
    }
    Some(extract_uint(data, 282, 8) as u16)
}

fn extract_destination(data: &[u8]) -> Option<String> {
    if data.len() < 8 {
        return None;
    }
    Some(extract_string(data, 290, 120))
}

