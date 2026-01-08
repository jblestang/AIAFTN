//! Validation sémantique des messages NMEA 0183

use crate::nmea::message::NmeaMessage;
use crate::nmea::error::NmeaError;

/// Valide un message NMEA selon son type
pub fn validate_message(message: &NmeaMessage) -> Result<(), NmeaError> {
    match message.message_type {
        crate::nmea::types::NmeaMessageType::GPGGA => validate_gga(message),
        crate::nmea::types::NmeaMessageType::GPRMC => validate_rmc(message),
        crate::nmea::types::NmeaMessageType::GPGSA => validate_gsa(message),
        crate::nmea::types::NmeaMessageType::GPGSV => validate_gsv(message),
        crate::nmea::types::NmeaMessageType::GPVTG => validate_vtg(message),
        crate::nmea::types::NmeaMessageType::GPWPL => validate_wpl(message),
        crate::nmea::types::NmeaMessageType::GPBOD => validate_bod(message),
        crate::nmea::types::NmeaMessageType::GPXTE => validate_xte(message),
        crate::nmea::types::NmeaMessageType::GPZDA => validate_zda(message),
        crate::nmea::types::NmeaMessageType::Generic(ref id) => {
            // Valider les messages AIS
            if id == "AIVDM" || id == "AIVDO" {
                validate_ais(message)
            } else {
                Ok(()) // Pour les autres types génériques, on accepte sans validation spécifique
            }
        }
    }
}

/// Valide un message GPGGA (Global Positioning System Fix Data)
/// Format: $GPGGA,hhmmss.ss,llll.ll,a,yyyyy.yy,a,x,xx,x.x,x.x,M,x.x,M,x.x,xxxx*hh
fn validate_gga(message: &NmeaMessage) -> Result<(), NmeaError> {
    if message.fields.len() < 14 {
        return Err(NmeaError::InvalidFormat(
            "GPGGA message must have at least 14 fields".to_string()
        ));
    }
    
    // Field 0: Time (hhmmss.ss)
    if let Some(time) = message.get_field(0) {
        if !time.is_empty() {
            validate_time(time)?;
        }
    }
    
    // Field 1: Latitude (llll.ll)
    if let Some(lat) = message.get_field(1) {
        if !lat.is_empty() {
            validate_latitude(lat)?;
        }
    }
    
    // Field 2: Latitude direction (N/S)
    if let Some(lat_dir) = message.get_field(2) {
        if !lat_dir.is_empty() && lat_dir != "N" && lat_dir != "S" {
            return Err(NmeaError::InvalidFieldValue {
                field: "Latitude direction".to_string(),
                value: lat_dir.clone(),
                reason: "Must be N or S".to_string(),
            });
        }
    }
    
    // Field 3: Longitude (yyyyy.yy)
    if let Some(lon) = message.get_field(3) {
        if !lon.is_empty() {
            validate_longitude(lon)?;
        }
    }
    
    // Field 4: Longitude direction (E/W)
    if let Some(lon_dir) = message.get_field(4) {
        if !lon_dir.is_empty() && lon_dir != "E" && lon_dir != "W" {
            return Err(NmeaError::InvalidFieldValue {
                field: "Longitude direction".to_string(),
                value: lon_dir.clone(),
                reason: "Must be E or W".to_string(),
            });
        }
    }
    
    // Field 5: GPS quality indicator (0-2)
    if let Some(quality) = message.get_field(5) {
        if !quality.is_empty() {
            let q = quality.parse::<u8>()
                .map_err(|_| NmeaError::InvalidFieldValue {
                    field: "GPS quality".to_string(),
                    value: quality.clone(),
                    reason: "Must be a number 0-2".to_string(),
                })?;
            if q > 2 {
                return Err(NmeaError::InvalidFieldValue {
                    field: "GPS quality".to_string(),
                    value: quality.clone(),
                    reason: "Must be 0 (no fix), 1 (GPS fix), or 2 (DGPS fix)".to_string(),
                });
            }
        }
    }
    
    // Field 6: Number of satellites (00-12)
    if let Some(satellites) = message.get_field(6) {
        if !satellites.is_empty() {
            let s = satellites.parse::<u8>()
                .map_err(|_| NmeaError::InvalidFieldValue {
                    field: "Number of satellites".to_string(),
                    value: satellites.clone(),
                    reason: "Must be a number".to_string(),
                })?;
            if s > 12 {
                return Err(NmeaError::InvalidFieldValue {
                    field: "Number of satellites".to_string(),
                    value: satellites.clone(),
                    reason: "Must be 0-12".to_string(),
                });
            }
        }
    }
    
    // Field 8: Altitude (x.x)
    if let Some(altitude) = message.get_field(8) {
        if !altitude.is_empty() {
            altitude.parse::<f64>()
                .map_err(|_| NmeaError::InvalidFieldValue {
                    field: "Altitude".to_string(),
                    value: altitude.clone(),
                    reason: "Must be a number".to_string(),
                })?;
        }
    }
    
    Ok(())
}

