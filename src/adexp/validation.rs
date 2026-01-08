//! Validation sémantique des champs ADEXP selon la spécification 3.4
//! Référence: https://www.eurocontrol.int/sites/default/files/2023-06/eurocontrol-released-specification-adexp-3-4.pdf

use crate::adexp::error::AdexpError;

/// Valide un champ ADEXP selon son type et sa valeur
pub fn validate_field(field_name: &str, value: &str) -> Result<(), AdexpError> {
    match field_name {
        // Dates
        "EOBD" | "EDA" | "DATE" => validate_date(value),
        
        // Timestamps
        "EOBT" | "ETO" | "ATOT" | "ETA" | "ACTARR" | "ACTDEP" | "ATD" | "ATAD" | "ATOD" | "ATOA" | 
        "ATOTD" | "ATOTA" | "TIMEHHMM" | "TIME" => validate_time_hhmm(value),
        
        "TIMEHHMMSS" => validate_time_hhmmss(value),
        
        // Aérodromes (codes ICAO)
        "ADEP" | "ADES" | "ALTRNT1" | "ALTRNT2" => validate_aerodrome_code(value),
        
        // Identification d'aéronef
        "ARCID" => validate_aircraft_id(value),
        
        // Niveaux de vol
        "RFL" | "CFL" | "AFL" | "TFL" | "FL" => validate_flight_level(value),
        
        // Vitesse
        "SPEED" | "GROUNDSPEED" | "TAS" | "MACH" => validate_speed(value),
        
        // Coordonnées géographiques
        "LAT" => validate_latitude(value),
        "LON" => validate_longitude(value),
        
        // Codes SSR/Mode S
        "COD" => validate_ssr_code(value),
        
        // Autres champs avec formats spécifiques
        "REG" => validate_registration(value),
        "SEL" => validate_selcal(value),
        
        _ => Ok(()), // Pas de validation spécifique pour ce champ
    }
}

/// Valide une date au format DDMMYY selon ADEXP 3.4
/// Format: 6 chiffres (DDMMYY)
pub fn validate_date(value: &str) -> Result<(), AdexpError> {
    if value.len() != 6 {
        return Err(AdexpError::InvalidDateTime(format!(
            "Date doit avoir 6 chiffres (DDMMYY), reçu: {}",
            value
        )));
    }
    
    if !value.chars().all(|c| c.is_ascii_digit()) {
        return Err(AdexpError::InvalidDateTime(format!(
            "Date doit contenir uniquement des chiffres, reçu: {}",
            value
        )));
    }
    
    let day = &value[0..2];
    let month = &value[2..4];
    let _year = &value[4..6];
    
    let day_num: u32 = day.parse().map_err(|_| {
        AdexpError::InvalidDateTime(format!("Jour invalide: {}", day))
    })?;
    
    let month_num: u32 = month.parse().map_err(|_| {
        AdexpError::InvalidDateTime(format!("Mois invalide: {}", month))
    })?;
    
    if day_num < 1 || day_num > 31 {
        return Err(AdexpError::InvalidDateTime(format!(
            "Jour doit être entre 01 et 31, reçu: {}",
            day
        )));
    }
    
    if month_num < 1 || month_num > 12 {
        return Err(AdexpError::InvalidDateTime(format!(
            "Mois doit être entre 01 et 12, reçu: {}",
            month
        )));
    }
    
    // Validation basique des jours par mois (approximative, ne gère pas les années bissextiles)
    let days_in_month = match month_num {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => 29, // Approximation
        _ => return Err(AdexpError::InvalidDateTime(format!("Mois invalide: {}", month))),
    };
    
    if day_num > days_in_month {
        return Err(AdexpError::InvalidDateTime(format!(
            "Jour {} invalide pour le mois {}",
            day, month
        )));
    }
    
    Ok(())
}

