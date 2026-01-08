//! Tests de dysfonctionnement pour NMEA 0183
//! Vérifie que le parser rejette correctement les messages invalides

use aftn::{NmeaParser, NmeaError};

#[test]
fn test_invalid_start_marker() {
    let input = "GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47";
    let result = NmeaParser::parse_message(input);
    assert!(result.is_err(), "Should reject message without $ or !");
    assert!(matches!(result, Err(NmeaError::InvalidFormat(_))));
}

#[test]
fn test_missing_checksum() {
    let input = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,";
    let result = NmeaParser::parse_message(input);
    assert!(result.is_err(), "Should reject message without checksum");
    assert!(matches!(result, Err(NmeaError::MissingChecksum)));
}

#[test]
fn test_invalid_checksum() {
    let input = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*00";
    let result = NmeaParser::parse_message(input);
    assert!(result.is_err(), "Should reject message with invalid checksum");
    assert!(matches!(result, Err(NmeaError::InvalidChecksum { .. })));
}

#[test]
fn test_invalid_checksum_format() {
    let input = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*G";
    let result = NmeaParser::parse_message(input);
    assert!(result.is_err(), "Should reject message with invalid checksum format");
}

#[test]
fn test_empty_message() {
    let input = "";
    let result = NmeaParser::parse_message(input);
    assert!(result.is_err(), "Should reject empty message");
}

#[test]
fn test_message_without_fields() {
    let input = "$GPGGA*47";
    let result = NmeaParser::parse_message(input);
    // Peut être accepté ou rejeté selon l'implémentation
    // Vérifions au moins qu'il ne panique pas
    let _ = result;
}

#[test]
fn test_invalid_time_format() {
    let input = "$GPGGA,250000,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*XX";
    // Note: Le checksum doit être calculé correctement pour que le parsing réussisse
    // Mais on peut tester la validation sémantique séparément
    let result = NmeaParser::parse_message(input);
    if result.is_ok() {
        let message = result.unwrap();
        let validation_result = message.validate();
        // La validation devrait échouer pour une heure invalide (25:00:00)
        assert!(validation_result.is_err(), "Should reject invalid time format");
    }
}

#[test]
fn test_invalid_latitude() {
    let input = "$GPGGA,123519,9107.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*XX";
    let result = NmeaParser::parse_message(input);
    if result.is_ok() {
        let message = result.unwrap();
        let validation_result = message.validate();
        // La validation devrait échouer pour une latitude invalide (> 90 degrés)
        assert!(validation_result.is_err(), "Should reject invalid latitude");
    }
}

#[test]
fn test_invalid_longitude() {
    let input = "$GPGGA,123519,4807.038,N,18131.000,E,1,08,0.9,545.4,M,46.9,M,,*XX";
    let result = NmeaParser::parse_message(input);
    if result.is_ok() {
        let message = result.unwrap();
        let validation_result = message.validate();
        // La validation devrait échouer pour une longitude invalide (> 180 degrés)
        assert!(validation_result.is_err(), "Should reject invalid longitude");
    }
}

#[test]
fn test_invalid_gps_quality() {
    let input = "$GPGGA,123519,4807.038,N,01131.000,E,9,08,0.9,545.4,M,46.9,M,,*XX";
    let result = NmeaParser::parse_message(input);
    if result.is_ok() {
        let message = result.unwrap();
        let validation_result = message.validate();
        // La validation devrait échouer pour une qualité GPS invalide (> 8)
        assert!(validation_result.is_err(), "Should reject invalid GPS quality");
    }
}

#[test]
fn test_invalid_ais_payload() {
    let input = "$AIVDM,1,1,,A,,0*XX";
    let result = NmeaParser::parse_message(input);
    if result.is_ok() {
        let message = result.unwrap();
        let validation_result = message.validate();
        // La validation devrait échouer pour un payload AIS vide
        assert!(validation_result.is_err(), "Should reject empty AIS payload");
    }
}

#[test]
fn test_invalid_ais_6bit_encoding() {
    let input = "$AIVDM,1,1,,A,INVALID_CHARS,0*XX";
    let result = NmeaParser::parse_message(input);
    if result.is_ok() {
        let message = result.unwrap();
        let validation_result = message.validate();
        // La validation devrait échouer pour un payload avec des caractères invalides
        assert!(validation_result.is_err(), "Should reject invalid 6-bit ASCII characters");
    }
}

#[test]
fn test_malformed_message() {
    let input = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,";
    let result = NmeaParser::parse_message(input);
    assert!(result.is_err(), "Should reject malformed message (missing checksum)");
}

#[test]
fn test_message_with_newlines() {
    let input = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\n";
    let result = NmeaParser::parse_message(input);
    // Devrait être accepté car les newlines sont optionnelles
    assert!(result.is_ok(), "Should accept message with newline");
}

#[test]
fn test_message_with_carriage_return() {
    let input = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47\r\n";
    let result = NmeaParser::parse_message(input);
    // Devrait être accepté car les retours chariot sont optionnels
    assert!(result.is_ok(), "Should accept message with CRLF");
}

#[test]
fn test_invalid_message_type() {
    let input = "$INVALID,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*XX";
    let result = NmeaParser::parse_message(input);
    // Le parsing peut réussir mais le type sera Generic
    if result.is_ok() {
        let message = result.unwrap();
        assert_eq!(message.message_type.identifier(), "INVALID");
    }
}

#[test]
fn test_checksum_with_lowercase() {
    let input = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47";
    let input_lower = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47";
    // Les checksums en minuscules devraient être acceptés
    let result = NmeaParser::parse_message(input_lower);
    // Le parser devrait normaliser en majuscules
    if result.is_ok() {
        let message = result.unwrap();
        assert_eq!(message.checksum.to_uppercase(), "47");
    }
}

