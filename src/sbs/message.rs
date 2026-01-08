//! Structures de données pour les messages SBS (Mode-S/ADS-B)

use serde::{Deserialize, Serialize};
use crate::sbs::types::SbsMessageType;
use crate::sbs::error::SbsError;

/// Représente un message SBS complet
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SbsMessage {
    /// Type de message (MSG,1 à MSG,8)
    pub message_type: SbsMessageType,
    
    /// Type de transmission (1-8)
    pub transmission_type: u8,
    
    /// Session ID
    pub session_id: Option<String>,
    
    /// Aircraft ID (ICAO 24-bit address en hexadécimal)
    pub aircraft_id: Option<String>,
    
    /// Hex Ident (identifiant hexadécimal)
    pub hex_ident: Option<String>,
    
    /// Flight ID (identifiant de vol)
    pub flight_id: Option<String>,
    
    /// Date message généré
    pub date_message_generated: Option<String>,
    
    /// Time message généré
    pub time_message_generated: Option<String>,
    
    /// Date message logged
    pub date_message_logged: Option<String>,
    
    /// Time message logged
    pub time_message_logged: Option<String>,
    
    /// Callsign (indicatif d'appel)
    pub callsign: Option<String>,
    
    /// Altitude (pieds)
    pub altitude: Option<i32>,
    
    /// Ground Speed (noeuds)
    pub ground_speed: Option<f64>,
    
    /// Track (degrés)
    pub track: Option<f64>,
    
    /// Latitude (degrés décimaux)
    pub latitude: Option<f64>,
    
    /// Longitude (degrés décimaux)
    pub longitude: Option<f64>,
    
    /// Vertical Rate (ft/min)
    pub vertical_rate: Option<i32>,
    
    /// Squawk (code transpondeur)
    pub squawk: Option<String>,
    
    /// Alert (alerte)
    pub alert: Option<bool>,
    
    /// Emergency (urgence)
    pub emergency: Option<bool>,
    
    /// SPI (Special Position Indicator)
    pub spi: Option<bool>,
    
    /// Is on ground
    pub is_on_ground: Option<bool>,
    
    /// Données brutes du message (pour référence)
    pub raw: String,
}

impl SbsMessage {
    /// Crée un nouveau message SBS
    pub fn new(message_type: SbsMessageType, raw: String) -> Self {
        let transmission_type = message_type.msg_number();
        SbsMessage {
            message_type,
            transmission_type,
            session_id: None,
            aircraft_id: None,
            hex_ident: None,
            flight_id: None,
            date_message_generated: None,
            time_message_generated: None,
            date_message_logged: None,
            time_message_logged: None,
            callsign: None,
            altitude: None,
            ground_speed: None,
            track: None,
            latitude: None,
            longitude: None,
            vertical_rate: None,
            squawk: None,
            alert: None,
            emergency: None,
            spi: None,
            is_on_ground: None,
            raw,
        }
    }
    
    /// Valide le message (structure et sémantique)
    pub fn validate(&self) -> Result<(), SbsError> {
        crate::sbs::validation::validate_message(self)
    }
}

