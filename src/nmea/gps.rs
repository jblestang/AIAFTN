//! Structures de données détaillées pour les messages GPS NMEA 0183

use serde::{Deserialize, Serialize};
use crate::nmea::error::NmeaError;
use crate::nmea::message::NmeaMessage;

/// Message GPGGA (Global Positioning System Fix Data)
/// Format: $GPGGA,hhmmss.ss,llll.ll,a,yyyyy.yy,a,x,xx,x.x,x.x,M,x.x,M,x.x,xxxx*hh
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GgaMessage {
    /// Time (UTC) - hhmmss.ss
    pub time: Option<String>,
    /// Latitude - llll.ll
    pub latitude: Option<f64>,
    /// Latitude direction - N or S
    pub latitude_direction: Option<char>,
    /// Longitude - yyyyy.yy
    pub longitude: Option<f64>,
    /// Longitude direction - E or W
    pub longitude_direction: Option<char>,
    /// GPS quality indicator (0=no fix, 1=GPS fix, 2=DGPS fix)
    pub quality: Option<u8>,
    /// Number of satellites being tracked
    pub satellites: Option<u8>,
    /// Horizontal dilution of position
    pub hdop: Option<f64>,
    /// Altitude above mean sea level
    pub altitude: Option<f64>,
    /// Altitude units (M=meters)
    pub altitude_units: Option<char>,
    /// Height of geoid above WGS84 ellipsoid
    pub geoid_height: Option<f64>,
    /// Geoid height units (M=meters)
    pub geoid_height_units: Option<char>,
    /// Time since last DGPS update (seconds)
    pub dgps_age: Option<f64>,
    /// DGPS station ID
    pub dgps_station_id: Option<u16>,
}

impl GgaMessage {
    /// Parse un message GPGGA depuis un NmeaMessage
    pub fn from_nmea(message: &NmeaMessage) -> Result<Self, NmeaError> {
        if message.fields.len() < 14 {
            return Err(NmeaError::InvalidFormat(
                "GPGGA message must have at least 14 fields".to_string()
            ));
        }
        
        Ok(GgaMessage {
            time: message.get_field(0).cloned(),
            latitude: message.get_field(1)
                .and_then(|s| if s.is_empty() { None } else { s.parse::<f64>().ok() }),
            latitude_direction: message.get_field(2)
                .and_then(|s| if s.is_empty() { None } else { s.chars().next() }),
            longitude: message.get_field(3)
                .and_then(|s| if s.is_empty() { None } else { s.parse::<f64>().ok() }),
            longitude_direction: message.get_field(4)
                .and_then(|s| if s.is_empty() { None } else { s.chars().next() }),
            quality: message.get_field(5)
                .and_then(|s| if s.is_empty() { None } else { s.parse::<u8>().ok() }),
            satellites: message.get_field(6)
                .and_then(|s| if s.is_empty() { None } else { s.parse::<u8>().ok() }),
            hdop: message.get_field(7)
                .and_then(|s| if s.is_empty() { None } else { s.parse::<f64>().ok() }),
            altitude: message.get_field(8)
                .and_then(|s| if s.is_empty() { None } else { s.parse::<f64>().ok() }),
            altitude_units: message.get_field(9)
                .and_then(|s| if s.is_empty() { None } else { s.chars().next() }),
            geoid_height: message.get_field(10)
                .and_then(|s| if s.is_empty() { None } else { s.parse::<f64>().ok() }),
            geoid_height_units: message.get_field(11)
                .and_then(|s| if s.is_empty() { None } else { s.chars().next() }),
            dgps_age: message.get_field(12)
                .and_then(|s| if s.is_empty() { None } else { s.parse::<f64>().ok() }),
            dgps_station_id: message.get_field(13)
                .and_then(|s| if s.is_empty() { None } else { s.parse::<u16>().ok() }),
        })
    }
    