/// Valide un temps au format HHMM selon ADEXP 3.4
/// Format: 4 chiffres (HHMM), HH: 00-23, MM: 00-59
pub fn validate_time_hhmm(value: &str) -> Result<(), AdexpError> {
    if value.len() != 4 {
        return Err(AdexpError::InvalidDateTime(format!(
            "Temps doit avoir 4 chiffres (HHMM), reçu: {}",
            value
        )));
    }
    
    if !value.chars().all(|c| c.is_ascii_digit()) {
        return Err(AdexpError::InvalidDateTime(format!(
            "Temps doit contenir uniquement des chiffres, reçu: {}",
            value
        )));
    }
    
    let hour = &value[0..2];
    let minute = &value[2..4];
    
    let hour_num: u32 = hour.parse().map_err(|_| {
        AdexpError::InvalidDateTime(format!("Heure invalide: {}", hour))
    })?;
    
    let minute_num: u32 = minute.parse().map_err(|_| {
        AdexpError::InvalidDateTime(format!("Minute invalide: {}", minute))
    })?;
    
    if hour_num > 23 {
        return Err(AdexpError::InvalidDateTime(format!(
            "Heure doit être entre 00 et 23, reçu: {}",
            hour
        )));
    }
    
    if minute_num > 59 {
        return Err(AdexpError::InvalidDateTime(format!(
            "Minute doit être entre 00 et 59, reçu: {}",
            minute
        )));
    }
    
    Ok(())
}

/// Valide un temps au format HHMMSS selon ADEXP 3.4
/// Format: 6 chiffres (HHMMSS), HH: 00-23, MM: 00-59, SS: 00-59
pub fn validate_time_hhmmss(value: &str) -> Result<(), AdexpError> {
    if value.len() != 6 {
        return Err(AdexpError::InvalidDateTime(format!(
            "Temps doit avoir 6 chiffres (HHMMSS), reçu: {}",
            value
        )));
    }
    
    if !value.chars().all(|c| c.is_ascii_digit()) {
        return Err(AdexpError::InvalidDateTime(format!(
            "Temps doit contenir uniquement des chiffres, reçu: {}",
            value
        )));
    }
    
    let hour = &value[0..2];
    let minute = &value[2..4];
    let second = &value[4..6];
    
    let hour_num: u32 = hour.parse().map_err(|_| {
        AdexpError::InvalidDateTime(format!("Heure invalide: {}", hour))
    })?;
    
    let minute_num: u32 = minute.parse().map_err(|_| {
        AdexpError::InvalidDateTime(format!("Minute invalide: {}", minute))
    })?;
    
    let second_num: u32 = second.parse().map_err(|_| {
        AdexpError::InvalidDateTime(format!("Seconde invalide: {}", second))
    })?;
    
    if hour_num > 23 {
        return Err(AdexpError::InvalidDateTime(format!(
            "Heure doit être entre 00 et 23, reçu: {}",
            hour
        )));
    }
    
    if minute_num > 59 {
        return Err(AdexpError::InvalidDateTime(format!(
            "Minute doit être entre 00 et 59, reçu: {}",
            minute
        )));
    }
    
    if second_num > 59 {
        return Err(AdexpError::InvalidDateTime(format!(
            "Seconde doit être entre 00 et 59, reçu: {}",
            second
        )));
    }
    
    Ok(())
}

/// Valide un code d'aérodrome ICAO
/// Format: 4 lettres majuscules
pub fn validate_aerodrome_code(value: &str) -> Result<(), AdexpError> {
    if value.len() != 4 {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Code aérodrome doit avoir 4 caractères (ICAO), reçu: {} ({} caractères)",
            value, value.len()
        )));
    }
    
    if !value.chars().all(|c| c.is_ascii_alphabetic() && c.is_uppercase()) {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Code aérodrome doit contenir uniquement des lettres majuscules, reçu: {}",
            value
        )));
    }
    
    Ok(())
}

