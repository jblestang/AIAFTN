use aftn::AftnParser;

/// Tests de robustesse - messages avec des caractères spéciaux
#[test]
fn test_special_characters() {
    // Message avec caractères spéciaux dans le corps
    let input = "GG LFPGYYYX LFPOYYYX 151230 TEST MESSAGE WITH SPECIAL CHARS: !@#$%^&*()";
    let result = AftnParser::parse_message(input);
    // Le parsing peut réussir ou échouer selon la grammaire, mais ne doit pas paniquer
    let _ = result;
}

/// Tests avec des espaces multiples
#[test]
fn test_multiple_spaces() {
    let input = "GG   LFPGYYYX   LFPOYYYX   151230   TEST   MESSAGE";
    let result = AftnParser::parse_message(input);
    // Devrait gérer les espaces multiples
    let _ = result;
}

/// Tests avec des tabulations
#[test]
fn test_tabs() {
    let input = "GG\tLFPGYYYX\tLFPOYYYX\t151230\tTEST\tMESSAGE";
    let result = AftnParser::parse_message(input);
    // Devrait gérer les tabulations
    let _ = result;
}

/// Tests avec des messages très courts
#[test]
fn test_very_short_messages() {
    let inputs = vec![
        "GG",
        "GG LFPG",
        "GG LFPGYYYX",
        "GG LFPGYYYX LFPO",
    ];
    
    for input in inputs {
        let result = AftnParser::parse_message(input);
        // Ces messages sont invalides mais ne doivent pas paniquer
        let _ = result;
    }
}

/// Tests avec des messages très longs
#[test]
fn test_very_long_messages() {
    let long_body = "A".repeat(1000);
    let input = format!("GG LFPGYYYX LFPOYYYX 151230 {}", long_body);
    let result = AftnParser::parse_message(&input);
    // Devrait gérer les messages longs
    let _ = result;
}

/// Tests avec des adresses invalides
#[test]
fn test_invalid_addresses() {
    let inputs = vec![
        "GG SHORT LFPOYYYX 151230 TEST",  // Adresse trop courte
        "GG LFPGYYYX TOOLONGADDRESS 151230 TEST",  // Adresse trop longue
        "GG 12345678 LFPOYYYX 151230 TEST",  // Adresse avec chiffres au début
    ];
    
    for input in inputs {
        let result = AftnParser::parse_message(input);
        // Le parsing peut réussir mais la validation devrait échouer
        if let Ok(msg) = result {
            let _ = msg.validate();  // Ne doit pas paniquer
        }
    }
}

/// Tests avec des dates/heures invalides
#[test]
fn test_invalid_datetimes() {
    let inputs = vec![
        "GG LFPGYYYX LFPOYYYX 321230 TEST",  // Jour invalide (32)
        "GG LFPGYYYX LFPOYYYX 152430 TEST",  // Heure invalide (24)
        "GG LFPGYYYX LFPOYYYX 151260 TEST",  // Minute invalide (60)
    ];
    
    for input in inputs {
        let result = AftnParser::parse_message(input);
        // Le parsing peut réussir mais la validation devrait échouer
        if let Ok(msg) = result {
            let _ = msg.validate();  // Ne doit pas paniquer
        }
    }
}

/// Tests avec des priorités invalides
#[test]
fn test_invalid_priorities() {
    let inputs = vec![
        "XX LFPGYYYX LFPOYYYX 151230 TEST",  // Priorité invalide
        "G LFPGYYYX LFPOYYYX 151230 TEST",   // Priorité trop courte
        "GGG LFPGYYYX LFPOYYYX 151230 TEST", // Priorité trop longue
    ];
    
    for input in inputs {
        let result = AftnParser::parse_message(input);
        // Le parsing peut échouer ou réussir, mais la validation devrait échouer
        if let Ok(msg) = result {
            let _ = msg.validate();  // Ne doit pas paniquer
        }
    }
}

/// Tests avec des caractères non-ASCII
#[test]
fn test_non_ascii() {
    // Les caractères non-ASCII dans le corps devraient être gérés
    let input = "GG LFPGYYYX LFPOYYYX 151230 TEST MESSAGE WITH ÉMOJIS 🛫🛬";
    let result = AftnParser::parse_message(input);
    // Le parsing peut échouer selon la grammaire, mais ne doit pas paniquer
    let _ = result;
}

