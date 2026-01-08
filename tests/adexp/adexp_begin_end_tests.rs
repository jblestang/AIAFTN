//! Tests pour les tableaux BEGIN/END dans ADEXP

use aftn::AdexpParser;

#[test]
fn test_parse_begin_end_rtepts_simple() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-BEGIN RTEPTS
-PT EDDF
-PT RID
-END RTEPTS
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse successfully");
    
    // Vérifier que la section RTEPTS existe
    let rtepts_section = message.get_section("RTEPTS");
    assert!(rtepts_section.is_some(), "RTEPTS section should exist");
    
    let rtepts = rtepts_section.unwrap();
    
    // Vérifier que les champs sont présents
    assert!(rtepts.fields.contains_key("PT"));
    
    // Vérifier qu'il y a 2 points
    let pt_values = rtepts.fields.get("PT").unwrap();
    assert_eq!(pt_values.len(), 2, "Should have 2 PT values");
    assert_eq!(pt_values[0], "EDDF");
    assert_eq!(pt_values[1], "RID");
}

#[test]
fn test_parse_begin_end_rtepts_with_multiple_fields() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-BEGIN RTEPTS
-PT EDDF -PTID EDDF -FL F004
-PT RID -PTID RID -FL F100
-END RTEPTS
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse successfully");
    
    let rtepts = message.get_section("RTEPTS").expect("RTEPTS section should exist");
    
    // Vérifier tous les champs
    assert!(rtepts.fields.contains_key("PT"));
    assert!(rtepts.fields.contains_key("PTID"));
    assert!(rtepts.fields.contains_key("FL"));
    
    let pt_values = rtepts.fields.get("PT").unwrap();
    assert_eq!(pt_values.len(), 2);
    
    let ptid_values = rtepts.fields.get("PTID").unwrap();
    assert_eq!(ptid_values.len(), 2);
    
    let fl_values = rtepts.fields.get("FL").unwrap();
    assert_eq!(fl_values.len(), 2);
    assert_eq!(fl_values[0], "F004");
    assert_eq!(fl_values[1], "F100");
}

#[test]
fn test_parse_begin_end_with_indentation() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-BEGIN RTEPTS
  -PT EDDF -PTID EDDF
  -PT RID -PTID RID
-END RTEPTS
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse successfully");
    
    let rtepts = message.get_section("RTEPTS").expect("RTEPTS section should exist");
    let pt_values = rtepts.fields.get("PT").unwrap();
    assert_eq!(pt_values.len(), 2);
}

#[test]
fn test_parse_begin_end_addr() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-BEGIN ADDR
-ADDR LFPGYYYX -FAC LFPG
-ADDR LFPBYYYX -FAC LFPB
-END ADDR
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse successfully");
    
    let addr = message.get_section("ADDR").expect("ADDR section should exist");
    
    assert!(addr.fields.contains_key("ADDR"));
    assert!(addr.fields.contains_key("FAC"));
    
    let addr_values = addr.fields.get("ADDR").unwrap();
    assert_eq!(addr_values.len(), 2);
}

#[test]
fn test_parse_multiple_begin_end_blocks() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-BEGIN ADDR
-ADDR LFPGYYYX
-END ADDR
-BEGIN RTEPTS
-PT LFPG -FL F004
-PT LFPB -FL F000
-END RTEPTS
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse successfully");
    
    // Vérifier que les deux sections existent
    assert!(message.get_section("ADDR").is_some());
    assert!(message.get_section("RTEPTS").is_some());
    
    let rtepts = message.get_section("RTEPTS").unwrap();
    let pt_values = rtepts.fields.get("PT").unwrap();
    assert_eq!(pt_values.len(), 2);
}

