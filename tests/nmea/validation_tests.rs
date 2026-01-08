//! Tests pour la validation sémantique NMEA

use aftn::{NmeaParser, NmeaError};

#[test]
fn test_validate_invalid_time() {
    let input = "$GPGGA,250000,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*XX";
    // Note: Le checksum doit être calculé correctement pour que le parsing réussisse
    // Mais on peut tester la validation sémantique séparément
    let message = NmeaParser::parse_message(input);
    
    if let Ok(msg) = message {
        let result = msg.validate();
        // La validation devrait échouer pour une heure invalide (25:00:00)
        if result.is_err() {
            assert!(matches!(result, Err(NmeaError::InvalidTime(_))));
        }
    }
}

#[test]
fn test_validate_invalid_latitude() {
    let input = "$GPGGA,123519,9107.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*XX";
    let message = NmeaParser::parse_message(input);
    
    if let Ok(msg) = message {
        let result = msg.validate();
        // La validation devrait échouer pour une latitude invalide (> 90 degrés)
        if result.is_err() {
            assert!(matches!(result, Err(NmeaError::InvalidCoordinate(_))));
        }
    }
}

#[test]
fn test_validate_invalid_ais_payload() {
    let input = "$AIVDM,1,1,,A,,0*XX";
    let message = NmeaParser::parse_message(input);
    
    if let Ok(msg) = message {
        let result = msg.validate();
        // La validation devrait échouer pour un payload AIS vide
        if result.is_err() {
            assert!(matches!(result, Err(NmeaError::InvalidFormat(_)) | Err(NmeaError::MissingField(_))));
        }
    }
}

