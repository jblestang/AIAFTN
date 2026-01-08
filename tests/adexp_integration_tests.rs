use aftn::{AdexpParser, AdexpError};
use aftn::AdexpMessageType;

#[test]
fn test_integration_complete_fpl() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-EOBD 15
-EOBT 1200
-ROUTE LFPG DCT LFPB
";
    let message = AdexpParser::parse_message(input).expect("Should parse successfully");
    
    assert_eq!(message.message_type, AdexpMessageType::FlightPlan);
    assert_eq!(message.get_field_value("", "ARCID").unwrap(), Some(&"ABC123".to_string()));
    assert_eq!(message.get_field_value("", "ADEP").unwrap(), Some(&"LFPG".to_string()));
    assert_eq!(message.get_field_value("", "ADES").unwrap(), Some(&"LFPB".to_string()));
}

#[test]
fn test_integration_chg_message() {
    let input = "-ADEXP
-TITLE CHG
-ARCID ABC123
-ADEP LFPG
";
    let message = AdexpParser::parse_message(input).expect("Should parse successfully");
    
    assert_eq!(message.message_type, AdexpMessageType::Change);
    assert_eq!(message.get_field_value("", "ARCID").unwrap(), Some(&"ABC123".to_string()));
}

#[test]
fn test_integration_cnl_message() {
    let input = "-ADEXP
-TITLE CNL
-ARCID ABC123
";
    let message = AdexpParser::parse_message(input).expect("Should parse successfully");
    
    assert_eq!(message.message_type, AdexpMessageType::Cancel);
}

#[test]
fn test_integration_dla_message() {
    let input = "-ADEXP
-TITLE DLA
-ARCID ABC123
-ADEP LFPG
";
    let message = AdexpParser::parse_message(input).expect("Should parse successfully");
    
    assert_eq!(message.message_type, AdexpMessageType::Delay);
}

#[test]
fn test_integration_with_sections() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ROUTE
-ADEP LFPG
-ADES LFPB
";
    let message = AdexpParser::parse_message(input).expect("Should parse successfully");
    
    // Les champs dans la section ROUTE devraient être accessibles
    assert!(message.get_field_value("ROUTE", "ADEP").is_ok());
}

#[test]
fn test_integration_with_end_marker() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-END
";
    let message = AdexpParser::parse_message(input).expect("Should parse successfully");
    
    assert_eq!(message.message_type, AdexpMessageType::FlightPlan);
}

