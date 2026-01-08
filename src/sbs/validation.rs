//! Validation sémantique des messages SBS (Mode-S/ADS-B)

use crate::sbs::message::SbsMessage;
use crate::sbs::error::SbsError;

/// Valide un message SBS selon son type
pub fn validate_message(message: &SbsMessage) -> Result<(), SbsError> {
    match message.message_type {
        crate::sbs::types::SbsMessageType::Identification => validate_identification(message),
        crate::sbs::types::SbsMessageType::SurfacePosition => validate_surface_position(message),
        crate::sbs::types::SbsMessageType::AirbornePosition => validate_airborne_position(message),
        crate::sbs::types::SbsMessageType::AirborneVelocity => validate_airborne_velocity(message),
        crate::sbs::types::SbsMessageType::SurveillanceAltitude => validate_surveillance_altitude(message),
        crate::sbs::types::SbsMessageType::SurveillanceId => validate_surveillance_id(message),
        crate::sbs::types::SbsMessageType::AirToAir => validate_air_to_air(message),
        crate::sbs::types::SbsMessageType::AllCallReply => validate_all_call_reply(message),
        _ => Ok(()), // Pour les types génériques, validation de base
    }
}

/// Valide un message MSG,1 (Identification)
fn validate_identification(message: &SbsMessage) -> Result<(), SbsError> {
    // MSG,1 devrait avoir un callsign ou un hex_ident, mais ce n'est pas strictement requis
    // Valider l'adresse ICAO si présente
    if let Some(ref hex_ident) = message.hex_ident {
        validate_icao_address(hex_ident)?;
    }
    
    Ok(())
}

/// Valide un message MSG,2 (Surface Position)
fn validate_surface_position(message: &SbsMessage) -> Result<(), SbsError> {
    // MSG,2 doit avoir une position (latitude/longitude) ou altitude
    if message.latitude.is_none() && message.longitude.is_none() && message.altitude.is_none() {
        return Err(SbsError::MissingField(
            "MSG,2 must have position or altitude".to_string()
        ));
    }
    
    // Valider les coordonnées si présentes
    if let Some(lat) = message.latitude {
        if lat < -90.0 || lat > 90.0 {
            return Err(SbsError::InvalidFieldValue {
                field: "Latitude".to_string(),
                value: lat.to_string(),
                reason: "Must be between -90 and 90 degrees".to_string(),
            });
        }
    }
    
    if let Some(lon) = message.longitude {
        if lon < -180.0 || lon > 180.0 {
            return Err(SbsError::InvalidFieldValue {
                field: "Longitude".to_string(),
                value: lon.to_string(),
                reason: "Must be between -180 and 180 degrees".to_string(),
            });
        }
    }
    
    // Valider l'altitude si présente
    if let Some(alt) = message.altitude {
        if alt < -1000 || alt > 100000 {
            return Err(SbsError::InvalidAltitude(
                format!("Altitude must be between -1000 and 100000 feet, got: {}", alt)
            ));
        }
    }
    
    Ok(())
}

/// Valide un message MSG,3 (Airborne Position)
fn validate_airborne_position(message: &SbsMessage) -> Result<(), SbsError> {
    // MSG,3 devrait avoir une altitude, mais ce n'est pas toujours présent
    // Valider l'altitude si présente
    if let Some(alt) = message.altitude {
        if alt < -1000 || alt > 100000 {
            return Err(SbsError::InvalidAltitude(
                format!("Altitude must be between -1000 and 100000 feet, got: {}", alt)
            ));
        }
    }
    
    // Valider les coordonnées si présentes
    if let Some(lat) = message.latitude {
        if lat < -90.0 || lat > 90.0 {
            return Err(SbsError::InvalidFieldValue {
                field: "Latitude".to_string(),
                value: lat.to_string(),
                reason: "Must be between -90 and 90 degrees".to_string(),
            });
        }
    }
    
    if let Some(lon) = message.longitude {
        if lon < -180.0 || lon > 180.0 {
            return Err(SbsError::InvalidFieldValue {
                field: "Longitude".to_string(),
                value: lon.to_string(),
                reason: "Must be between -180 and 180 degrees".to_string(),
            });
        }
    }
    
    Ok(())
}