    /// Convertit la latitude en degrés décimaux
    pub fn latitude_decimal(&self) -> Option<f64> {
        self.latitude.and_then(|lat| {
            self.latitude_direction.map(|dir| {
                let degrees = (lat / 100.0).floor();
                let minutes = lat - (degrees * 100.0);
                let decimal = degrees + (minutes / 60.0);
                if dir == 'S' { -decimal } else { decimal }
            })
        })
    }
    
    /// Convertit la longitude en degrés décimaux
    pub fn longitude_decimal(&self) -> Option<f64> {
        self.longitude.and_then(|lon| {
            self.longitude_direction.map(|dir| {
                let degrees = (lon / 100.0).floor();
                let minutes = lon - (degrees * 100.0);
                let decimal = degrees + (minutes / 60.0);
                if dir == 'W' { -decimal } else { decimal }
            })
        })
    }
}

/// Message GPRMC (Recommended Minimum Specific GPS/Transit Data)
/// Format: $GPRMC,hhmmss.ss,A,llll.ll,a,yyyyy.yy,a,x.x,x.x,ddmmyy,x.x,a*hh
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RmcMessage {
    /// Time (UTC) - hhmmss.ss
    pub time: Option<String>,
    /// Status - A=active, V=void
    pub status: Option<char>,
    /// Latitude - llll.ll
    pub latitude: Option<f64>,
    /// Latitude direction - N or S
    pub latitude_direction: Option<char>,
    /// Longitude - yyyyy.yy
    pub longitude: Option<f64>,
    /// Longitude direction - E or W
    pub longitude_direction: Option<char>,
    /// Speed over ground (knots)
    pub speed: Option<f64>,
    /// Course over ground (degrees)
    pub course: Option<f64>,
    /// Date - ddmmyy
    pub date: Option<String>,
    /// Magnetic variation (degrees)
    pub magnetic_variation: Option<f64>,
    /// Magnetic variation direction - E or W
    pub magnetic_variation_direction: Option<char>,
}

impl RmcMessage {
    /// Parse un message GPRMC depuis un NmeaMessage
    pub fn from_nmea(message: &NmeaMessage) -> Result<Self, NmeaError> {
        if message.fields.len() < 11 {
            return Err(NmeaError::InvalidFormat(
                "GPRMC message must have at least 11 fields".to_string()
            ));
        }
        
        Ok(RmcMessage {
            time: message.get_field(0).cloned(),
            status: message.get_field(1)
                .and_then(|s| if s.is_empty() { None } else { s.chars().next() }),
            latitude: message.get_field(2)
                .and_then(|s| if s.is_empty() { None } else { s.parse::<f64>().ok() }),
            latitude_direction: message.get_field(3)
                .and_then(|s| if s.is_empty() { None } else { s.chars().next() }),
            longitude: message.get_field(4)
                .and_then(|s| if s.is_empty() { None } else { s.parse::<f64>().ok() }),
            longitude_direction: message.get_field(5)
                .and_then(|s| if s.is_empty() { None } else { s.chars().next() }),
            speed: message.get_field(6)
                .and_then(|s| if s.is_empty() { None } else { s.parse::<f64>().ok() }),
            course: message.get_field(7)
                .and_then(|s| if s.is_empty() { None } else { s.parse::<f64>().ok() }),
            date: message.get_field(8).cloned(),
            magnetic_variation: message.get_field(9)
                .and_then(|s| if s.is_empty() { None } else { s.parse::<f64>().ok() }),
            magnetic_variation_direction: message.get_field(10)
                .and_then(|s| if s.is_empty() { None } else { s.chars().next() }),
        })
    }
    
    /// Convertit la latitude en degrés décimaux
    pub fn latitude_decimal(&self) -> Option<f64> {
        self.latitude.and_then(|lat| {
            self.latitude_direction.map(|dir| {
                let degrees = (lat / 100.0).floor();
                let minutes = lat - (degrees * 100.0);
                let decimal = degrees + (minutes / 60.0);
                if dir == 'S' { -decimal } else { decimal }
            })
        })
    }
    