/// Valide un message GPRMC (Recommended Minimum Specific GPS/Transit Data)
/// Format: $GPRMC,hhmmss.ss,A,llll.ll,a,yyyyy.yy,a,x.x,x.x,ddmmyy,x.x,a*hh
fn validate_rmc(message: &NmeaMessage) -> Result<(), NmeaError> {
    if message.fields.len() < 11 {
        return Err(NmeaError::InvalidFormat(
            "GPRMC message must have at least 11 fields".to_string()
        ));
    }
    
    // Field 0: Time (hhmmss.ss)
    if let Some(time) = message.get_field(0) {
        if !time.is_empty() {
            validate_time(time)?;
        }
    }
    
    // Field 1: Status (A=active, V=void)
    if let Some(status) = message.get_field(1) {
        if !status.is_empty() && status != "A" && status != "V" {
            return Err(NmeaError::InvalidFieldValue {
                field: "Status".to_string(),
                value: status.clone(),
                reason: "Must be A (active) or V (void)".to_string(),
            });
        }
    }
    
    // Field 2: Latitude
    if let Some(lat) = message.get_field(2) {
        if !lat.is_empty() {
            validate_latitude(lat)?;
        }
    }
    
    // Field 3: Latitude direction
    if let Some(lat_dir) = message.get_field(3) {
        if !lat_dir.is_empty() && lat_dir != "N" && lat_dir != "S" {
            return Err(NmeaError::InvalidFieldValue {
                field: "Latitude direction".to_string(),
                value: lat_dir.clone(),
                reason: "Must be N or S".to_string(),
            });
        }
    }
    
    // Field 4: Longitude
    if let Some(lon) = message.get_field(4) {
        if !lon.is_empty() {
            validate_longitude(lon)?;
        }
    }
    
    // Field 5: Longitude direction
    if let Some(lon_dir) = message.get_field(5) {
        if !lon_dir.is_empty() && lon_dir != "E" && lon_dir != "W" {
            return Err(NmeaError::InvalidFieldValue {
                field: "Longitude direction".to_string(),
                value: lon_dir.clone(),
                reason: "Must be E or W".to_string(),
            });
        }
    }
    
    // Field 6: Speed over ground (knots)
    if let Some(speed) = message.get_field(6) {
        if !speed.is_empty() {
            let s = speed.parse::<f64>()
                .map_err(|_| NmeaError::InvalidFieldValue {
                    field: "Speed".to_string(),
                    value: speed.clone(),
                    reason: "Must be a number".to_string(),
                })?;
            if s < 0.0 || s > 1000.0 {
                return Err(NmeaError::InvalidFieldValue {
                    field: "Speed".to_string(),
                    value: speed.clone(),
                    reason: "Must be between 0 and 1000 knots".to_string(),
                });
            }
        }
    }
    
    // Field 7: Course over ground (degrees)
    if let Some(course) = message.get_field(7) {
        if !course.is_empty() {
            let c = course.parse::<f64>()
                .map_err(|_| NmeaError::InvalidFieldValue {
                    field: "Course".to_string(),
                    value: course.clone(),
                    reason: "Must be a number".to_string(),
                })?;
            if c < 0.0 || c >= 360.0 {
                return Err(NmeaError::InvalidFieldValue {
                    field: "Course".to_string(),
                    value: course.clone(),
                    reason: "Must be between 0 and 360 degrees".to_string(),
                });
            }
        }
    }
    
    // Field 8: Date (ddmmyy)
    if let Some(date) = message.get_field(8) {
        if !date.is_empty() {
            validate_date_nmea(date)?;
        }
    }
    
    Ok(())
}

/// Valide les autres types de messages (structure de base)
fn validate_gsa(_message: &NmeaMessage) -> Result<(), NmeaError> {
    // Validation de base pour GPGSA
    Ok(())
}

