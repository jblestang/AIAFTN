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
    
    /// Sérialise le message NMEA en chaîne de caractères.
    /// 
    /// Reconstruit le message dans le format NMEA standard sans espaces/tabulations supplémentaires.
    /// Recalcule le checksum pour garantir sa validité.
    /// Format: `$MESSAGE_TYPE,field1,field2,...,fieldN*CHECKSUM`
    /// 
    /// # Returns
    /// * `String` - Message NMEA sérialisé avec checksum valide
    /// 
    /// # Exemples
    /// ```
    /// use aftn::{NmeaParser, NmeaMessage};
    /// let input = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47";
    /// let message = NmeaParser::parse_message(input)?;
    /// let serialized = message.serialize();
    /// ```
    pub fn serialize(&self) -> String {
        // Déterminer le marqueur de début ($ ou !)
        let start_marker = if self.raw.starts_with('!') { '!' } else { '$' };
        
        // Construire la partie message (sans checksum)
        let mut message_part = String::new();
        message_part.push_str(&self.message_type.identifier());
        
        // Ajouter les champs séparés par des virgules
        for field in &self.fields {
            message_part.push(',');
            message_part.push_str(field);
        }
        
        // Calculer le checksum (même algorithme que dans le parser)
        let mut checksum: u8 = 0;
        for byte in message_part.bytes() {
            checksum ^= byte;
        }
        let checksum_str = format!("{:02X}", checksum);
        
        // Construire le message complet
        format!("{}{}*{}", start_marker, message_part, checksum_str)
    }
}

