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
    
    /// Sérialise le message SBS en chaîne de caractères.
    /// 
    /// Reconstruit le message dans le format SBS (BaseStation) standard sans espaces/tabulations supplémentaires.
    /// Format: `MSG,type,transmission_type,session_id,aircraft_id,hex_ident,flight_id,date_gen,time_gen,date_log,time_log,callsign,altitude,speed,track,lat,lon,vr,squawk,alert,emergency,spi,is_on_ground`
    /// 
    /// # Returns
    /// * `String` - Message SBS sérialisé
    /// 
    /// # Exemples
    /// ```
    /// use aftn::{SbsParser, SbsMessage};
    /// let input = "MSG,3,145,29315,4CA2E6,27215,2015/02/05,14:53:22.734,2015/02/05,14:53:22.734,,37025,1035.0,295.6,51.4703,-0.4543,,,,,0";
    /// let message = SbsParser::parse_message(input)?;
    /// let serialized = message.serialize();
    /// ```
    pub fn serialize(&self) -> String {
        let mut result = String::new();
        
        // MSG,type
        result.push_str("MSG,");
        result.push_str(&self.message_type.msg_number().to_string());
        result.push(',');
        
        // transmission_type
        result.push_str(&self.transmission_type.to_string());
        result.push(',');
        
        // session_id
        if let Some(ref val) = self.session_id {
            result.push_str(val);
        }
        result.push(',');
        
        // aircraft_id
        if let Some(ref val) = self.aircraft_id {
            result.push_str(val);
        }
        result.push(',');
        
        // hex_ident
        if let Some(ref val) = self.hex_ident {
            result.push_str(val);
        }
        result.push(',');
        
        // flight_id
        if let Some(ref val) = self.flight_id {
            result.push_str(val);
        }
        result.push(',');
        
        // date_message_generated
        if let Some(ref val) = self.date_message_generated {
            result.push_str(val);
        }
        result.push(',');
        
        // time_message_generated
        if let Some(ref val) = self.time_message_generated {
            result.push_str(val);
        }
        result.push(',');
        
        // date_message_logged
        if let Some(ref val) = self.date_message_logged {
            result.push_str(val);
        }
        result.push(',');
        
        // time_message_logged
        if let Some(ref val) = self.time_message_logged {
            result.push_str(val);
        }
        result.push(',');
        
        // callsign
        if let Some(ref val) = self.callsign {
            result.push_str(val);
        }
        result.push(',');
        
        // altitude
        if let Some(ref val) = self.altitude {
            result.push_str(&val.to_string());
        }
        result.push(',');
        
        // ground_speed
        if let Some(ref val) = self.ground_speed {
            result.push_str(&val.to_string());
        }
        result.push(',');
        
        // track
        if let Some(ref val) = self.track {
            result.push_str(&val.to_string());
        }
        result.push(',');
        
        // latitude
        if let Some(ref val) = self.latitude {
            result.push_str(&val.to_string());
        }
        result.push(',');
        
        // longitude
        if let Some(ref val) = self.longitude {
            result.push_str(&val.to_string());
        }
        result.push(',');
        
        // vertical_rate
        if let Some(ref val) = self.vertical_rate {
            result.push_str(&val.to_string());
        }
        result.push(',');
        
        // squawk
        if let Some(ref val) = self.squawk {
            result.push_str(val);
        }
        result.push(',');
        
        // alert
        if let Some(ref val) = self.alert {
            result.push_str(if *val { "1" } else { "0" });
        }
        result.push(',');
        
        // emergency
        if let Some(ref val) = self.emergency {
            result.push_str(if *val { "1" } else { "0" });
        }
        result.push(',');
        
        // spi
        if let Some(ref val) = self.spi {
            result.push_str(if *val { "1" } else { "0" });
        }
        result.push(',');
        
        // is_on_ground
        if let Some(ref val) = self.is_on_ground {
            result.push_str(if *val { "1" } else { "0" });
        }
        
        result
    }
}

