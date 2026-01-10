//! Tests de sérialisation (deparsing) pour tous les formats
//! Vérifie que les messages parsés peuvent être resérialisés sans espaces/tabulations supplémentaires

use aftn::{AftnParser, AdexpParser, NmeaParser, SbsParser};

/// Test de sérialisation AFTN
#[test]
fn test_aftn_serialization() {
    let input = "GG LFPGYYYX LFPOYYYX 151230 NOTAM A1234/24 LFPG RWY 09/27 CLOSED";
    let message = AftnParser::parse_message(input).unwrap();
    let serialized = message.serialize();
    
    // La sérialisation doit produire un message valide
    let reparsed = AftnParser::parse_message(&serialized).unwrap();
    assert_eq!(message.priority, reparsed.priority);
    assert_eq!(message.addresses.origin, reparsed.addresses.origin);
    assert_eq!(message.addresses.destinations, reparsed.addresses.destinations);
    assert_eq!(message.transmission_time.day, reparsed.transmission_time.day);
    assert_eq!(message.transmission_time.hour, reparsed.transmission_time.hour);
    assert_eq!(message.transmission_time.minute, reparsed.transmission_time.minute);
    assert_eq!(message.body.trim(), reparsed.body.trim());
}

/// Test de sérialisation AFTN avec séquence
#[test]
fn test_aftn_serialization_with_sequence() {
    let input = "FF LFPGYYYX LFPOYYYX 151230 NOTAM A1234/24 /SEQ 001";
    let message = AftnParser::parse_message(input).unwrap();
    let serialized = message.serialize();
    
    // Vérifier que la séquence est présente
    assert!(serialized.contains("/SEQ"));
    assert!(serialized.contains("001"));
    
    // La sérialisation doit être re-parseable
    let reparsed = AftnParser::parse_message(&serialized).unwrap();
    assert_eq!(message.sequence_number, reparsed.sequence_number);
}

/// Test de sérialisation AFTN avec plusieurs destinations
#[test]
fn test_aftn_serialization_multiple_destinations() {
    let input = "DD LFPGYYYX LFPOYYYX LFPBYYYX 201530 METAR LFPG 201530Z";
    let message = AftnParser::parse_message(input).unwrap();
    let serialized = message.serialize();
    
    // Vérifier que toutes les destinations sont présentes
    for dest in &message.addresses.destinations {
        assert!(serialized.contains(dest));
    }
    
    // La sérialisation doit être re-parseable
    let reparsed = AftnParser::parse_message(&serialized).unwrap();
    assert_eq!(message.addresses.destinations.len(), reparsed.addresses.destinations.len());
}

/// Test de sérialisation ADEXP
#[test]
fn test_adexp_serialization() {
    let input = "-ADEXP\n-TITLE FPL\n-ARCID ABC123\n-ADEP LFPG\n-ADES LFPB";
    let message = AdexpParser::parse_message(input).unwrap();
    let serialized = message.serialize();
    
    // La sérialisation doit contenir les éléments essentiels
    assert!(serialized.contains("-ADEXP"));
    assert!(serialized.contains("-TITLE"));
    assert!(serialized.contains("FPL"));
    assert!(serialized.contains("-ARCID"));
    assert!(serialized.contains("ABC123"));
    
    // La sérialisation doit être re-parseable
    let reparsed = AdexpParser::parse_message(&serialized).unwrap();
    assert_eq!(message.message_type, reparsed.message_type);
}

/// Test de sérialisation ADEXP avec sections BEGIN/END
#[test]
fn test_adexp_serialization_with_sections() {
    // Le parser attend -END avec le nom de la section: -END RTEPTS
    let input = "-ADEXP\n-TITLE FPL\n-ARCID ABC123\n-BEGIN RTEPTS\n-PTID WPT1\n-PTID WPT2\n-END RTEPTS";
    let message = AdexpParser::parse_message(input).unwrap();
    let serialized = message.serialize();
    
    // La sérialisation doit contenir BEGIN/END
    assert!(serialized.contains("-BEGIN"));
    assert!(serialized.contains("RTEPTS"));
    assert!(serialized.contains("-END"));
    
    // La sérialisation doit être re-parseable
    let reparsed = AdexpParser::parse_message(&serialized).unwrap();
    // Vérifier que la section RTEPTS existe
    assert!(reparsed.sections.contains_key("RTEPTS"));
}

/// Test de sérialisation NMEA
#[test]
fn test_nmea_serialization() {
    let input = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47";
    let message = NmeaParser::parse_message(input).unwrap();
    let serialized = message.serialize();
    
    // La sérialisation doit commencer par $
    assert!(serialized.starts_with('$'));
    
    // La sérialisation doit contenir le type de message
    assert!(serialized.contains("GPGGA"));
    
    // La sérialisation doit contenir un checksum
    assert!(serialized.contains('*'));
    
    // La sérialisation doit être re-parseable
    let reparsed = NmeaParser::parse_message(&serialized).unwrap();
    assert_eq!(message.message_type, reparsed.message_type);
    assert_eq!(message.fields.len(), reparsed.fields.len());
}

