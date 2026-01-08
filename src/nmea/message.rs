//! Structures de données pour les messages NMEA 0183

use serde::{Deserialize, Serialize};
use crate::nmea::types::NmeaMessageType;
use crate::nmea::error::NmeaError;

/// Représente un message NMEA 0183 complet
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NmeaMessage {
    /// Type de message (GPGGA, GPRMC, etc.)
    pub message_type: NmeaMessageType,
    
    /// Champs du message (index = position dans la phrase NMEA)
    pub fields: Vec<String>,
    
    /// Checksum fourni dans le message
    pub checksum: String,
    
    /// Données brutes du message (pour référence)
    pub raw: String,
}

impl NmeaMessage {
    /// Crée un nouveau message NMEA
    pub fn new(message_type: NmeaMessageType, fields: Vec<String>, checksum: String, raw: String) -> Self {
        NmeaMessage {
            message_type,
            fields,
            checksum,
            raw,
        }
    }
    
    /// Obtient un champ par index (0-based)
    pub fn get_field(&self, index: usize) -> Option<&String> {
        self.fields.get(index)
    }
    
    /// Obtient un champ par index, retourne une erreur si manquant
    pub fn get_field_required(&self, index: usize, name: &str) -> Result<&String, NmeaError> {
        self.fields.get(index)
            .ok_or_else(|| NmeaError::MissingField(format!("{} (field index {})", name, index)))
    }
    
    /// Valide le message (structure, checksum, et sémantique)
    pub fn validate(&self) -> Result<(), NmeaError> {
        // La validation du checksum est faite lors du parsing
        // Ici on valide la sémantique des champs
        crate::nmea::validation::validate_message(self)
    }
}