/// Valide un message MSG,4 (Airborne Velocity)
fn validate_airborne_velocity(message: &SbsMessage) -> Result<(), SbsError> {
    // MSG,4 doit avoir une vitesse ou un cap
    if message.ground_speed.is_none() && message.track.is_none() {
        return Err(SbsError::MissingField(
            "MSG,4 must have ground speed or track".to_string()
        ));
    }
    
    // Valider la vitesse si présente
    if let Some(speed) = message.ground_speed {
        if speed < 0.0 || speed > 2000.0 {
            return Err(SbsError::InvalidSpeed(
                format!("Ground speed must be between 0 and 2000 knots, got: {}", speed)
            ));
        }
    }
    
    // Valider le cap si présent
    if let Some(track) = message.track {
        if track < 0.0 || track >= 360.0 {
            return Err(SbsError::InvalidHeading(
                format!("Track must be between 0 and 360 degrees, got: {}", track)
            ));
        }
    }
    
    // Valider le taux de montée/descente si présent
    if let Some(vr) = message.vertical_rate {
        if vr < -10000 || vr > 10000 {
            return Err(SbsError::InvalidFieldValue {
                field: "Vertical rate".to_string(),
                value: vr.to_string(),
                reason: "Must be between -10000 and 10000 ft/min".to_string(),
            });
        }
    }
    
    Ok(())
}

/// Valide un message MSG,5 (Surveillance Altitude)
fn validate_surveillance_altitude(message: &SbsMessage) -> Result<(), SbsError> {
    // MSG,5 doit avoir une altitude
    if message.altitude.is_none() {
        return Err(SbsError::MissingField(
            "MSG,5 must have altitude".to_string()
        ));
    }
    
    // Valider l'altitude
    if let Some(alt) = message.altitude {
        if alt < -1000 || alt > 100000 {
            return Err(SbsError::InvalidAltitude(
                format!("Altitude must be between -1000 and 100000 feet, got: {}", alt)
            ));
        }
    }
    
    Ok(())
}

/// Valide un message MSG,6 (Surveillance ID)
fn validate_surveillance_id(message: &SbsMessage) -> Result<(), SbsError> {
    // MSG,6 doit avoir un squawk
    if message.squawk.is_none() {
        return Err(SbsError::MissingField(
            "MSG,6 must have squawk".to_string()
        ));
    }
    
    // Valider le squawk (code transpondeur, 4 chiffres octaux)
    if let Some(ref squawk) = message.squawk {
        if squawk.len() != 4 {
            return Err(SbsError::InvalidFieldValue {
                field: "Squawk".to_string(),
                value: squawk.clone(),
                reason: "Must be 4 digits".to_string(),
            });
        }
        
        for ch in squawk.chars() {
            if !ch.is_ascii_digit() || ch > '7' {
                return Err(SbsError::InvalidFieldValue {
                    field: "Squawk".to_string(),
                    value: squawk.clone(),
                    reason: "Must contain only octal digits (0-7)".to_string(),
                });
            }
        }
    }
    
    Ok(())
}

/// Valide un message MSG,7 (Air to Air)
fn validate_air_to_air(_message: &SbsMessage) -> Result<(), SbsError> {
    // Validation de base pour MSG,7
    Ok(())
}

/// Valide un message MSG,8 (All Call Reply)
fn validate_all_call_reply(message: &SbsMessage) -> Result<(), SbsError> {
    // MSG,8 doit avoir un hex_ident
    if message.hex_ident.is_none() {
        return Err(SbsError::MissingField(
            "MSG,8 must have hex_ident".to_string()
        ));
    }
    
    // Valider l'adresse ICAO
    if let Some(ref hex_ident) = message.hex_ident {
        validate_icao_address(hex_ident)?;
    }
    
    Ok(())
}

/// Valide une adresse ICAO 24-bit (format hexadécimal)
fn validate_icao_address(address: &str) -> Result<(), SbsError> {
    if address.len() != 6 {
        return Err(SbsError::InvalidIcaoAddress(
            format!("ICAO address must be 6 hex characters, got: {}", address)
        ));
    }
    
    for ch in address.chars() {
        if !ch.is_ascii_hexdigit() {
            return Err(SbsError::InvalidIcaoAddress(
                format!("ICAO address must contain only hex characters, got: {}", ch)
            ));
        }
    }
    
    Ok(())
}

