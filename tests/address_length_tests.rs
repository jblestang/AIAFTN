use aftn::AftnParser;

/// Tests pour valider que le parser accepte les adresses de 7 et 8 caractères
#[test]
fn test_address_7_characters() {
    // Adresse de 7 caractères (LFPYYYX)
    let input = "GG LFPYYYX LFPOYYYX 181521 NOTAM A7002/24 LFPG RWY 09/27 CLOSED";
    let result = AftnParser::parse_message(input);
    assert!(result.is_ok(), "Should parse 7-character address");
    
    let message = result.unwrap();
    assert_eq!(message.addresses.origin, "LFPYYYX");
    assert!(message.addresses.origin.len() == 7);
    assert!(message.validate().is_ok(), "Should validate 7-character address");
}

#[test]
fn test_address_8_characters() {
    // Adresse de 8 caractères (LFPGYYYX)
    let input = "GG LFPGYYYX LFPOYYYX 181521 NOTAM A7002/24 LFPG RWY 09/27 CLOSED";
    let result = AftnParser::parse_message(input);
    assert!(result.is_ok(), "Should parse 8-character address");
    
    let message = result.unwrap();
    assert_eq!(message.addresses.origin, "LFPGYYYX");
    assert!(message.addresses.origin.len() == 8);
    assert!(message.validate().is_ok(), "Should validate 8-character address");
}

#[test]
fn test_mixed_address_lengths() {
    // Message avec adresses de longueurs différentes
    let input = "GG LFPYYYX LFPGYYYX LFPOYYYX 181521 NOTAM A7002/24 LFPG RWY 09/27 CLOSED";
    let result = AftnParser::parse_message(input);
    assert!(result.is_ok(), "Should parse mixed address lengths");
    
    let message = result.unwrap();
    assert_eq!(message.addresses.origin.len(), 7);
    assert!(message.addresses.destinations.iter().any(|d| d.len() == 8));
    assert!(message.validate().is_ok());
}

#[test]
fn test_invalid_address_lengths() {
    // Adresse trop courte (6 caractères)
    let input = "GG LFPYYY LFPOYYYX 181521 NOTAM A7002/24 LFPG RWY 09/27 CLOSED";
    let result = AftnParser::parse_message(input);
    // Le parsing peut réussir mais la validation devrait échouer
    if let Ok(msg) = result {
        assert!(msg.validate().is_err(), "Should reject 6-character address");
    }
    
    // Adresse trop longue (9 caractères)
    let input = "GG LFPGYYYXX LFPOYYYX 181521 NOTAM A7002/24 LFPG RWY 09/27 CLOSED";
    let result = AftnParser::parse_message(input);
    // Le parsing peut réussir mais la validation devrait échouer
    if let Ok(msg) = result {
        assert!(msg.validate().is_err(), "Should reject 9-character address");
    }
}

