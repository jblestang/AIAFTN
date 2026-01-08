//! Test pour le champ ACTARR (Actual Arrival)

use aftn::{AdexpParser, AdexpError};
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

