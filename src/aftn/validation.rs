//! Validation sémantique des champs AFTN selon la spécification 3.4
//! Référence: AFTN 3.4 specification

use crate::aftn::error::AftnError;

/// Valide un callsign selon la grammaire AFTN 3.4
/// Format: 1-7 caractères alphanumériques, doit commencer par une lettre
pub fn validate_callsign(callsign: &str) -> Result<(), AftnError> {
    if callsign.is_empty() {
        return Err(AftnError::InvalidFormat("Callsign cannot be empty".to_string()));
    }
    
    if callsign.len() > 7 {
        return Err(AftnError::InvalidFormat(format!(
            "Callsign must be 1-7 characters, got {} characters",
            callsign.len()
        )));
    }
    
    // Le callsign doit commencer par une lettre
    // PANIC: callsign.chars().next().unwrap() peut panic si callsign est vide,
    // mais cela est impossible ici car on vérifie is_empty() juste avant (ligne 9)
    let first_char = callsign.chars().next().unwrap();
    if !first_char.is_ascii_alphabetic() {
        return Err(AftnError::InvalidFormat(format!(
            "Callsign must start with a letter, got: {}",
            callsign
        )));
    }
    
    // Le callsign ne peut contenir que des lettres et des chiffres
    if !callsign.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(AftnError::InvalidFormat(format!(
            "Callsign can only contain letters and digits, got: {}",
            callsign
        )));
    }
    
    Ok(())
}

/// Valide un code aérodrome ICAO
/// Format: 4 lettres majuscules
pub fn validate_aerodrome_code(code: &str) -> Result<(), AftnError> {
    if code.len() != 4 {
        return Err(AftnError::InvalidFormat(format!(
            "Aerodrome code must be 4 characters, got {} characters",
            code.len()
        )));
    }
    
    if !code.chars().all(|c| c.is_ascii_alphabetic() && c.is_uppercase()) {
        return Err(AftnError::InvalidFormat(format!(
            "Aerodrome code must be 4 uppercase letters, got: {}",
            code
        )));
    }
    
    Ok(())
}

/// Valide un temps au format HHMM
/// Format: 4 chiffres, heure 00-23, minute 00-59
pub fn validate_time_hhmm(time: &str) -> Result<(), AftnError> {
    if time.len() != 4 {
        return Err(AftnError::InvalidFormat(format!(
            "Time must be 4 digits (HHMM), got {} characters",
            time.len()
        )));
    }
    
    if !time.chars().all(|c| c.is_ascii_digit()) {
        return Err(AftnError::InvalidFormat(format!(
            "Time must contain only digits, got: {}",
            time
        )));
    }
    
    // PANIC: time[0..2] et time[2..4] peuvent panic si time.len() < 4,
    // mais cela est impossible ici car on vérifie time.len() != 4 juste avant (ligne 63)
    let hour: u32 = time[0..2].parse().map_err(|_| {
        AftnError::InvalidFormat(format!("Invalid hour in time: {}", time))
    })?;
    
    let minute: u32 = time[2..4].parse().map_err(|_| {
        AftnError::InvalidFormat(format!("Invalid minute in time: {}", time))
    })?;
    
    if hour > 23 {
        return Err(AftnError::InvalidFormat(format!(
            "Hour must be between 00 and 23, got: {}",
            hour
        )));
    }
    
    if minute > 59 {
        return Err(AftnError::InvalidFormat(format!(
            "Minute must be between 00 and 59, got: {}",
            minute
        )));
    }
    
    Ok(())
}

/// Valide un niveau de vol
/// Format: F### (ex: F350, F045, F999)
pub fn validate_flight_level(level: &str) -> Result<(), AftnError> {
    if level.is_empty() {
        return Err(AftnError::InvalidFormat("Flight level cannot be empty".to_string()));
    }
    
    if !level.starts_with('F') && !level.starts_with('A') {
        return Err(AftnError::InvalidFormat(format!(
            "Flight level must start with F or A, got: {}",
            level
        )));
    }
    
    // PANIC: level[1..] peut panic si level est vide,
    // mais cela est impossible ici car on vérifie is_empty() juste avant (ligne 105)
    // et on vérifie que level commence par 'F' ou 'A' juste avant (ligne 109)
    let numeric_part = &level[1..];
    if numeric_part.len() != 3 {
        return Err(AftnError::InvalidFormat(format!(
            "Flight level must have 3 digits after F/A, got: {}",
            level
        )));
    }
    
    if !numeric_part.chars().all(|c| c.is_ascii_digit()) {
        return Err(AftnError::InvalidFormat(format!(
            "Flight level must contain only digits after F/A, got: {}",
            level
        )));
    }
    
    let level_num: u32 = numeric_part.parse().map_err(|_| {
        AftnError::InvalidFormat(format!("Invalid flight level number: {}", level))
    })?;
    
    if level_num > 999 {
        return Err(AftnError::InvalidFormat(format!(
            "Flight level must be between 000 and 999, got: {}",
            level_num
        )));
    }
    
    Ok(())
}

