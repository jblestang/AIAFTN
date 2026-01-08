//! Tests pour la validation sémantique SBS

use aftn::{SbsParser, SbsError};

#[test]
fn test_validate_identification() {
    let input = "MSG,1,145,29315,4CA2E6,27215,2015/02/05,14:53:22.734,2015/02/05,14:53:22.734,BAW1425,,,,,,,,,,,0";
    let message = SbsParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    assert!(result.is_ok(), "Valid identification message should pass validation");
}

#[test]
fn test_validate_airborne_position() {
    let input = "MSG,3,145,29315,4CA2E6,27215,2015/02/05,14:53:22.734,2015/02/05,14:53:22.734,,37025,1035.0,295.6,51.4703,-0.4543,,,,,0";
    let message = SbsParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    assert!(result.is_ok(), "Valid airborne position message should pass validation");
}

#[test]
fn test_validate_airborne_velocity() {
    let input = "MSG,4,145,29315,4CA2E6,27215,2015/02/05,14:53:22.734,2015/02/05,14:53:22.734,,,1035.0,295.6,,,-3200,,,,,0";
    let message = SbsParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    assert!(result.is_ok(), "Valid airborne velocity message should pass validation");
}

#[test]
fn test_validate_invalid_altitude() {
    let input = "MSG,3,145,29315,4CA2E6,27215,2015/02/05,14:53:22.734,2015/02/05,14:53:22.734,,200000,1035.0,295.6,51.4703,-0.4543,,,,,0";
    let message = SbsParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    assert!(result.is_err(), "Should reject invalid altitude");
    
    if let Err(SbsError::InvalidAltitude(_)) = result {
        // OK
    } else {
        panic!("Expected InvalidAltitude error");
    }
}

#[test]
fn test_validate_invalid_latitude() {
    let input = "MSG,3,145,29315,4CA2E6,27215,2015/02/05,14:53:22.734,2015/02/05,14:53:22.734,,37025,1035.0,295.6,91.0,-0.4543,,,,,0";
    let message = SbsParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    assert!(result.is_err(), "Should reject invalid latitude");
}

#[test]
fn test_validate_invalid_squawk() {
    let input = "MSG,6,145,29315,4CA2E6,27215,2015/02/05,14:53:22.734,2015/02/05,14:53:22.734,,,,,,,,,9999,,,0";
    let message = SbsParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    assert!(result.is_err(), "Should reject invalid squawk (contains 9, which is not octal)");
}

#[test]
fn test_validate_missing_altitude() {
    let input = "MSG,3,145,29315,4CA2E6,27215,2015/02/05,14:53:22.734,2015/02/05,14:53:22.734,,,,295.6,51.4703,-0.4543,,,,,0";
    let message = SbsParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    assert!(result.is_err(), "Should reject missing altitude for MSG,3");
    
    if let Err(SbsError::MissingField(_)) = result {
        // OK
    } else {
        panic!("Expected MissingField error");
    }
}

