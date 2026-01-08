//! Tests d'intégration pour les validations sémantiques étendues ADEXP

use aftn::AdexpParser;
use aftn::AdexpError;

#[test]
fn test_validate_fpl_with_pbn() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-PBN A1,B1,C1
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_ok(), "Validation should succeed for valid PBN");
}

#[test]
fn test_validate_fpl_with_invalid_pbn() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-PBN X1
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_err(), "Validation should fail for invalid PBN");
}

#[test]
fn test_validate_fpl_with_flight_rules() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-FLTRUL I
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_ok(), "Validation should succeed for valid flight rules");
}

#[test]
fn test_validate_fpl_with_invalid_flight_rules() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-FLTRUL X
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_err(), "Validation should fail for invalid flight rules");
}

#[test]
fn test_validate_fpl_with_flight_type() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-FLTTYP S
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_ok(), "Validation should succeed for valid flight type");
}

#[test]
fn test_validate_fpl_with_aircraft_type() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-ARCTYP A320
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_ok(), "Validation should succeed for valid aircraft type");
}

#[test]
fn test_validate_fpl_with_invalid_aircraft_type() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-ARCTYP A
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_err(), "Validation should fail for invalid aircraft type (too short)");
}

#[test]
fn test_validate_fpl_with_weather() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-WINDIR 270
-WINDSPEED 15
-QNH 1013
-AIRTEMP 20
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_ok(), "Validation should succeed for valid weather data");
}

#[test]
fn test_validate_fpl_with_invalid_wind_direction() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-WINDIR 361
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_err(), "Validation should fail for invalid wind direction");
}

#[test]
fn test_validate_fpl_with_track_angle() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-TRACKANGLE 180
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_ok(), "Validation should succeed for valid track angle");
}

#[test]
fn test_validate_fpl_with_invalid_track_angle() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-TRACKANGLE 000
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_err(), "Validation should fail for invalid track angle (000)");
}

#[test]
fn test_validate_fpl_with_wake_turbulence() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-WKTRC H
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_ok(), "Validation should succeed for valid wake turbulence");
}

#[test]
fn test_validate_fpl_with_invalid_wake_turbulence() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-WKTRC S
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_err(), "Validation should fail for invalid wake turbulence");
}

#[test]
fn test_validate_fpl_with_procedures() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-SID RWY27L
-STAR ILS-27L
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_ok(), "Validation should succeed for valid procedures");
}

#[test]
fn test_validate_fpl_with_equipment_codes() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-NAV A,B,C
-COM A,B
-DAT A
-SUR A,B1
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_ok(), "Validation should succeed for valid equipment codes");
}

#[test]
fn test_validate_fpl_with_invalid_nav_code() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-NAV X1
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_err(), "Validation should fail for invalid NAV code");
}

#[test]
fn test_validate_fpl_with_hex_address() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-HEXADDR ABC123
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_ok(), "Validation should succeed for valid hex address");
}

#[test]
fn test_validate_fpl_with_invalid_hex_address() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-HEXADDR ABC12G
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_err(), "Validation should fail for invalid hex address (G is not hex)");
}

#[test]
fn test_validate_fpl_with_icao_codes() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-OPRICAO AFR
-PERICAO FRA
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_ok(), "Validation should succeed for valid ICAO codes");
}

#[test]
fn test_validate_fpl_with_invalid_icao_code() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-OPRICAO AF
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    
    assert!(result.is_err(), "Validation should fail for invalid ICAO code (too short)");
}

