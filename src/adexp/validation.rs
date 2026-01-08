//! Validation sémantique des champs ADEXP selon la spécification 3.4
//! Référence: https://www.eurocontrol.int/sites/default/files/2023-06/eurocontrol-released-specification-adexp-3-4.pdf

use crate::adexp::error::AdexpError;
use crate::adexp::message::Section;

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
        
        // Flight Rules et Types
        "FLTRUL" => validate_flight_rules(value),
        "FLTTYP" => validate_flight_type(value),
        
        // Equipment codes
        "PBN" => validate_pbn(value),
        "NAV" => validate_nav_equipment(value),
        "COM" => validate_com_equipment(value),
        "DAT" => validate_dat_equipment(value),
        "SUR" => validate_sur_equipment(value),
        "CEQPT" => validate_ceqpt(value),
        
        // Aircraft Type
        "ARCTYP" => validate_aircraft_type(value),
        
        // Météorologie
        "WINDIR" => validate_wind_direction(value),
        "WINDSPEED" => validate_wind_speed(value),
        "QNH" | "QFE" => validate_pressure(value),
        "AIRTEMP" => validate_temperature(value),
        
        // Temps et angles
        "EET" => validate_time_hhmm(value),
        "TRACKANGLE" => validate_track_angle(value),
        
        // Altitude et distance
        "ALT" | "ALTNZ" => validate_altitude(value),
        "DIST" | "RELDIST" => validate_distance(value),
        
        // Adresses et codes
        "HEXADDR" => validate_hex_address(value),
        "OPRICAO" | "PERICAO" => validate_icao_3letter_code(value),
        "CODEICAO" => validate_code_icao(value),
        
        // Wake Turbulence
        "WKTRC" => validate_wake_turbulence(value),
        
        // IFPS
        "IFPLID" => validate_ifplid(value),
        
        // Procedures
        "SID" | "STAR" | "ARRPROC" | "DEPPROC" => validate_procedure(value),
        
        // Champs composés - validation de structure uniquement (les sous-champs sont validés séparément)
        "ROUTE" => validate_route_structure(value),
        
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
    
    // PANIC: value.chars().next().unwrap() peut panic si value est vide,
    // mais cela est impossible ici car on vérifie value.is_empty() juste avant (ligne 278)
    // et value.len() > 7 juste après (ligne 285)
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
    // PANIC: c.to_digit(10).unwrap() peut panic si c n'est pas un chiffre ASCII,
    // mais cela est impossible ici car on vérifie c.is_ascii_digit() dans le .all() juste avant
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

/// Valide les règles de vol (Flight Rules)
/// Valeurs valides: I (IFR), V (VFR), Y (YFR), Z (ZFR)
pub fn validate_flight_rules(value: &str) -> Result<(), AdexpError> {
    match value {
        "I" | "V" | "Y" | "Z" => Ok(()),
        _ => Err(AdexpError::InvalidFieldValue(format!(
            "Flight Rules doit être I, V, Y ou Z, reçu: {}",
            value
        ))),
    }
}

/// Valide le type de vol (Flight Type)
/// Valeurs valides: S (Scheduled), N (Non-scheduled), G (General Aviation), M (Military), X (Other)
pub fn validate_flight_type(value: &str) -> Result<(), AdexpError> {
    match value {
        "S" | "N" | "G" | "M" | "X" => Ok(()),
        _ => Err(AdexpError::InvalidFieldValue(format!(
            "Flight Type doit être S, N, G, M ou X, reçu: {}",
            value
        ))),
    }
}

/// Valide PBN (Performance Based Navigation)
/// Format: codes séparés par virgule (ex: A1, B1, C1, D1, L1, O1, S1, T1)
pub fn validate_pbn(value: &str) -> Result<(), AdexpError> {
    if value.is_empty() {
        return Err(AdexpError::InvalidFieldValue(
            "PBN ne peut pas être vide".to_string()
        ));
    }
    
    // Format: codes séparés par virgule ou espace
    let codes: Vec<&str> = value.split([',', ' ']).filter(|s| !s.is_empty()).collect();
    
    if codes.is_empty() {
        return Err(AdexpError::InvalidFieldValue(
            "PBN doit contenir au moins un code".to_string()
        ));
    }
    
    // Codes PBN valides selon ICAO: A1, B1, B2, B3, B4, B5, B6, C1, C2, C3, C4, D1, D2, D3, D4, L1, O1, O2, O3, O4, S1, S2, T1, T2
    let valid_codes = ["A1", "B1", "B2", "B3", "B4", "B5", "B6", "C1", "C2", "C3", "C4", 
                       "D1", "D2", "D3", "D4", "L1", "O1", "O2", "O3", "O4", "S1", "S2", "T1", "T2"];
    
    for code in codes {
        if !valid_codes.contains(&code) {
            return Err(AdexpError::InvalidFieldValue(format!(
                "Code PBN invalide: {} (codes valides: A1, B1-B6, C1-C4, D1-D4, L1, O1-O4, S1-S2, T1-T2)",
                code
            )));
        }
    }
    
    Ok(())
}

