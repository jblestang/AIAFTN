use aftn::{AftnParser, AftnError};
use aftn::categories::MessageCategory;

#[test]
fn test_integration_complete_notam() {
    let input = "GG LFPGYYYX LFPOYYYX LFPBYYYX 151230 NOTAM A1234/24 LFPG RWY 09/27 CLOSED DUE TO MAINTENANCE";
    let message = AftnParser::parse_message(input).expect("Should parse successfully");
    
    assert_eq!(message.priority, "GG");
    assert_eq!(message.addresses.origin, "LFPGYYYX");
    assert_eq!(message.addresses.destinations.len(), 2);
    assert_eq!(message.category, MessageCategory::Notam);
    assert!(message.body.contains("NOTAM"));
}

#[test]
fn test_integration_metar_format() {
    let input = "DD LFPGYYYX LFPOYYYX 201530 METAR LFPG 201530Z 28015KT 9999 FEW030 12/08 Q1013 NOSIG";
    let message = AftnParser::parse_message(input).expect("Should parse successfully");
    
    assert_eq!(message.category, MessageCategory::Metar);
    assert!(message.body.contains("METAR"));
    assert_eq!(message.transmission_time.hour, 15);
    assert_eq!(message.transmission_time.minute, 30);
}

#[test]
fn test_integration_taf_format() {
    let input = "FF LFPGYYYX LFPOYYYX 151200 TAF LFPG 151200Z 1512/1612 28015KT 9999 FEW030 BECMG 1518/1520 30020G35KT";
    let message = AftnParser::parse_message(input).expect("Should parse successfully");
    
    assert_eq!(message.category, MessageCategory::Taf);
    assert!(message.body.contains("TAF"));
}

#[test]
fn test_integration_flight_plan() {
    let input = "SS LFPGYYYX LFPOYYYX 151200 FPL ABC123 V LFPG 151200 LFPB 1800";
    let message = AftnParser::parse_message(input).expect("Should parse successfully");
    
    assert_eq!(message.category, MessageCategory::FlightPlan);
}

#[test]
fn test_integration_with_sequence() {
    let input = "GG LFPGYYYX LFPOYYYX 151230 TEST MESSAGE CONTENT /SEQ 12345";
    let message = AftnParser::parse_message(input).expect("Should parse successfully");
    
    assert_eq!(message.sequence_number, Some("12345".to_string()));
    assert!(message.body.contains("TEST MESSAGE CONTENT"));
}