    /// Convertit la longitude en degrés décimaux
    pub fn longitude_decimal(&self) -> Option<f64> {
        self.longitude.and_then(|lon| {
            self.longitude_direction.map(|dir| {
                let degrees = (lon / 100.0).floor();
                let minutes = lon - (degrees * 100.0);
                let decimal = degrees + (minutes / 60.0);
                if dir == 'W' { -decimal } else { decimal }
            })
        })
    }
}

/// Message GPGSA (GPS DOP and Active Satellites)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GsaMessage {
    /// Selection mode - M=manual, A=automatic
    pub selection_mode: Option<char>,
    /// Fix mode - 1=no fix, 2=2D fix, 3=3D fix
    pub fix_mode: Option<u8>,
    /// Satellite PRN numbers (up to 12)
    pub satellites: Vec<Option<u16>>,
    /// Position dilution of precision
    pub pdop: Option<f64>,
    /// Horizontal dilution of precision
    pub hdop: Option<f64>,
    /// Vertical dilution of precision
    pub vdop: Option<f64>,
}

impl GsaMessage {
    /// Parse un message GPGSA depuis un NmeaMessage
    pub fn from_nmea(message: &NmeaMessage) -> Result<Self, NmeaError> {
        if message.fields.len() < 17 {
            return Err(NmeaError::InvalidFormat(
                "GPGSA message must have at least 17 fields".to_string()
            ));
        }
        
        let mut satellites = Vec::new();
        for i in 2..14 {
            satellites.push(
                message.get_field(i)
                    .and_then(|s| if s.is_empty() { None } else { s.parse::<u16>().ok() })
            );
        }
        
        Ok(GsaMessage {
            selection_mode: message.get_field(0)
                .and_then(|s| if s.is_empty() { None } else { s.chars().next() }),
            fix_mode: message.get_field(1)
                .and_then(|s| if s.is_empty() { None } else { s.parse::<u8>().ok() }),
            satellites,
            pdop: message.get_field(14)
                .and_then(|s| if s.is_empty() { None } else { s.parse::<f64>().ok() }),
            hdop: message.get_field(15)
                .and_then(|s| if s.is_empty() { None } else { s.parse::<f64>().ok() }),
            vdop: message.get_field(16)
                .and_then(|s| if s.is_empty() { None } else { s.parse::<f64>().ok() }),
        })
    }
}

/// Message GPVTG (Track Made Good and Ground Speed)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VtgMessage {
    /// Course over ground (degrees True)
    pub course_true: Option<f64>,
    /// Course over ground (degrees Magnetic)
    pub course_magnetic: Option<f64>,
    /// Speed over ground (knots)
    pub speed_knots: Option<f64>,
    /// Speed over ground (km/h)
    pub speed_kmh: Option<f64>,
}

impl VtgMessage {
    /// Parse un message GPVTG depuis un NmeaMessage
    /// Format: $GPVTG,cogt,T,cogm,M,sog,N,sog,K,mode*hh
    /// Les champs pairs (0,2,4,6) contiennent les valeurs, les champs impairs (1,3,5,7) contiennent les unités
    pub fn from_nmea(message: &NmeaMessage) -> Result<Self, NmeaError> {
        // GPVTG peut avoir 8 ou 9 champs (le dernier est optionnel)
        if message.fields.len() < 8 {
            return Err(NmeaError::InvalidFormat(
                "GPVTG message must have at least 8 fields".to_string()
            ));
        }
        
        Ok(VtgMessage {
            course_true: message.get_field(0)
                .and_then(|s| if s.is_empty() { None } else { s.parse::<f64>().ok() }),
            course_magnetic: message.get_field(2)
                .and_then(|s| if s.is_empty() { None } else { s.parse::<f64>().ok() }),
            speed_knots: message.get_field(4)
                .and_then(|s| if s.is_empty() { None } else { s.parse::<f64>().ok() }),
            speed_kmh: message.get_field(6)
                .and_then(|s| if s.is_empty() { None } else { s.parse::<f64>().ok() }),
        })
    }
}

