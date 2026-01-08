//! Test pour les champs ACTARR (Actual Arrival) et ACTDEP (Actual Departure)

use aftn::AdexpParser;
use aftn::adexp::fields::AdexpFields;

#[test]
fn test_actarr_field_validation() {
    // Vérifier que ACTARR est reconnu comme un champ valide
    assert!(AdexpFields::is_primary_field("ACTARR"));
    assert!(AdexpFields::is_valid_field("ACTARR"));
}

#[test]
fn test_parse_message_with_actarr() {
    let input = "-ADEXP
-TITLE ARR
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-ACTARR 1400
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse successfully");
    
    // Vérifier que ACTARR est parsé
    let actarr = message.get_field_value("", "ACTARR").unwrap();
    assert_eq!(actarr, Some(&"1400".to_string()));
}

#[test]
fn test_actarr_in_arrival_message() {
    let input = "-ADEXP
-TITLE ARR
-ARCID ABC123
-ADES LFPB
-ACTARR 151430
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse successfully");
    assert_eq!(message.message_type, aftn::AdexpMessageType::Arrival);
    
    let actarr = message.get_field_value("", "ACTARR").unwrap();
    assert_eq!(actarr, Some(&"151430".to_string()));
}

#[test]
fn test_actdep_field_validation() {
    // Vérifier que ACTDEP est reconnu comme un champ valide
    assert!(AdexpFields::is_primary_field("ACTDEP"));
    assert!(AdexpFields::is_valid_field("ACTDEP"));
}

#[test]
fn test_parse_message_with_actdep() {
    let input = "-ADEXP
-TITLE DEP
-ARCID ABC123
-ADEP LFPG
-ACTDEP 1200
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse successfully");
    
    // Vérifier que ACTDEP est parsé
    let actdep = message.get_field_value("", "ACTDEP").unwrap();
    assert_eq!(actdep, Some(&"1200".to_string()));
}

#[test]
fn test_actdep_in_departure_message() {
    let input = "-ADEXP
-TITLE DEP
-ARCID ABC123
-ADEP LFPG
-ACTDEP 121530
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse successfully");
    assert_eq!(message.message_type, aftn::AdexpMessageType::Departure);
    
    let actdep = message.get_field_value("", "ACTDEP").unwrap();
    assert_eq!(actdep, Some(&"121530".to_string()));
}

#[test]
fn test_actarr_and_actdep_together() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-ACTDEP 1200
-ACTARR 1400
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse successfully");
    
    let actdep = message.get_field_value("", "ACTDEP").unwrap();
    let actarr = message.get_field_value("", "ACTARR").unwrap();
    
    assert_eq!(actdep, Some(&"1200".to_string()));
    assert_eq!(actarr, Some(&"1400".to_string()));
}