/// Valide l'équipement de navigation (NAV)
/// Format: codes séparés par virgule (ex: A, B, C, D, E1, E2, E3, F, G1, G2, H, I, J1, J2, J3, J4, J5, J6, J7, K, L, M1, M2, M3, O1, O2, O3, O4, P1, P2, P3, P4, P5, P6, P7, P8, P9, R, T, U, V, W, X, Y, Z)
pub fn validate_nav_equipment(value: &str) -> Result<(), AdexpError> {
    if value.is_empty() {
        return Ok(()); // NAV peut être vide
    }
    
    // Codes NAV valides selon ICAO
    let valid_codes = ["A", "B", "C", "D", "E1", "E2", "E3", "F", "G1", "G2", "H", "I", 
                       "J1", "J2", "J3", "J4", "J5", "J6", "J7", "K", "L", "M1", "M2", "M3",
                       "O1", "O2", "O3", "O4", "P1", "P2", "P3", "P4", "P5", "P6", "P7", "P8", "P9",
                       "R", "T", "U", "V", "W", "X", "Y", "Z"];
    
    let codes: Vec<&str> = value.split([',', ' ']).filter(|s| !s.is_empty()).collect();
    
    for code in codes {
        if !valid_codes.contains(&code) {
            return Err(AdexpError::InvalidFieldValue(format!(
                "Code NAV invalide: {}",
                code
            )));
        }
    }
    
    Ok(())
}

/// Valide l'équipement de communication (COM)
/// Format: codes séparés par virgule
pub fn validate_com_equipment(value: &str) -> Result<(), AdexpError> {
    if value.is_empty() {
        return Ok(()); // COM peut être vide
    }
    
    // Codes COM valides selon ICAO
    let valid_codes = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];
    
    let codes: Vec<&str> = value.split([',', ' ']).filter(|s| !s.is_empty()).collect();
    
    for code in codes {
        if !valid_codes.contains(&code) {
            return Err(AdexpError::InvalidFieldValue(format!(
                "Code COM invalide: {}",
                code
            )));
        }
    }
    
    Ok(())
}

/// Valide l'équipement de liaison de données (DAT)
/// Format: codes séparés par virgule
pub fn validate_dat_equipment(value: &str) -> Result<(), AdexpError> {
    if value.is_empty() {
        return Ok(()); // DAT peut être vide
    }
    
    // Codes DAT valides selon ICAO
    let valid_codes = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];
    
    let codes: Vec<&str> = value.split([',', ' ']).filter(|s| !s.is_empty()).collect();
    
    for code in codes {
        if !valid_codes.contains(&code) {
            return Err(AdexpError::InvalidFieldValue(format!(
                "Code DAT invalide: {}",
                code
            )));
        }
    }
    
    Ok(())
}

/// Valide l'équipement de surveillance (SUR)
/// Format: codes séparés par virgule
pub fn validate_sur_equipment(value: &str) -> Result<(), AdexpError> {
    if value.is_empty() {
        return Ok(()); // SUR peut être vide
    }
    
    // Codes SUR valides selon ICAO
    let valid_codes = ["A", "B1", "B2", "C", "D1", "D2", "E1", "E2", "E3", "E4", "E5", 
                       "F", "G", "H", "I", "J1", "J2", "J3", "J4", "J5", "J6", "J7", 
                       "K", "L", "M1", "M2", "M3", "N", "P1", "P2", "P3", "P4", "P5", 
                       "P6", "P7", "P8", "P9", "Q", "R", "S", "T", "U1", "U2", "U3", 
                       "U4", "V1", "V2", "W", "X", "Y", "Z"];
    
    let codes: Vec<&str> = value.split([',', ' ']).filter(|s| !s.is_empty()).collect();
    
    for code in codes {
        if !valid_codes.contains(&code) {
            return Err(AdexpError::InvalidFieldValue(format!(
                "Code SUR invalide: {}",
                code
            )));
        }
    }
    
    Ok(())
}