/// Valide un identifiant d'aéronef (ARCID)
/// Format: 1-7 caractères alphanumériques, commençant par une lettre
pub fn validate_aircraft_id(value: &str) -> Result<(), AdexpError> {
    if value.is_empty() {
        return Err(AdexpError::InvalidFieldValue(
            "Identifiant d'aéronef ne peut pas être vide".to_string()
        ));
    }
    
    if value.len() > 7 {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Identifiant d'aéronef ne peut pas dépasser 7 caractères, reçu: {} ({} caractères)",
            value, value.len()
        )));
    }
    
    if !value.chars().next().unwrap().is_ascii_alphabetic() {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Identifiant d'aéronef doit commencer par une lettre, reçu: {}",
            value
        )));
    }
    
    if !value.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Identifiant d'aéronef doit contenir uniquement des caractères alphanumériques, reçu: {}",
            value
        )));
    }
    
    Ok(())
}

/// Valide un niveau de vol (Flight Level)
/// Format: FL suivi de 3 chiffres (000-999) ou niveau en centaines de pieds
pub fn validate_flight_level(value: &str) -> Result<(), AdexpError> {
    // Format FLXXX (ex: FL350) ou XXX (ex: 350)
    let numeric_part = if value.starts_with("FL") {
        &value[2..]
    } else {
        value
    };
    
    if numeric_part.len() != 3 {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Niveau de vol doit avoir 3 chiffres (000-999), reçu: {}",
            value
        )));
    }
    
    if !numeric_part.chars().all(|c| c.is_ascii_digit()) {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Niveau de vol doit contenir uniquement des chiffres, reçu: {}",
            value
        )));
    }
    
    let level: u32 = numeric_part.parse().map_err(|_| {
        AdexpError::InvalidFieldValue(format!("Niveau de vol invalide: {}", value))
    })?;
    
    if level > 999 {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Niveau de vol doit être entre 000 et 999, reçu: {}",
            value
        )));
    }
    
    Ok(())
}

/// Valide une vitesse
/// Format: nombre avec unité optionnelle (KTS, MACH, etc.) ou format spécifique
pub fn validate_speed(value: &str) -> Result<(), AdexpError> {
    if value.is_empty() {
        return Err(AdexpError::InvalidFieldValue(
            "Vitesse ne peut pas être vide".to_string()
        ));
    }
    
    // Format MACH: M suivi de 3 chiffres (ex: M082)
    if value.starts_with('M') && value.len() == 4 {
        let mach_part = &value[1..];
        if mach_part.chars().all(|c| c.is_ascii_digit()) {
            let mach: u32 = mach_part.parse().map_err(|_| {
                AdexpError::InvalidFieldValue(format!("MACH invalide: {}", value))
            })?;
            if mach > 999 {
                return Err(AdexpError::InvalidFieldValue(format!(
                    "MACH doit être entre 000 et 999, reçu: {}",
                    value
                )));
            }
            return Ok(());
        }
    }
    
    // Format MACH avec plus de 3 chiffres après M (invalide)
    if value.starts_with('M') && value.len() > 4 {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Format MACH invalide (M suivi de 3 chiffres max), reçu: {}",
            value
        )));
    }
    
    // Format numérique simple (vitesse en nœuds)
    if value.chars().all(|c| c.is_ascii_digit()) {
        let speed: u32 = value.parse().map_err(|_| {
            AdexpError::InvalidFieldValue(format!("Vitesse invalide: {}", value))
        })?;
        if speed > 9999 {
            return Err(AdexpError::InvalidFieldValue(format!(
                "Vitesse doit être raisonnable (< 10000), reçu: {}",
                value
            )));
        }
        return Ok(());
    }
    
    // Format avec unité (ex: 450KTS, 0.82MACH)
    // Validation basique - accepter si contient des chiffres
    if value.chars().any(|c| c.is_ascii_digit()) {
        return Ok(());
    }
    
    Err(AdexpError::InvalidFieldValue(format!(
        "Format de vitesse invalide: {}",
        value
    )))
}