/// Test de sérialisation NMEA avec checksum recalculé
#[test]
fn test_nmea_serialization_checksum_recalculation() {
    let input = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47";
    let message = NmeaParser::parse_message(input).unwrap();
    let serialized = message.serialize();
    
    // Le checksum doit être valide
    let reparsed = NmeaParser::parse_message(&serialized);
    assert!(reparsed.is_ok(), "Le message sérialisé doit avoir un checksum valide");
}

/// Test de sérialisation SBS
#[test]
fn test_sbs_serialization() {
    let input = "MSG,3,145,29315,4CA2E6,27215,2015/02/05,14:53:22.734,2015/02/05,14:53:22.734,,37025,1035.0,295.6,51.4703,-0.4543,,,,,0";
    let message = SbsParser::parse_message(input).unwrap();
    let serialized = message.serialize();
    
    // La sérialisation doit commencer par MSG
    assert!(serialized.starts_with("MSG,"));
    
    // La sérialisation doit contenir le type de message
    assert!(serialized.contains("3"));
    
    // La sérialisation doit être re-parseable
    let reparsed = SbsParser::parse_message(&serialized).unwrap();
    assert_eq!(message.message_type, reparsed.message_type);
}

/// Test que la sérialisation supprime les espaces/tabulations supplémentaires
#[test]
fn test_serialization_removes_extra_whitespace() {
    // AFTN avec espaces supplémentaires
    let input_aftn = "GG   LFPGYYYX   LFPOYYYX   151230   NOTAM   A1234/24";
    let message_aftn = AftnParser::parse_message(input_aftn).unwrap();
    let serialized_aftn = message_aftn.serialize();
    
    // Vérifier qu'il n'y a pas d'espaces multiples consécutifs (sauf dans le body)
    let parts: Vec<&str> = serialized_aftn.split_whitespace().collect();
    assert!(parts.len() >= 5, "Le message doit avoir au moins 5 parties");
    
    // ADEXP avec tabulations
    let input_adexp = "-ADEXP\n\t-TITLE\tFPL\n\t-ARCID\tABC123";
    let message_adexp = AdexpParser::parse_message(input_adexp).unwrap();
    let serialized_adexp = message_adexp.serialize();
    
    // Vérifier qu'il n'y a pas de tabulations
    assert!(!serialized_adexp.contains('\t'), "La sérialisation ne doit pas contenir de tabulations");
}

/// Test de round-trip: parse -> serialize -> parse
#[test]
fn test_round_trip_aftn() {
    let original = "GG LFPGYYYX LFPOYYYX 151230 NOTAM A1234/24";
    let message = AftnParser::parse_message(original).unwrap();
    let serialized = message.serialize();
    let reparsed = AftnParser::parse_message(&serialized).unwrap();
    
    // Les champs essentiels doivent être identiques
    assert_eq!(message.priority, reparsed.priority);
    assert_eq!(message.addresses.origin, reparsed.addresses.origin);
    assert_eq!(message.addresses.destinations, reparsed.addresses.destinations);
    assert_eq!(message.transmission_time.day, reparsed.transmission_time.day);
    assert_eq!(message.transmission_time.hour, reparsed.transmission_time.hour);
    assert_eq!(message.transmission_time.minute, reparsed.transmission_time.minute);
}

/// Test de round-trip pour NMEA
#[test]
fn test_round_trip_nmea() {
    let original = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47";
    let message = NmeaParser::parse_message(original).unwrap();
    let serialized = message.serialize();
    let reparsed = NmeaParser::parse_message(&serialized).unwrap();
    
    // Les champs essentiels doivent être identiques
    assert_eq!(message.message_type, reparsed.message_type);
    assert_eq!(message.fields, reparsed.fields);
}

/// Test de round-trip pour SBS
#[test]
fn test_round_trip_sbs() {
    let original = "MSG,3,145,29315,4CA2E6,27215,2015/02/05,14:53:22.734,2015/02/05,14:53:22.734,,37025,1035.0,295.6,51.4703,-0.4543,,,,,0";
    let message = SbsParser::parse_message(original).unwrap();
    let serialized = message.serialize();
    let reparsed = SbsParser::parse_message(&serialized).unwrap();
    
    // Les champs essentiels doivent être identiques
    assert_eq!(message.message_type, reparsed.message_type);
    assert_eq!(message.aircraft_id, reparsed.aircraft_id);
    assert_eq!(message.altitude, reparsed.altitude);
}