/// Valide CEQPT (Communication Equipment)
/// Format: codes séparés par virgule (similaire à COM)
pub fn validate_ceqpt(value: &str) -> Result<(), AdexpError> {
    validate_com_equipment(value)
}

/// Valide le type d'aéronef (ARCTYP)
/// Format: code ICAO (2-4 caractères alphanumériques)
pub fn validate_aircraft_type(value: &str) -> Result<(), AdexpError> {
    if value.is_empty() {
        return Err(AdexpError::InvalidFieldValue(
            "Type d'aéronef ne peut pas être vide".to_string()
        ));
    }
    
    if value.len() < 2 || value.len() > 4 {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Type d'aéronef doit avoir 2-4 caractères (code ICAO), reçu: {} ({} caractères)",
            value, value.len()
        )));
    }
    
    if !value.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Type d'aéronef doit contenir uniquement des caractères alphanumériques, reçu: {}",
            value
        )));
    }
    
    Ok(())
}

/// Valide la direction du vent (WINDIR)
/// Format: 000-360 (3 chiffres)
pub fn validate_wind_direction(value: &str) -> Result<(), AdexpError> {
    if value.len() != 3 {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Direction du vent doit avoir 3 chiffres (000-360), reçu: {}",
            value
        )));
    }
    
    if !value.chars().all(|c| c.is_ascii_digit()) {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Direction du vent doit contenir uniquement des chiffres, reçu: {}",
            value
        )));
    }
    
    let dir: u32 = value.parse().map_err(|_| {
        AdexpError::InvalidFieldValue(format!("Direction du vent invalide: {}", value))
    })?;
    
    if dir > 360 {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Direction du vent doit être entre 000 et 360, reçu: {}",
            value
        )));
    }
    
    Ok(())
}

/// Valide la vitesse du vent (WINDSPEED)
/// Format: nombre (nœuds)
pub fn validate_wind_speed(value: &str) -> Result<(), AdexpError> {
    if value.is_empty() {
        return Err(AdexpError::InvalidFieldValue(
            "Vitesse du vent ne peut pas être vide".to_string()
        ));
    }
    
    if !value.chars().all(|c| c.is_ascii_digit()) {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Vitesse du vent doit contenir uniquement des chiffres, reçu: {}",
            value
        )));
    }
    
    let speed: u32 = value.parse().map_err(|_| {
        AdexpError::InvalidFieldValue(format!("Vitesse du vent invalide: {}", value))
    })?;
    
    if speed > 999 {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Vitesse du vent doit être raisonnable (< 1000 nœuds), reçu: {}",
            value
        )));
    }
    
    Ok(())
}

/// Valide la pression (QNH, QFE)
/// Format: nombre (hPa, généralement 3-4 chiffres)
pub fn validate_pressure(value: &str) -> Result<(), AdexpError> {
    if value.is_empty() {
        return Err(AdexpError::InvalidFieldValue(
            "Pression ne peut pas être vide".to_string()
        ));
    }
    
    if !value.chars().all(|c| c.is_ascii_digit()) {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Pression doit contenir uniquement des chiffres, reçu: {}",
            value
        )));
    }
    
    let pressure: u32 = value.parse().map_err(|_| {
        AdexpError::InvalidFieldValue(format!("Pression invalide: {}", value))
    })?;
    
    // Plage raisonnable pour la pression: 800-1100 hPa
    if pressure < 800 || pressure > 1100 {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Pression doit être entre 800 et 1100 hPa, reçu: {}",
            value
        )));
    }
    
    Ok(())
}

/// Valide la température de l'air (AIRTEMP)
/// Format: nombre avec signe optionnel (degrés Celsius)
pub fn validate_temperature(value: &str) -> Result<(), AdexpError> {
    if value.is_empty() {
        return Err(AdexpError::InvalidFieldValue(
            "Température ne peut pas être vide".to_string()
        ));
    }
    
    let numeric_value = value.trim_start_matches(['+', '-']);
    
    if numeric_value.is_empty() {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Température invalide: {}",
            value
        )));
    }
    
    if !numeric_value.chars().all(|c| c.is_ascii_digit()) {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Température doit contenir uniquement des chiffres et un signe optionnel, reçu: {}",
            value
        )));
    }
    
    if let Ok(temp) = value.parse::<i32>() {
        // Plage raisonnable: -80 à +60 degrés Celsius
        if temp < -80 || temp > 60 {
            return Err(AdexpError::InvalidFieldValue(format!(
                "Température doit être entre -80 et +60°C, reçu: {}",
                value
            )));
        }
    }
    
    Ok(())
}