/// Valide une latitude
/// Format: +/-DDMMSS ou +/-DDMM.mm ou +/-DD.dd
pub fn validate_latitude(value: &str) -> Result<(), AdexpError> {
    if value.is_empty() {
        return Err(AdexpError::InvalidFieldValue(
            "Latitude ne peut pas être vide".to_string()
        ));
    }
    
    // Format simple: nombre avec signe optionnel
    let numeric_value = value.trim_start_matches(['+', '-']);
    
    if numeric_value.is_empty() {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Latitude invalide: {}",
            value
        )));
    }
    
    // Vérifier que c'est un nombre valide (peut contenir un point décimal)
    let parts: Vec<&str> = numeric_value.split('.').collect();
    if parts.len() > 2 {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Latitude invalide (trop de points décimaux): {}",
            value
        )));
    }
    
    for part in &parts {
        if !part.chars().all(|c| c.is_ascii_digit()) {
            return Err(AdexpError::InvalidFieldValue(format!(
                "Latitude doit contenir uniquement des chiffres et un point décimal, reçu: {}",
                value
            )));
        }
    }
    
    // Validation de la plage: -90 à +90
    if let Ok(lat) = value.parse::<f64>() {
        if lat < -90.0 || lat > 90.0 {
            return Err(AdexpError::InvalidFieldValue(format!(
                "Latitude doit être entre -90 et +90, reçu: {}",
                value
            )));
        }
    }
    
    Ok(())
}

/// Valide une longitude
/// Format: +/-DDDMMSS ou +/-DDDMM.mm ou +/-DDD.dd
pub fn validate_longitude(value: &str) -> Result<(), AdexpError> {
    if value.is_empty() {
        return Err(AdexpError::InvalidFieldValue(
            "Longitude ne peut pas être vide".to_string()
        ));
    }
    
    // Format simple: nombre avec signe optionnel
    let numeric_value = value.trim_start_matches(['+', '-']);
    
    if numeric_value.is_empty() {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Longitude invalide: {}",
            value
        )));
    }
    
    // Vérifier que c'est un nombre valide (peut contenir un point décimal)
    let parts: Vec<&str> = numeric_value.split('.').collect();
    if parts.len() > 2 {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Longitude invalide (trop de points décimaux): {}",
            value
        )));
    }
    
    for part in &parts {
        if !part.chars().all(|c| c.is_ascii_digit()) {
            return Err(AdexpError::InvalidFieldValue(format!(
                "Longitude doit contenir uniquement des chiffres et un point décimal, reçu: {}",
                value
            )));
        }
    }
    
    // Validation de la plage: -180 à +180
    if let Ok(lon) = value.parse::<f64>() {
        if lon < -180.0 || lon > 180.0 {
            return Err(AdexpError::InvalidFieldValue(format!(
                "Longitude doit être entre -180 et +180, reçu: {}",
                value
            )));
        }
    }
    
    Ok(())
}

/// Valide un code SSR (Secondary Surveillance Radar)
/// Format: 4 chiffres (0000-7777 en octal) ou format Mode S
pub fn validate_ssr_code(value: &str) -> Result<(), AdexpError> {
    if value.is_empty() {
        return Err(AdexpError::InvalidFieldValue(
            "Code SSR ne peut pas être vide".to_string()
        ));
    }
    
    // Format 4 chiffres (octal: 0-7)
    if value.len() == 4 && value.chars().all(|c| c.is_ascii_digit()) {
        for c in value.chars() {
            let digit: u32 = c.to_digit(10).unwrap();
            if digit > 7 {
                return Err(AdexpError::InvalidFieldValue(format!(
                    "Code SSR doit être en octal (chiffres 0-7), reçu: {}",
                    value
                )));
            }
        }
        return Ok(());
    }
    
    // Format Mode S (6 caractères hexadécimaux)
    if value.len() == 6 && value.chars().all(|c| c.is_ascii_hexdigit()) {
        return Ok(());
    }
    
    Err(AdexpError::InvalidFieldValue(format!(
        "Code SSR invalide (format attendu: 4 chiffres octal ou 6 hexadécimaux), reçu: {}",
        value
    )))
}

