use aftn::{AftnParser, AftnError};
use aftn::MessageCategory;

/// Tests d'intégration pour les sous-messages
#[test]
fn test_notam_submessage() {
    let input = "GG LFPGYYYX LFPOYYYX 151230 NOTAM A1234/24 LFPG RWY 09/27 CLOSED";
    let message = AftnParser::parse_message(input).expect("Should parse successfully");
    
    assert_eq!(message.category, MessageCategory::Notam);
    assert!(message.body.contains("NOTAM"));
    
    // Le sous-message devrait être parsable
    let submessage = aftn::submessages::parse_submessage(&message.category, &message.body);
    assert!(submessage.is_ok());
}

#[test]
fn test_metar_submessage() {
    let input = "GG LFPGYYYX LFPOYYYX 151230 METAR LFPG 151230Z 28015KT 9999 FEW030 12/08 Q1013";
    let message = AftnParser::parse_message(input).expect("Should parse successfully");
    
    assert_eq!(message.category, MessageCategory::Metar);
    
    let submessage = aftn::submessages::parse_submessage(&message.category, &message.body);
    assert!(submessage.is_ok());
}

#[test]
fn test_taf_submessage() {
    let input = "DD LFPGYYYX LFPOYYYX 151200 TAF LFPG 151200Z 1512/1612 28015KT 9999 FEW030";
    let message = AftnParser::parse_message(input).expect("Should parse successfully");
    
    assert_eq!(message.category, MessageCategory::Taf);
    
    let submessage = aftn::submessages::parse_submessage(&message.category, &message.body);
    assert!(submessage.is_ok());
}

#[test]
fn test_fpl_submessage() {
    let input = "SS LFPGYYYX LFPOYYYX 151200 FPL ABC123 V LFPG 151200 LFPB 1800";
    let message = AftnParser::parse_message(input).expect("Should parse successfully");
    
    assert_eq!(message.category, MessageCategory::FlightPlan);
    
    let submessage = aftn::submessages::parse_submessage(&message.category, &message.body);
    assert!(submessage.is_ok());
}

#[test]
fn test_all_submessage_categories() {
    let test_cases = vec![
        ("NOTAM A1234/24 LFPG", MessageCategory::Notam),
        ("METAR LFPG 151230Z 28015KT", MessageCategory::Metar),
        ("TAF LFPG 151200Z 1512/1612 28015KT", MessageCategory::Taf),
        ("SIGMET VALID 151230/152030", MessageCategory::Sigmet),
        ("AIRMET VALID 151230/152030", MessageCategory::Airmet),
        ("ATIS LFPG INFORMATION", MessageCategory::Atis),
        ("VOLMET LFPG METAR", MessageCategory::Volmet),
        ("FPL ABC123 V LFPG 151200 LFPB 1800", MessageCategory::FlightPlan),
        ("POS ABC123 151230", MessageCategory::PositionReport),
    ];
    
    for (body, expected_category) in test_cases {
        let submessage = aftn::aftn::submessages::parse_submessage(&expected_category, body);
        assert!(submessage.is_ok(), "Failed to parse {}: {:?}", body, submessage);
        
        let msg = submessage.unwrap();
        assert_eq!(msg.category(), expected_category);
    }
}