/// Valide l'angle de route (TRACKANGLE)
/// Format: 001-360 (3 chiffres)
pub fn validate_track_angle(value: &str) -> Result<(), AdexpError> {
    if value.len() != 3 {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Angle de route doit avoir 3 chiffres (001-360), reçu: {}",
            value
        )));
    }
    
    if !value.chars().all(|c| c.is_ascii_digit()) {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Angle de route doit contenir uniquement des chiffres, reçu: {}",
            value
        )));
    }
    
    let angle: u32 = value.parse().map_err(|_| {
        AdexpError::InvalidFieldValue(format!("Angle de route invalide: {}", value))
    })?;
    
    if angle < 1 || angle > 360 {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Angle de route doit être entre 001 et 360, reçu: {}",
            value
        )));
    }
    
    Ok(())
}

/// Valide une altitude (ALT, ALTNZ)
/// Format: nombre (pieds) ou format FL
pub fn validate_altitude(value: &str) -> Result<(), AdexpError> {
    if value.is_empty() {
        return Err(AdexpError::InvalidFieldValue(
            "Altitude ne peut pas être vide".to_string()
        ));
    }
    
    // Format FL
    if value.starts_with("FL") {
        return validate_flight_level(value);
    }
    
    // Format numérique (pieds)
    if value.chars().all(|c| c.is_ascii_digit()) {
        let alt: u32 = value.parse().map_err(|_| {
            AdexpError::InvalidFieldValue(format!("Altitude invalide: {}", value))
        })?;
        
        // Plage raisonnable: 0-100000 pieds
        if alt > 100000 {
            return Err(AdexpError::InvalidFieldValue(format!(
                "Altitude doit être raisonnable (< 100000 pieds), reçu: {}",
                value
            )));
        }
        
        return Ok(());
    }
    
    Err(AdexpError::InvalidFieldValue(format!(
        "Format d'altitude invalide: {}",
        value
    )))
}

/// Valide une distance (DIST, RELDIST)
/// Format: nombre (unités variables)
pub fn validate_distance(value: &str) -> Result<(), AdexpError> {
    if value.is_empty() {
        return Err(AdexpError::InvalidFieldValue(
            "Distance ne peut pas être vide".to_string()
        ));
    }
    
    // Format numérique simple
    if value.chars().all(|c| c.is_ascii_digit() || c == '.') {
        if let Ok(dist) = value.parse::<f64>() {
            if dist < 0.0 || dist > 99999.0 {
                return Err(AdexpError::InvalidFieldValue(format!(
                    "Distance doit être raisonnable (0-99999), reçu: {}",
                    value
                )));
            }
            return Ok(());
        }
    }
    
    Err(AdexpError::InvalidFieldValue(format!(
        "Format de distance invalide: {}",
        value
    )))
}

/// Valide une adresse hexadécimale Mode S (HEXADDR)
/// Format: 6 caractères hexadécimaux
pub fn validate_hex_address(value: &str) -> Result<(), AdexpError> {
    if value.len() != 6 {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Adresse hexadécimale doit avoir 6 caractères, reçu: {} ({} caractères)",
            value, value.len()
        )));
    }
    
    if !value.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Adresse hexadécimale doit contenir uniquement des caractères hexadécimaux, reçu: {}",
            value
        )));
    }
    
    Ok(())
}

/// Valide un code ICAO à 3 lettres (OPRICAO, PERICAO)
/// Format: 3 lettres majuscules
pub fn validate_icao_3letter_code(value: &str) -> Result<(), AdexpError> {
    if value.len() != 3 {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Code ICAO doit avoir 3 lettres, reçu: {} ({} caractères)",
            value, value.len()
        )));
    }
    
    if !value.chars().all(|c| c.is_ascii_alphabetic() && c.is_uppercase()) {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Code ICAO doit contenir uniquement des lettres majuscules, reçu: {}",
            value
        )));
    }
    
    Ok(())
}

/// Valide un code ICAO (CODEICAO)
/// Format: variable selon le type de code
pub fn validate_code_icao(value: &str) -> Result<(), AdexpError> {
    if value.is_empty() {
        return Err(AdexpError::InvalidFieldValue(
            "Code ICAO ne peut pas être vide".to_string()
        ));
    }
    
    // Format général: lettres et chiffres, 2-8 caractères
    if value.len() < 2 || value.len() > 8 {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Code ICAO doit avoir 2-8 caractères, reçu: {} ({} caractères)",
            value, value.len()
        )));
    }
    
    if !value.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Code ICAO doit contenir uniquement des caractères alphanumériques, reçu: {}",
            value
        )));
    }
    
    Ok(())
}

