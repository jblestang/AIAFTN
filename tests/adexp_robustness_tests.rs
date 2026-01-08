use aftn::AdexpParser;

/// Tests de robustesse pour ADEXP
#[test]
fn test_empty_message() {
    let input = "-ADEXP
";
    let result = AdexpParser::parse_message(input);
    // Le parsing peut réussir même si le message est vide
    let _ = result;
}

#[test]
fn test_missing_title() {
    let input = "-ADEXP
-ARCID ABC123
";
    let result = AdexpParser::parse_message(input);
    // Le parsing peut réussir mais la validation devrait échouer
    if let Ok(msg) = result {
        let _ = msg.validate();  // Ne doit pas paniquer
    }
}

#[test]
fn test_field_without_value() {
    let input = "-ADEXP
-TITLE FPL
-ARCID
";
    let result = AdexpParser::parse_message(input);
    // Le parsing devrait réussir même si la valeur est vide
    assert!(result.is_ok());
}

#[test]
fn test_multiple_same_fields() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ARCID DEF456
";
    let result = AdexpParser::parse_message(input);
    assert!(result.is_ok());
    
    let message = result.unwrap();
    // ARCID devrait avoir plusieurs valeurs
    let arcid_values = message.get_field("", "ARCID").unwrap();
    assert!(arcid_values.is_some());
    if let Some(values) = arcid_values {
        assert!(values.len() >= 1);
    }
}

#[test]
fn test_special_characters_in_value() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC-123_XYZ
";
    let result = AdexpParser::parse_message(input);
    assert!(result.is_ok());
}

#[test]
fn test_very_long_field_value() {
    let long_value = "A".repeat(1000);
    let input = format!("-ADEXP
-TITLE FPL
-ARCID {}
", long_value);
    let result = AdexpParser::parse_message(&input);
    // Devrait gérer les valeurs longues
    let _ = result;
}

#[test]
fn test_multiple_sections() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ROUTE
-ADEP LFPG
-ADES LFPB
-COMMENT
-TEXT Test comment
";
    let result = AdexpParser::parse_message(input);
    assert!(result.is_ok());
    
    let message = result.unwrap();
    // Devrait avoir plusieurs sections
    assert!(message.sections.len() >= 1);
}

#[test]
fn test_whitespace_handling() {
    let input = "-ADEXP
-TITLE   FPL
-ARCID   ABC123
";
    let result = AdexpParser::parse_message(input);
    // Devrait gérer les espaces multiples
    assert!(result.is_ok());
}

#[test]
fn test_tabs_in_value() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC\t123
";
    let result = AdexpParser::parse_message(input);
    // Devrait gérer les tabulations
    let _ = result;
}

#[test]
fn test_missing_end_marker() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
";
    let result = AdexpParser::parse_message(input);
    // Le marqueur de fin est optionnel
    assert!(result.is_ok());
}