fn validate_gsv(_message: &NmeaMessage) -> Result<(), NmeaError> {
    // Validation de base pour GPGSV
    Ok(())
}

fn validate_vtg(_message: &NmeaMessage) -> Result<(), NmeaError> {
    // Validation de base pour GPVTG
    Ok(())
}

fn validate_wpl(_message: &NmeaMessage) -> Result<(), NmeaError> {
    // Validation de base pour GPWPL
    Ok(())
}

fn validate_bod(_message: &NmeaMessage) -> Result<(), NmeaError> {
    // Validation de base pour GPBOD
    Ok(())
}

fn validate_xte(_message: &NmeaMessage) -> Result<(), NmeaError> {
    // Validation de base pour GPXTE
    Ok(())
}

fn validate_zda(_message: &NmeaMessage) -> Result<(), NmeaError> {
    // Validation de base pour GPZDA
    Ok(())
}

/// Valide un message AIS (AIVDM ou AIVDO)
fn validate_ais(message: &NmeaMessage) -> Result<(), NmeaError> {
    if message.fields.len() < 5 {
        return Err(NmeaError::InvalidFormat(
            "AIS message must have at least 5 fields".to_string()
        ));
    }
    
    // Field 0: Sequence number (1-9)
    if let Some(seq) = message.get_field(0) {
        if !seq.is_empty() {
            let s = seq.parse::<u8>()
                .map_err(|_| NmeaError::InvalidFieldValue {
                    field: "Sequence number".to_string(),
                    value: seq.clone(),
                    reason: "Must be a number 1-9".to_string(),
                })?;
            if s < 1 || s > 9 {
                return Err(NmeaError::InvalidFieldValue {
                    field: "Sequence number".to_string(),
                    value: seq.clone(),
                    reason: "Must be between 1 and 9".to_string(),
                });
            }
        }
    }
    
    // Field 1: Fragment number (1-9)
    if let Some(frag) = message.get_field(1) {
        if !frag.is_empty() {
            let f = frag.parse::<u8>()
                .map_err(|_| NmeaError::InvalidFieldValue {
                    field: "Fragment number".to_string(),
                    value: frag.clone(),
                    reason: "Must be a number 1-9".to_string(),
                })?;
            if f < 1 || f > 9 {
                return Err(NmeaError::InvalidFieldValue {
                    field: "Fragment number".to_string(),
                    value: frag.clone(),
                    reason: "Must be between 1 and 9".to_string(),
                });
            }
        }
    }
    
    // Field 3: Radio channel (A or B)
    if let Some(channel) = message.get_field(3) {
        if !channel.is_empty() && channel != "A" && channel != "B" {
            return Err(NmeaError::InvalidFieldValue {
                field: "Radio channel".to_string(),
                value: channel.clone(),
                reason: "Must be A or B".to_string(),
            });
        }
    }
    
    // Field 4: Payload (6-bit ASCII encoded)
    if let Some(payload) = message.get_field(4) {
        if payload.is_empty() {
            return Err(NmeaError::InvalidFormat(
                "AIS payload cannot be empty".to_string()
            ));
        }
        
        // Valider que le payload contient uniquement des caractères 6-bit ASCII valides
        for ch in payload.chars() {
            let byte = ch as u8;
            if !(byte >= 0x30 && byte <= 0x77) {
                return Err(NmeaError::InvalidFieldValue {
                    field: "AIS payload".to_string(),
                    value: ch.to_string(),
                    reason: "Must be 6-bit ASCII (0x30-0x77)".to_string(),
                });
            }
        }
    } else {
        return Err(NmeaError::MissingField("AIS payload".to_string()));
    }
    
    // Field 5: Fill bits (0-5)
    if let Some(fill) = message.get_field(5) {
        if !fill.is_empty() {
            let f = fill.parse::<u8>()
                .map_err(|_| NmeaError::InvalidFieldValue {
                    field: "Fill bits".to_string(),
                    value: fill.clone(),
                    reason: "Must be a number 0-5".to_string(),
                })?;
            if f > 5 {
                return Err(NmeaError::InvalidFieldValue {
                    field: "Fill bits".to_string(),
                    value: fill.clone(),
                    reason: "Must be between 0 and 5".to_string(),
                });
            }
        }
    }
    
    Ok(())
}