/// Valide la catégorie de turbulence de sillage (WKTRC)
/// Valeurs valides: L (Light), M (Medium), H (Heavy), J (Super)
pub fn validate_wake_turbulence(value: &str) -> Result<(), AdexpError> {
    match value {
        "L" | "M" | "H" | "J" => Ok(()),
        _ => Err(AdexpError::InvalidFieldValue(format!(
            "Wake Turbulence Category doit être L, M, H ou J, reçu: {}",
            value
        ))),
    }
}

/// Valide un IFPS Flight Plan ID (IFPLID)
/// Format: variable, généralement alphanumérique
pub fn validate_ifplid(value: &str) -> Result<(), AdexpError> {
    if value.is_empty() {
        return Err(AdexpError::InvalidFieldValue(
            "IFPLID ne peut pas être vide".to_string()
        ));
    }
    
    if value.len() > 20 {
        return Err(AdexpError::InvalidFieldValue(format!(
            "IFPLID ne peut pas dépasser 20 caractères, reçu: {} ({} caractères)",
            value, value.len()
        )));
    }
    
    // Format général: alphanumérique avec tirets et underscores possibles
    if !value.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
        return Err(AdexpError::InvalidFieldValue(format!(
            "IFPLID doit contenir uniquement des caractères alphanumériques, tirets et underscores, reçu: {}",
            value
        )));
    }
    
    Ok(())
}

/// Valide une procédure (SID, STAR, ARRPROC, DEPPROC)
/// Format: variable, généralement alphanumérique avec tirets
pub fn validate_procedure(value: &str) -> Result<(), AdexpError> {
    if value.is_empty() {
        return Err(AdexpError::InvalidFieldValue(
            "Procédure ne peut pas être vide".to_string()
        ));
    }
    
    if value.len() > 20 {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Procédure ne peut pas dépasser 20 caractères, reçu: {} ({} caractères)",
            value, value.len()
        )));
    }
    
    // Format général: alphanumérique avec tirets, underscores et points
    if !value.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.') {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Procédure doit contenir uniquement des caractères alphanumériques, tirets, underscores et points, reçu: {}",
            value
        )));
    }
    
    Ok(())
}

/// Valide la structure d'une route (ROUTE)
/// Format: texte libre mais peut contenir des éléments structurés selon ICAO Field 15
pub fn validate_route_structure(value: &str) -> Result<(), AdexpError> {
    if value.is_empty() {
        return Ok(()); // ROUTE peut être vide
    }
    
    // Validation basique: vérifier que la route ne contient pas de caractères invalides
    // Format ICAO Field 15: peut contenir des points, vitesses, niveaux de vol, etc.
    // Pour l'instant, validation permissive - juste vérifier qu'il n'y a pas de caractères de contrôle
    if value.chars().any(|c| c.is_control() && c != '\n' && c != '\r' && c != '\t') {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Route contient des caractères de contrôle invalides: {}",
            value
        )));
    }
    
    // Longueur raisonnable (max 2000 caractères selon ADEXP)
    if value.len() > 2000 {
        return Err(AdexpError::InvalidFieldValue(format!(
            "Route trop longue (max 2000 caractères), reçu: {} caractères",
            value.len()
        )));
    }
    
    Ok(())
}

/// Valide une structure composée ADDR (Address compound field)
/// Les sous-champs sont validés individuellement, cette fonction valide la cohérence
pub fn validate_addr_structure(section: &Section) -> Result<(), AdexpError> {
    // ADDR peut avoir des sous-champs comme ADDR (adresse), FAC (facility)
    // Si ADDR est présent, valider que c'est une adresse valide
    if let Some(addr_values) = section.fields.get("ADDR") {
        for addr in addr_values {
            // Adresse ADEXP: généralement 8 caractères alphanumériques
            if addr.len() > 0 && addr.len() <= 8 {
                if !addr.chars().all(|c| c.is_ascii_alphanumeric()) {
                    return Err(AdexpError::InvalidFieldValue(format!(
                        "Adresse ADDR invalide: {}",
                        addr
                    )));
                }
            } else if addr.len() > 8 {
                return Err(AdexpError::InvalidFieldValue(format!(
                    "Adresse ADDR trop longue (max 8 caractères), reçu: {} caractères",
                    addr.len()
                )));
            }
        }
    }
    
    // FAC est optionnel, mais s'il est présent, valider
    if let Some(fac_values) = section.fields.get("FAC") {
        for fac in fac_values {
            if fac.len() > 20 {
                return Err(AdexpError::InvalidFieldValue(format!(
                    "FAC trop long (max 20 caractères), reçu: {} caractères",
                    fac.len()
                )));
            }
        }
    }
    
    Ok(())
}