/// Valide un numéro d'immatriculation d'aéronef
/// Format: variable selon le pays, généralement lettres et chiffres
pub fn validate_registration(value: &str) -> Result<(), AdexpError> {
    if value.is_empty() {
        return Err(AdexpError::InvalidFieldValue(
            "Immatriculation ne peut pas être vide".to_string()
        ));
    }
    
    if value.len() > 10 {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Immatriculation ne peut pas dépasser 10 caractères, reçu: {} ({} caractères)",
            value, value.len()
        )));
    }
    
    // Format général: lettres et chiffres, au moins une lettre
    if !value.chars().any(|c| c.is_ascii_alphabetic()) {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Immatriculation doit contenir au moins une lettre, reçu: {}",
            value
        )));
    }
    
    if !value.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Immatriculation doit contenir uniquement des caractères alphanumériques et tirets, reçu: {}",
            value
        )));
    }
    
    Ok(())
}

/// Valide un code SELCAL
/// Format: 4 lettres (A-S, sauf I, N, O, Q)
pub fn validate_selcal(value: &str) -> Result<(), AdexpError> {
    if value.len() != 4 {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Code SELCAL doit avoir 4 lettres, reçu: {} ({} caractères)",
            value, value.len()
        )));
    }
    
    let valid_letters = "ABCDEFGHJKLM PRSTUVWXYZ"; // A-S sauf I, N, O, Q
    
    for c in value.chars() {
        if !c.is_ascii_alphabetic() || !valid_letters.contains(c) {
            return Err(AdexpError::InvalidFieldValue(format!(
                "Code SELCAL doit contenir uniquement les lettres A-S (sauf I, N, O, Q), reçu: {}",
                value
            )));
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_date() {
        assert!(validate_date("010120").is_ok());
        assert!(validate_date("311231").is_ok());
        assert!(validate_date("290229").is_ok()); // Année bissextile approximative
        assert!(validate_date("12345").is_err()); // Trop court
        assert!(validate_date("1234567").is_err()); // Trop long
        assert!(validate_date("320120").is_err()); // Jour invalide
        assert!(validate_date("131320").is_err()); // Mois invalide
    }

    #[test]
    fn test_validate_time_hhmm() {
        assert!(validate_time_hhmm("0000").is_ok());
        assert!(validate_time_hhmm("2359").is_ok());
        assert!(validate_time_hhmm("1230").is_ok());
        assert!(validate_time_hhmm("123").is_err()); // Trop court
        assert!(validate_time_hhmm("12345").is_err()); // Trop long
        assert!(validate_time_hhmm("2400").is_err()); // Heure invalide
        assert!(validate_time_hhmm("1260").is_err()); // Minute invalide
    }

    #[test]
    fn test_validate_time_hhmmss() {
        assert!(validate_time_hhmmss("000000").is_ok());
        assert!(validate_time_hhmmss("235959").is_ok());
        assert!(validate_time_hhmmss("123045").is_ok());
        assert!(validate_time_hhmmss("12345").is_err()); // Trop court
        assert!(validate_time_hhmmss("1234567").is_err()); // Trop long
        assert!(validate_time_hhmmss("240000").is_err()); // Heure invalide
        assert!(validate_time_hhmmss("126000").is_err()); // Minute invalide
        assert!(validate_time_hhmmss("123060").is_err()); // Seconde invalide
    }

    #[test]
    fn test_validate_aerodrome_code() {
        assert!(validate_aerodrome_code("LFPG").is_ok());
        assert!(validate_aerodrome_code("KJFK").is_ok());
        assert!(validate_aerodrome_code("EGLL").is_ok());
        assert!(validate_aerodrome_code("LFP").is_err()); // Trop court
        assert!(validate_aerodrome_code("LFPGG").is_err()); // Trop long
        assert!(validate_aerodrome_code("lfpg").is_err()); // Minuscules
        assert!(validate_aerodrome_code("LF1G").is_err()); // Chiffre
    }

    #[test]
    fn test_validate_aircraft_id() {
        assert!(validate_aircraft_id("ABC123").is_ok());
        assert!(validate_aircraft_id("A").is_ok());
        assert!(validate_aircraft_id("ABCDEFG").is_ok()); // 7 caractères max
        assert!(validate_aircraft_id("").is_err()); // Vide
        assert!(validate_aircraft_id("ABCDEFGH").is_err()); // Trop long
        assert!(validate_aircraft_id("123ABC").is_err()); // Commence par chiffre
        assert!(validate_aircraft_id("AB-C123").is_err()); // Caractère spécial
    }

    #[test]
    fn test_validate_flight_level() {
        assert!(validate_flight_level("FL350").is_ok());
        assert!(validate_flight_level("350").is_ok());
        assert!(validate_flight_level("000").is_ok());
        assert!(validate_flight_level("999").is_ok());
        assert!(validate_flight_level("FL35").is_err()); // Trop court
        assert!(validate_flight_level("FL3500").is_err()); // Trop long
        assert!(validate_flight_level("FL35A").is_err()); // Lettre
        assert!(validate_flight_level("1000").is_err()); // Trop élevé
    }

    #[test]
    fn test_validate_speed() {
        assert!(validate_speed("450").is_ok());
        assert!(validate_speed("M082").is_ok());
        assert!(validate_speed("450KTS").is_ok());
        assert!(validate_speed("").is_err()); // Vide
        assert!(validate_speed("M9999").is_err()); // MACH trop élevé
    }

    #[test]
    fn test_validate_latitude() {
        assert!(validate_latitude("48.8566").is_ok());
        assert!(validate_latitude("-48.8566").is_ok());
        assert!(validate_latitude("90").is_ok());
        assert!(validate_latitude("-90").is_ok());
        assert!(validate_latitude("91").is_err()); // Trop élevé
        assert!(validate_latitude("-91").is_err()); // Trop bas
        assert!(validate_latitude("").is_err()); // Vide
    }

    #[test]
    fn test_validate_longitude() {
        assert!(validate_longitude("2.3522").is_ok());
        assert!(validate_longitude("-2.3522").is_ok());
        assert!(validate_longitude("180").is_ok());
        assert!(validate_longitude("-180").is_ok());
        assert!(validate_longitude("181").is_err()); // Trop élevé
        assert!(validate_longitude("-181").is_err()); // Trop bas
        assert!(validate_longitude("").is_err()); // Vide
    }

    #[test]
    fn test_validate_ssr_code() {
        assert!(validate_ssr_code("1234").is_ok());
        assert!(validate_ssr_code("0000").is_ok());
        assert!(validate_ssr_code("7777").is_ok());
        assert!(validate_ssr_code("ABCDEF").is_ok()); // Mode S hex
        assert!(validate_ssr_code("123").is_err()); // Trop court
        assert!(validate_ssr_code("12345").is_err()); // Trop long
        assert!(validate_ssr_code("1238").is_err()); // Chiffre > 7
        assert!(validate_ssr_code("").is_err()); // Vide
    }

    #[test]
    fn test_validate_registration() {
        assert!(validate_registration("F-ABCD").is_ok());
        assert!(validate_registration("N12345").is_ok());
        assert!(validate_registration("G-ABCDEFGH").is_ok());
        assert!(validate_registration("").is_err()); // Vide
        assert!(validate_registration("12345678901").is_err()); // Trop long
        assert!(validate_registration("12345").is_err()); // Pas de lettre
    }

    #[test]
    fn test_validate_selcal() {
        assert!(validate_selcal("ABCD").is_ok());
        assert!(validate_selcal("JKLM").is_ok());
        assert!(validate_selcal("ABC").is_err()); // Trop court
        assert!(validate_selcal("ABCDE").is_err()); // Trop long
        assert!(validate_selcal("ABCI").is_err()); // I interdit
        assert!(validate_selcal("ABCN").is_err()); // N interdit
    }
}

