//! Test pour le type de message CHGDEP (Changed Departure)
//! CHGDEP est un titre de message réservé par la France selon ADEXP 3.4

use aftn::AdexpParser;
use aftn::AdexpMessageType;

#[test]
fn test_parse_chgdep_message() {
    let input = "-ADEXP
-TITLE CHGDEP
-ARCID ABC123
-ADEP LFPG
-ACTDEP 1200
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse successfully");
    
    // Vérifier que le type de message est ChangedDeparture
    assert_eq!(message.message_type, AdexpMessageType::ChangedDeparture);
    
    // Vérifier que les champs sont parsés
    let arcid = message.get_field_value("", "ARCID").unwrap();
    assert_eq!(arcid, Some(&"ABC123".to_string()));
    
    let adep = message.get_field_value("", "ADEP").unwrap();
    assert_eq!(adep, Some(&"LFPG".to_string()));
}

#[test]
fn test_chgdep_with_departure_time() {
    let input = "-ADEXP
-TITLE CHGDEP
-ARCID ABC123
-ADEP LFPG
-ACTDEP 121530
-EOBT 1200
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse successfully");
    assert_eq!(message.message_type, AdexpMessageType::ChangedDeparture);
    
    let actdep = message.get_field_value("", "ACTDEP").unwrap();
    assert_eq!(actdep, Some(&"121530".to_string()));
    
    let eobt = message.get_field_value("", "EOBT").unwrap();
    assert_eq!(eobt, Some(&"1200".to_string()));
}

#[test]
fn test_chgdep_message_type_prefix() {
    use aftn::adexp::types::MessageType;
    
    let msg_type = MessageType::ChangedDeparture;
    assert_eq!(msg_type.prefix(), "CHGDEP");
}

#[test]
fn test_chgdep_from_title() {
    use aftn::adexp::types::MessageType;
    
    let msg_type = MessageType::from_title("CHGDEP").unwrap();
    assert_eq!(msg_type, MessageType::ChangedDeparture);
    
    // Test case insensitive
    let msg_type_lower = MessageType::from_title("chgdep").unwrap();
    assert_eq!(msg_type_lower, MessageType::ChangedDeparture);
}

