//! Tests d'intégration pour la validation sémantique ADEXP

use aftn::AdexpParser;
use aftn::AdexpError;

#[test]
fn test_validate_fpl_with_invalid_date() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-EOBD 320120
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_err(), "Validation should fail for invalid date");
    if let Err(AdexpError::InvalidDateTime(_)) = result {
        // OK
    } else {
        panic!("Expected InvalidDateTime error");
    }
}

#[test]
fn test_validate_fpl_with_invalid_time() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-EOBT 2400
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_err(), "Validation should fail for invalid time");
    if let Err(AdexpError::InvalidDateTime(_)) = result {
        // OK
    } else {
        panic!("Expected InvalidDateTime error");
    }
}

#[test]
fn test_validate_fpl_with_invalid_aerodrome() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFP
-ADES LFPB
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_err(), "Validation should fail for invalid aerodrome code");
    if let Err(AdexpError::InvalidFieldValue(_)) = result {
        // OK
    } else {
        panic!("Expected InvalidFieldValue error");
    }
}

#[test]
fn test_validate_fpl_with_invalid_aircraft_id() {
    let input = "-ADEXP
-TITLE FPL
-ARCID 123ABC
-ADEP LFPG
-ADES LFPB
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_err(), "Validation should fail for invalid aircraft ID");
    if let Err(AdexpError::InvalidFieldValue(_)) = result {
        // OK
    } else {
        panic!("Expected InvalidFieldValue error");
    }
}

#[test]
fn test_validate_fpl_with_invalid_flight_level() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-RFL 1000
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_err(), "Validation should fail for invalid flight level");
    if let Err(AdexpError::InvalidFieldValue(_)) = result {
        // OK
    } else {
        panic!("Expected InvalidFieldValue error");
    }
}

#[test]
fn test_validate_fpl_with_valid_data() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-EOBD 151220
-EOBT 1200
-RFL FL350
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_ok(), "Validation should succeed for valid data: {:?}", result);
}

#[test]
fn test_validate_coordinates() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-LAT 48.8566
-LON 2.3522
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_ok(), "Validation should succeed for valid coordinates");
}

#[test]
fn test_validate_invalid_latitude() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-LAT 91.0
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_err(), "Validation should fail for invalid latitude");
}

#[test]
fn test_validate_invalid_longitude() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-LON 181.0
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_err(), "Validation should fail for invalid longitude");
}

#[test]
fn test_validate_ssr_code() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-COD 1234
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_ok(), "Validation should succeed for valid SSR code");
}

#[test]
fn test_validate_invalid_ssr_code() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-COD 1238
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_err(), "Validation should fail for invalid SSR code (8 > 7)");
}

