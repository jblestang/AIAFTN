//! Tests d'intégration pour les structures composées ADEXP

use aftn::AdexpParser;
use aftn::adexp::message::Section;
use aftn::adexp::validation;

#[test]
fn test_validate_addr_structure() {
    let mut section = Section::new("ADDR".to_string());
    section.add_field("ADDR".to_string(), "LFPGYYYX".to_string());
    section.add_field("FAC".to_string(), "LFPG".to_string());
    
    let result = validation::validate_addr_structure(&section);
    assert!(result.is_ok(), "Validation should succeed for valid ADDR structure");
}

#[test]
fn test_validate_addr_invalid_address() {
    let mut section = Section::new("ADDR".to_string());
    section.add_field("ADDR".to_string(), "LFPGYYYXX".to_string()); // Trop long
    
    let result = validation::validate_addr_structure(&section);
    assert!(result.is_err(), "Validation should fail for invalid address length");
}

#[test]
fn test_validate_rtepts_structure() {
    let mut section = Section::new("RTEPTS".to_string());
    section.add_field("PT".to_string(), "LFPG".to_string());
    section.add_field("FL".to_string(), "FL350".to_string());
    section.add_field("ETO".to_string(), "1200".to_string());
    
    let result = validation::validate_rtepts_structure(&section);
    assert!(result.is_ok(), "Validation should succeed for valid RTEPTS structure");
}

#[test]
fn test_validate_rtepts_with_coordinates() {
    let mut section = Section::new("RTEPTS".to_string());
    section.add_field("LAT".to_string(), "48.8566".to_string());
    section.add_field("LON".to_string(), "2.3522".to_string());
    section.add_field("FL".to_string(), "350".to_string());
    
    let result = validation::validate_rtepts_structure(&section);
    assert!(result.is_ok(), "Validation should succeed for RTEPTS with coordinates");
}

#[test]
fn test_validate_rtepts_missing_identifier() {
    let mut section = Section::new("RTEPTS".to_string());
    // Pas de PT, PTID, LAT/LON
    section.add_field("FL".to_string(), "FL350".to_string());
    
    let result = validation::validate_rtepts_structure(&section);
    assert!(result.is_err(), "Validation should fail when no point identifier is present");
}

#[test]
fn test_validate_rtepts_incomplete_coordinates() {
    let mut section = Section::new("RTEPTS".to_string());
    section.add_field("LAT".to_string(), "48.8566".to_string());
    // LON manquant
    
    let result = validation::validate_rtepts_structure(&section);
    assert!(result.is_err(), "Validation should fail when LAT is present without LON");
}

#[test]
fn test_validate_vec_structure() {
    let mut section = Section::new("VEC".to_string());
    section.add_field("TRACKANGLE".to_string(), "180".to_string());
    section.add_field("GROUNDSPEED".to_string(), "450".to_string());
    section.add_field("ALT".to_string(), "FL350".to_string());
    
    let result = validation::validate_vec_structure(&section);
    assert!(result.is_ok(), "Validation should succeed for valid VEC structure");
}

#[test]
fn test_validate_vec_minimal() {
    let mut section = Section::new("VEC".to_string());
    section.add_field("TRACKANGLE".to_string(), "090".to_string());
    
    let result = validation::validate_vec_structure(&section);
    assert!(result.is_ok(), "Validation should succeed with at least one VEC element");
}

#[test]
fn test_validate_vec_empty() {
    let section = Section::new("VEC".to_string());
    
    let result = validation::validate_vec_structure(&section);
    assert!(result.is_err(), "Validation should fail when no VEC element is present");
}

#[test]
fn test_validate_refdata_structure() {
    let mut section = Section::new("REFDATA".to_string());
    section.add_field("IFPLID".to_string(), "IFPL-2024-001".to_string());
    section.add_field("ORIGIN".to_string(), "LFPG".to_string());
    section.add_field("FAC".to_string(), "LFPG".to_string());
    section.add_field("NETWORKTYPE".to_string(), "IFPS".to_string());
    
    let result = validation::validate_refdata_structure(&section);
    assert!(result.is_ok(), "Validation should succeed for valid REFDATA structure");
}

#[test]
fn test_validate_refdata_minimal() {
    let mut section = Section::new("REFDATA".to_string());
    section.add_field("IFPLID".to_string(), "IFPL-2024-001".to_string());
    
    let result = validation::validate_refdata_structure(&section);
    assert!(result.is_ok(), "Validation should succeed with at least one REFDATA element");
}

#[test]
fn test_validate_refdata_empty() {
    let section = Section::new("REFDATA".to_string());
    
    let result = validation::validate_refdata_structure(&section);
    assert!(result.is_err(), "Validation should fail when no REFDATA element is present");
}

#[test]
fn test_validate_route_in_message() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-ROUTE LFPG LFPB
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_ok(), "Validation should succeed for valid ROUTE");
}

#[test]
fn test_validate_route_too_long() {
    let long_route = "A".repeat(2001);
    let input = format!("-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-ROUTE {}
", long_route);
    
    let message = AdexpParser::parse_message(&input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_err(), "Validation should fail for route too long");
}

#[test]
fn test_validate_rtepts_in_message() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-RTEPTS
-PT LFPG
-FL FL350
-ETO 1200
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_ok(), "Validation should succeed for valid RTEPTS in message");
}

#[test]
fn test_validate_vec_in_message() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-VEC
-TRACKANGLE 180
-GROUNDSPEED 450
-ALT FL350
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_ok(), "Validation should succeed for valid VEC in message");
}

#[test]
fn test_validate_refdata_in_message() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-REFDATA
-IFPLID IFPL-2024-001
-ORIGIN LFPG
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_ok(), "Validation should succeed for valid REFDATA in message");
}