/// Valide une structure composée RTEPTS (Route Points compound field)
/// Valide que les points de route sont cohérents
pub fn validate_rtepts_structure(section: &Section) -> Result<(), AdexpError> {
    // RTEPTS contient une liste de points avec des sous-champs: PT, PTID, LAT, LON, FL, ETO, ATOT, etc.
    // Validation: au moins un identifiant de point (PT ou PTID) doit être présent
    let has_pt = section.fields.contains_key("PT");
    let has_ptid = section.fields.contains_key("PTID");
    let has_lat = section.fields.contains_key("LAT");
    let has_lon = section.fields.contains_key("LON");
    
    // Si on a des coordonnées, elles doivent être toutes les deux présentes
    if has_lat && !has_lon {
        return Err(AdexpError::InvalidFieldValue(
            "RTEPTS: LAT présent sans LON".to_string()
        ));
    }
    if has_lon && !has_lat {
        return Err(AdexpError::InvalidFieldValue(
            "RTEPTS: LON présent sans LAT".to_string()
        ));
    }
    
    // Au moins un identifiant de point doit être présent (PT, PTID, ou coordonnées)
    if !has_pt && !has_ptid && !has_lat {
        return Err(AdexpError::InvalidFieldValue(
            "RTEPTS: au moins un identifiant de point (PT, PTID, ou LAT/LON) doit être présent".to_string()
        ));
    }
    
    // Valider les valeurs des sous-champs si présents
    if let Some(fl_values) = section.fields.get("FL") {
        for fl in fl_values {
            validate_flight_level(fl)?;
        }
    }
    
    if let Some(lat_values) = section.fields.get("LAT") {
        for lat in lat_values {
            validate_latitude(lat)?;
        }
    }
    
    if let Some(lon_values) = section.fields.get("LON") {
        for lon in lon_values {
            validate_longitude(lon)?;
        }
    }
    
    if let Some(eto_values) = section.fields.get("ETO") {
        for eto in eto_values {
            validate_time_hhmm(eto)?;
        }
    }
    
    if let Some(atot_values) = section.fields.get("ATOT") {
        for atot in atot_values {
            validate_time_hhmm(atot)?;
        }
    }
    
    if let Some(speed_values) = section.fields.get("SPEED") {
        for speed in speed_values {
            validate_speed(speed)?;
        }
    }
    
    Ok(())
}

/// Valide une structure composée VEC (Vector compound field)
/// Valide que les éléments de vecteur sont cohérents
pub fn validate_vec_structure(section: &Section) -> Result<(), AdexpError> {
    // VEC contient: TRACKANGLE, GROUNDSPEED, ALT
    // Au moins un élément doit être présent
    
    let has_track = section.fields.contains_key("TRACKANGLE");
    let has_speed = section.fields.contains_key("GROUNDSPEED");
    let has_alt = section.fields.contains_key("ALT");
    
    if !has_track && !has_speed && !has_alt {
        return Err(AdexpError::InvalidFieldValue(
            "VEC: au moins un élément (TRACKANGLE, GROUNDSPEED, ou ALT) doit être présent".to_string()
        ));
    }
    
    // Valider les valeurs si présentes
    if let Some(track_values) = section.fields.get("TRACKANGLE") {
        for track in track_values {
            validate_track_angle(track)?;
        }
    }
    
    if let Some(speed_values) = section.fields.get("GROUNDSPEED") {
        for speed in speed_values {
            validate_speed(speed)?;
        }
    }
    
    if let Some(alt_values) = section.fields.get("ALT") {
        for alt in alt_values {
            validate_altitude(alt)?;
        }
    }
    
    Ok(())
}