/// Valide un type de vol
/// Format: V (VFR), I (IFR), Y (YFR), Z (ZFR)
pub fn validate_flight_type(flight_type: &str) -> Result<(), AftnError> {
    let valid_types = ["V", "I", "Y", "Z"];
    if !valid_types.contains(&flight_type) {
        return Err(AftnError::InvalidFormat(format!(
            "Flight type must be one of {:?}, got: {}",
            valid_types, flight_type
        )));
    }
    Ok(())
}

/// Valide un point de route (waypoint)
/// Format: 2-5 caractères alphanumériques
pub fn validate_waypoint(waypoint: &str) -> Result<(), AftnError> {
    if waypoint.len() < 2 || waypoint.len() > 5 {
        return Err(AftnError::InvalidFormat(format!(
            "Waypoint must be 2-5 characters, got {} characters",
            waypoint.len()
        )));
    }
    
    if !waypoint.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(AftnError::InvalidFormat(format!(
            "Waypoint must contain only alphanumeric characters, got: {}",
            waypoint
        )));
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_callsign() {
        assert!(validate_callsign("ABC123").is_ok());
        assert!(validate_callsign("A").is_ok());
        assert!(validate_callsign("ABCDEFG").is_ok()); // 7 caractères max
        assert!(validate_callsign("").is_err());
        assert!(validate_callsign("ABCDEFGH").is_err()); // Trop long
        assert!(validate_callsign("123ABC").is_err()); // Commence par chiffre
        assert!(validate_callsign("AB-C123").is_err()); // Caractère spécial
    }

    #[test]
    fn test_validate_aerodrome_code() {
        assert!(validate_aerodrome_code("LFPG").is_ok());
        assert!(validate_aerodrome_code("EDDF").is_ok());
        assert!(validate_aerodrome_code("LFPO").is_ok());
        assert!(validate_aerodrome_code("LFP").is_err()); // Trop court
        assert!(validate_aerodrome_code("LFPGB").is_err()); // Trop long
        assert!(validate_aerodrome_code("lfpg").is_err()); // Minuscules
        assert!(validate_aerodrome_code("LF1G").is_err()); // Chiffre
    }

    #[test]
    fn test_validate_time_hhmm() {
        assert!(validate_time_hhmm("1200").is_ok());
        assert!(validate_time_hhmm("0000").is_ok());
        assert!(validate_time_hhmm("2359").is_ok());
        assert!(validate_time_hhmm("120").is_err()); // Trop court
        assert!(validate_time_hhmm("12000").is_err()); // Trop long
        assert!(validate_time_hhmm("2400").is_err()); // Heure invalide
        assert!(validate_time_hhmm("1260").is_err()); // Minute invalide
        assert!(validate_time_hhmm("12AB").is_err()); // Non numérique
    }

    #[test]
    fn test_validate_flight_level() {
        assert!(validate_flight_level("F350").is_ok());
        assert!(validate_flight_level("F045").is_ok());
        assert!(validate_flight_level("F999").is_ok());
        assert!(validate_flight_level("A350").is_ok());
        assert!(validate_flight_level("F35").is_err()); // Trop court
        assert!(validate_flight_level("F3500").is_err()); // Trop long
        assert!(validate_flight_level("350").is_err()); // Pas de préfixe
        assert!(validate_flight_level("F35A").is_err()); // Non numérique
    }

    #[test]
    fn test_validate_flight_type() {
        assert!(validate_flight_type("V").is_ok());
        assert!(validate_flight_type("I").is_ok());
        assert!(validate_flight_type("Y").is_ok());
        assert!(validate_flight_type("Z").is_ok());
        assert!(validate_flight_type("X").is_err());
        assert!(validate_flight_type("VI").is_err());
    }

    #[test]
    fn test_validate_waypoint() {
        assert!(validate_waypoint("AB").is_ok());
        assert!(validate_waypoint("ABCDE").is_ok());
        assert!(validate_waypoint("RID").is_ok());
        assert!(validate_waypoint("A").is_err()); // Trop court
        assert!(validate_waypoint("ABCDEF").is_err()); // Trop long
        assert!(validate_waypoint("AB-CD").is_err()); // Caractère spécial
    }
}