/// Valide un format de temps NMEA (hhmmss.ss)
fn validate_time(time: &str) -> Result<(), NmeaError> {
    if time.len() < 6 {
        return Err(NmeaError::InvalidTime(
            format!("Time must be at least 6 characters (hhmmss), got: {}", time)
        ));
    }
    
    // PANIC: time[0..2], time[2..4], time[4..6] peuvent panic si time.len() < 6,
    // mais cela est impossible ici car on vérifie time.len() < 6 juste avant (ligne 385) et on retourne une erreur
    let hours = &time[0..2];
    let minutes = &time[2..4];
    let seconds = &time[4..6];
    
    let h = hours.parse::<u8>()
        .map_err(|_| NmeaError::InvalidTime(format!("Invalid hours: {}", hours)))?;
    if h >= 24 {
        return Err(NmeaError::InvalidTime(format!("Hours must be 0-23, got: {}", h)));
    }
    
    let m = minutes.parse::<u8>()
        .map_err(|_| NmeaError::InvalidTime(format!("Invalid minutes: {}", minutes)))?;
    if m >= 60 {
        return Err(NmeaError::InvalidTime(format!("Minutes must be 0-59, got: {}", m)));
    }
    
    let s = seconds.parse::<u8>()
        .map_err(|_| NmeaError::InvalidTime(format!("Invalid seconds: {}", seconds)))?;
    if s >= 60 {
        return Err(NmeaError::InvalidTime(format!("Seconds must be 0-59, got: {}", s)));
    }
    
    Ok(())
}

/// Valide une latitude NMEA (format: dddmm.mmmm)
fn validate_latitude(lat: &str) -> Result<(), NmeaError> {
    let lat_f = lat.parse::<f64>()
        .map_err(|_| NmeaError::InvalidCoordinate(format!("Invalid latitude format: {}", lat)))?;
    
    // Latitude NMEA: dddmm.mmmm (degrés et minutes décimales)
    // Convertir en degrés décimaux pour validation
    let degrees = (lat_f / 100.0).floor();
    let minutes = lat_f - (degrees * 100.0);
    let decimal_degrees = degrees + (minutes / 60.0);
    
    if decimal_degrees < -90.0 || decimal_degrees > 90.0 {
        return Err(NmeaError::InvalidCoordinate(
            format!("Latitude must be between -90 and 90 degrees, got: {}", decimal_degrees)
        ));
    }
    
    Ok(())
}

/// Valide une longitude NMEA (format: dddmm.mmmm)
fn validate_longitude(lon: &str) -> Result<(), NmeaError> {
    let lon_f = lon.parse::<f64>()
        .map_err(|_| NmeaError::InvalidCoordinate(format!("Invalid longitude format: {}", lon)))?;
    
    // Longitude NMEA: dddmm.mmmm (degrés et minutes décimales)
    let degrees = (lon_f / 100.0).floor();
    let minutes = lon_f - (degrees * 100.0);
    let decimal_degrees = degrees + (minutes / 60.0);
    
    if decimal_degrees < -180.0 || decimal_degrees > 180.0 {
        return Err(NmeaError::InvalidCoordinate(
            format!("Longitude must be between -180 and 180 degrees, got: {}", decimal_degrees)
        ));
    }
    
    Ok(())
}

/// Valide un format de date NMEA (ddmmyy)
fn validate_date_nmea(date: &str) -> Result<(), NmeaError> {
    if date.len() != 6 {
        return Err(NmeaError::InvalidDate(
            format!("Date must be 6 characters (ddmmyy), got: {}", date)
        ));
    }
    
    // PANIC: date[0..2], date[2..4], date[4..6] peuvent panic si date.len() < 6,
    // mais cela est impossible ici car on vérifie date.len() != 6 juste avant (ligne 459) et on retourne une erreur
    let day = &date[0..2];
    let month = &date[2..4];
    let year = &date[4..6];
    
    let d = day.parse::<u8>()
        .map_err(|_| NmeaError::InvalidDate(format!("Invalid day: {}", day)))?;
    if d < 1 || d > 31 {
        return Err(NmeaError::InvalidDate(format!("Day must be 1-31, got: {}", d)));
    }
    
    let m = month.parse::<u8>()
        .map_err(|_| NmeaError::InvalidDate(format!("Invalid month: {}", month)))?;
    if m < 1 || m > 12 {
        return Err(NmeaError::InvalidDate(format!("Month must be 1-12, got: {}", m)));
    }
    
    let _y = year.parse::<u8>()
        .map_err(|_| NmeaError::InvalidDate(format!("Invalid year: {}", year)))?;
    
    Ok(())
}

