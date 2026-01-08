//! Types de messages NMEA 0183

use serde::{Deserialize, Serialize};
use crate::nmea::error::NmeaError;

/// Types de messages NMEA 0183 supportés
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NmeaMessageType {
    /// Global Positioning System Fix Data
    GPGGA,
    /// Recommended Minimum Specific GPS/Transit Data
    GPRMC,
    /// GPS DOP and Active Satellites
    GPGSA,
    /// GPS Satellites in View
    GPGSV,
    /// Track Made Good and Ground Speed
    GPVTG,
    /// Waypoint Location
    GPWPL,
    /// Bearing and Distance to Waypoint
    GPBOD,
    /// Cross Track Error
    GPXTE,
    /// Time and Date
    GPZDA,
    /// Generic message type (non-standard or unknown)
    Generic(String),
}

impl NmeaMessageType {
    /// Détermine le type de message depuis l'identifiant NMEA
    pub fn from_identifier(identifier: &str) -> Result<Self, NmeaError> {
        match identifier {
            // GPS messages
            "GPGGA" => Ok(NmeaMessageType::GPGGA),
            "GPRMC" => Ok(NmeaMessageType::GPRMC),
            "GPGSA" => Ok(NmeaMessageType::GPGSA),
            "GPGSV" => Ok(NmeaMessageType::GPGSV),
            "GPVTG" => Ok(NmeaMessageType::GPVTG),
            "GPWPL" => Ok(NmeaMessageType::GPWPL),
            "GPBOD" => Ok(NmeaMessageType::GPBOD),
            "GPXTE" => Ok(NmeaMessageType::GPXTE),
            "GPZDA" => Ok(NmeaMessageType::GPZDA),
            // AIS messages
            "AIVDM" | "AIVDO" => Ok(NmeaMessageType::Generic(identifier.to_string())),
            _ => Ok(NmeaMessageType::Generic(identifier.to_string())),
        }
    }
    
    /// Retourne l'identifiant du type de message
    pub fn identifier(&self) -> &str {
        match self {
            NmeaMessageType::GPGGA => "GPGGA",
            NmeaMessageType::GPRMC => "GPRMC",
            NmeaMessageType::GPGSA => "GPGSA",
            NmeaMessageType::GPGSV => "GPGSV",
            NmeaMessageType::GPVTG => "GPVTG",
            NmeaMessageType::GPWPL => "GPWPL",
            NmeaMessageType::GPBOD => "GPBOD",
            NmeaMessageType::GPXTE => "GPXTE",
            NmeaMessageType::GPZDA => "GPZDA",
            NmeaMessageType::Generic(id) => id,
        }
    }
}