/// Valide une structure composée REFDATA (Reference data compound field)
/// Valide que les données de référence sont cohérentes
pub fn validate_refdata_structure(section: &Section) -> Result<(), AdexpError> {
    // REFDATA contient: IFPLID, ORIGIN, FAC, NETWORKTYPE
    // Au moins un élément doit être présent
    
    let has_ifplid = section.fields.contains_key("IFPLID");
    let has_origin = section.fields.contains_key("ORIGIN");
    let has_fac = section.fields.contains_key("FAC");
    let has_networktype = section.fields.contains_key("NETWORKTYPE");
    
    if !has_ifplid && !has_origin && !has_fac && !has_networktype {
        return Err(AdexpError::InvalidFieldValue(
            "REFDATA: au moins un élément (IFPLID, ORIGIN, FAC, ou NETWORKTYPE) doit être présent".to_string()
        ));
    }
    
    // Valider les valeurs si présentes
    if let Some(ifplid_values) = section.fields.get("IFPLID") {
        for ifplid in ifplid_values {
            validate_ifplid(ifplid)?;
        }
    }
    
    if let Some(origin_values) = section.fields.get("ORIGIN") {
        for origin in origin_values {
            if origin.len() > 20 {
                return Err(AdexpError::InvalidFieldValue(format!(
                    "ORIGIN trop long (max 20 caractères), reçu: {} caractères",
                    origin.len()
                )));
            }
        }
    }
    
    if let Some(fac_values) = section.fields.get("FAC") {
        for fac in fac_values {
            if fac.len() > 20 {
                return Err(AdexpError::InvalidFieldValue(format!(
                    "FAC trop long (max 20 caractères), reçu: {} caractères",
                    fac.len()
                )));
            }
        }
    }
    
    if let Some(networktype_values) = section.fields.get("NETWORKTYPE") {
        for networktype in networktype_values {
            // NETWORKTYPE: valeurs typiques comme "IFPS", "ATFM", etc.
            if networktype.len() > 20 {
                return Err(AdexpError::InvalidFieldValue(format!(
                    "NETWORKTYPE trop long (max 20 caractères), reçu: {} caractères",
                    networktype.len()
                )));
            }
        }
    }
    
    Ok(())
}

