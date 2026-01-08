use aftn::AdexpParser;
use aftn::AdexpMessageType;

/// Tests pour les différents types de messages ADEXP
#[test]
fn test_fpl_message() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
";
    let msg = AdexpParser::parse_message(input).unwrap();
    assert_eq!(msg.message_type, AdexpMessageType::FlightPlan);
}

#[test]
fn test_chg_message() {
    let input = "-ADEXP
-TITLE CHG
-ARCID ABC123
";
    let msg = AdexpParser::parse_message(input).unwrap();
    assert_eq!(msg.message_type, AdexpMessageType::Change);
}

#[test]
fn test_dla_message() {
    let input = "-ADEXP
-TITLE DLA
-ARCID ABC123
";
    let msg = AdexpParser::parse_message(input).unwrap();
    assert_eq!(msg.message_type, AdexpMessageType::Delay);
}

#[test]
fn test_cnl_message() {
    let input = "-ADEXP
-TITLE CNL
-ARCID ABC123
";
    let msg = AdexpParser::parse_message(input).unwrap();
    assert_eq!(msg.message_type, AdexpMessageType::Cancel);
}

#[test]
fn test_dep_message() {
    let input = "-ADEXP
-TITLE DEP
-ARCID ABC123
";
    let msg = AdexpParser::parse_message(input).unwrap();
    assert_eq!(msg.message_type, AdexpMessageType::Departure);
}

#[test]
fn test_arr_message() {
    let input = "-ADEXP
-TITLE ARR
-ARCID ABC123
";
    let msg = AdexpParser::parse_message(input).unwrap();
    assert_eq!(msg.message_type, AdexpMessageType::Arrival);
}

#[test]
fn test_cof_message() {
    let input = "-ADEXP
-TITLE COF
-ARCID ABC123
";
    let msg = AdexpParser::parse_message(input).unwrap();
    assert_eq!(msg.message_type, AdexpMessageType::Coordination);
}

#[test]
fn test_req_message() {
    let input = "-ADEXP
-TITLE REQ
-ARCID ABC123
";
    let msg = AdexpParser::parse_message(input).unwrap();
    assert_eq!(msg.message_type, AdexpMessageType::Request);
}

#[test]
fn test_est_message() {
    let input = "-ADEXP
-TITLE EST
-ARCID ABC123
";
    let msg = AdexpParser::parse_message(input).unwrap();
    assert_eq!(msg.message_type, AdexpMessageType::Estimate);
}

#[test]
fn test_pos_message() {
    let input = "-ADEXP
-TITLE POS
-ARCID ABC123
";
    let msg = AdexpParser::parse_message(input).unwrap();
    assert_eq!(msg.message_type, AdexpMessageType::Position);
}

#[test]
fn test_chgdep_message() {
    let input = "-ADEXP
-TITLE CHGDEP
-ARCID ABC123
";
    let msg = AdexpParser::parse_message(input).unwrap();
    assert_eq!(msg.message_type, AdexpMessageType::Chgdep);
}

#[test]
fn test_generic_message() {
    let input = "-ADEXP
-TITLE UNKNOWN
-ARCID ABC123
";
    let msg = AdexpParser::parse_message(input).unwrap();
    assert_eq!(msg.message_type, AdexpMessageType::Generic);
}