/// Valide les structures composées dans une section
/// Détecte si une section contient un champ composé et valide sa structure
pub fn validate_compound_fields_in_section(section: &Section) -> Result<(), AdexpError> {
    // Vérifier si cette section est une section de champ composé
    match section.name.as_str() {
        "ADDR" | "RTEPTS" | "VEC" | "REFDATA" => {
            // C'est une section dédiée à un champ composé
            match section.name.as_str() {
                "ADDR" => validate_addr_structure(section)?,
                "RTEPTS" => validate_rtepts_structure(section)?,
                "VEC" => validate_vec_structure(section)?,
                "REFDATA" => validate_refdata_structure(section)?,
                _ => {}
            }
        }
        _ => {
            // Vérifier si la section contient des champs composés comme sous-champs
            // Par exemple, une section peut contenir plusieurs RTEPTS
            if section.fields.contains_key("RTEPTS") {
                // Si RTEPTS est présent comme champ, valider la section entière comme RTEPTS
                validate_rtepts_structure(section)?;
            }
            if section.fields.contains_key("VEC") {
                validate_vec_structure(section)?;
            }
            if section.fields.contains_key("REFDATA") {
                validate_refdata_structure(section)?;
            }
            // ADDR peut être présent dans la section principale
            if section.fields.contains_key("ADDR") {
                validate_addr_structure(section)?;
            }
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

    #[test]
    fn test_validate_flight_rules() {
        assert!(validate_flight_rules("I").is_ok());
        assert!(validate_flight_rules("V").is_ok());
        assert!(validate_flight_rules("Y").is_ok());
        assert!(validate_flight_rules("Z").is_ok());
        assert!(validate_flight_rules("X").is_err()); // Invalide
        assert!(validate_flight_rules("IFR").is_err()); // Format incorrect
    }

    #[test]
    fn test_validate_flight_type() {
        assert!(validate_flight_type("S").is_ok());
        assert!(validate_flight_type("N").is_ok());
        assert!(validate_flight_type("G").is_ok());
        assert!(validate_flight_type("M").is_ok());
        assert!(validate_flight_type("X").is_ok());
        assert!(validate_flight_type("A").is_err()); // Invalide
    }

    #[test]
    fn test_validate_pbn() {
        assert!(validate_pbn("A1").is_ok());
        assert!(validate_pbn("A1,B1,C1").is_ok());
        assert!(validate_pbn("B1 B2").is_ok());
        assert!(validate_pbn("X1").is_err()); // Code invalide
        assert!(validate_pbn("").is_err()); // Vide
    }

    #[test]
    fn test_validate_wind_direction() {
        assert!(validate_wind_direction("000").is_ok());
        assert!(validate_wind_direction("360").is_ok());
        assert!(validate_wind_direction("180").is_ok());
        assert!(validate_wind_direction("361").is_err()); // Trop élevé
        assert!(validate_wind_direction("12").is_err()); // Trop court
    }

    #[test]
    fn test_validate_wind_speed() {
        assert!(validate_wind_speed("0").is_ok());
        assert!(validate_wind_speed("50").is_ok());
        assert!(validate_wind_speed("999").is_ok());
        assert!(validate_wind_speed("1000").is_err()); // Trop élevé
        assert!(validate_wind_speed("ABC").is_err()); // Non numérique
    }

    #[test]
    fn test_validate_pressure() {
        assert!(validate_pressure("1013").is_ok());
        assert!(validate_pressure("800").is_ok());
        assert!(validate_pressure("1100").is_ok());
        assert!(validate_pressure("799").is_err()); // Trop bas
        assert!(validate_pressure("1101").is_err()); // Trop élevé
    }

    #[test]
    fn test_validate_temperature() {
        assert!(validate_temperature("20").is_ok());
        assert!(validate_temperature("-40").is_ok());
        assert!(validate_temperature("+15").is_ok());
        assert!(validate_temperature("-81").is_err()); // Trop bas
        assert!(validate_temperature("61").is_err()); // Trop élevé
    }

    #[test]
    fn test_validate_track_angle() {
        assert!(validate_track_angle("001").is_ok());
        assert!(validate_track_angle("360").is_ok());
        assert!(validate_track_angle("180").is_ok());
        assert!(validate_track_angle("000").is_err()); // Trop bas
        assert!(validate_track_angle("361").is_err()); // Trop élevé
    }

    #[test]
    fn test_validate_altitude() {
        assert!(validate_altitude("FL350").is_ok());
        assert!(validate_altitude("35000").is_ok());
        assert!(validate_altitude("0").is_ok());
        assert!(validate_altitude("100001").is_err()); // Trop élevé
    }

    #[test]
    fn test_validate_hex_address() {
        assert!(validate_hex_address("ABCDEF").is_ok());
        assert!(validate_hex_address("123456").is_ok());
        assert!(validate_hex_address("abc123").is_ok());
        assert!(validate_hex_address("ABCDE").is_err()); // Trop court
        assert!(validate_hex_address("ABCDEFG").is_err()); // Trop long
        assert!(validate_hex_address("ABCDEG").is_err()); // Caractère invalide
    }

    #[test]
    fn test_validate_icao_3letter_code() {
        assert!(validate_icao_3letter_code("AFR").is_ok());
        assert!(validate_icao_3letter_code("UAE").is_ok());
        assert!(validate_icao_3letter_code("AF").is_err()); // Trop court
        assert!(validate_icao_3letter_code("AFRA").is_err()); // Trop long
        assert!(validate_icao_3letter_code("afr").is_err()); // Minuscules
    }

    #[test]
    fn test_validate_wake_turbulence() {
        assert!(validate_wake_turbulence("L").is_ok());
        assert!(validate_wake_turbulence("M").is_ok());
        assert!(validate_wake_turbulence("H").is_ok());
        assert!(validate_wake_turbulence("J").is_ok());
        assert!(validate_wake_turbulence("S").is_err()); // Invalide
    }

    #[test]
    fn test_validate_ifplid() {
        assert!(validate_ifplid("ABC123").is_ok());
        assert!(validate_ifplid("IFPL-2024-001").is_ok());
        assert!(validate_ifplid("").is_err()); // Vide
        assert!(validate_ifplid("A".repeat(21).as_str()).is_err()); // Trop long
    }

    #[test]
    fn test_validate_procedure() {
        assert!(validate_procedure("RWY27L").is_ok());
        assert!(validate_procedure("ILS-27L").is_ok());
        assert!(validate_procedure("STAR.ABC").is_ok());
        assert!(validate_procedure("").is_err()); // Vide
        assert!(validate_procedure("A".repeat(21).as_str()).is_err()); // Trop long
    }

    #[test]
    fn test_validate_route_structure() {
        assert!(validate_route_structure("LFPG LFPB").is_ok());
        assert!(validate_route_structure("").is_ok()); // Vide autorisé
        assert!(validate_route_structure("A".repeat(2001).as_str()).is_err()); // Trop long
    }
}

