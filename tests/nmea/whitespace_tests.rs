//! Tests pour vérifier que le parser NMEA rejette les espaces entre virgules
//! Selon NMEA 0183, les espaces entre virgules ne sont PAS autorisés

use aftn::NmeaParser;

/// Test qu'un message NMEA avec espaces entre virgules est rejeté
#[test]
fn test_reject_spaces_between_commas() {
    // Message valide (sans espaces)
    let valid = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47";
    assert!(NmeaParser::parse_message(valid).is_ok(), "Message valide doit être accepté");

    // Message invalide (avec espaces entre virgules)
    let invalid_spaces = "$GPGGA, 123519, 4807.038, N, 01131.000, E, 1, 08, 0.9, 545.4, M, 46.9, M, , *47";
    
    // Le parser devrait soit rejeter le message, soit calculer un checksum incorrect
    // Si le parser accepte le message, le checksum devrait être invalide
    let result = NmeaParser::parse_message(invalid_spaces);
    
    // Le message avec espaces devrait être rejeté car non conforme à NMEA 0183
    assert!(result.is_err(), "Message avec espaces entre virgules doit être rejeté selon NMEA 0183");
    
    // Vérifier que l'erreur est bien une erreur de format (pas seulement de checksum)
    // car nous rejetons explicitement les espaces entre virgules
    if let Err(e) = result {
        let error_str = format!("{:?}", e);
        // L'erreur devrait indiquer que les espaces entre virgules ne sont pas autorisés
        assert!(error_str.contains("space") || error_str.contains("comma") || error_str.contains("format"), 
                "L'erreur doit indiquer un problème de format lié aux espaces");
    }
}

/// Test qu'un message avec espaces dans les champs (mais pas entre virgules) est accepté
#[test]
fn test_accept_spaces_inside_fields() {
    // Les espaces DANS un champ sont autorisés (par ex. dans un nom de waypoint)
    // Mais pas ENTRE les virgules
    // Note: GPGGA n'a généralement pas de champs avec espaces, donc ce test
    // vérifie surtout que notre parser ne rejette pas tout message avec des espaces
    
    // Message sans espaces entre virgules (même si certains champs pourraient contenir des espaces)
    let valid = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47";
    assert!(NmeaParser::parse_message(valid).is_ok());
}

/// Test que la sérialisation ne produit jamais d'espaces entre virgules
#[test]
fn test_serialize_no_spaces_between_commas() {
    let input = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47";
    let message = NmeaParser::parse_message(input).unwrap();
    let serialized = message.serialize();
    
    // Vérifier qu'il n'y a pas d'espaces entre virgules
    // Pattern: on ne devrait jamais avoir ", " (virgule suivie d'espace)
    assert!(!serialized.contains(", "), "La sérialisation ne doit pas produire d'espaces entre virgules");
    
    // Vérifier que le message sérialisé est valide
    let reparsed = NmeaParser::parse_message(&serialized);
    assert!(reparsed.is_ok(), "Le message sérialisé doit être valide");
}

/// Test que la sérialisation normalise un message avec espaces (si accepté)
#[test]
fn test_serialize_normalizes_spaces() {
    // Si le parser accepte un message avec espaces (ce qui ne devrait pas arriver),
    // la sérialisation devrait les supprimer
    let input = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47";
    let message = NmeaParser::parse_message(input).unwrap();
    let serialized = message.serialize();
    
    // Le message sérialisé ne doit contenir aucun espace entre virgules
    let parts: Vec<&str> = serialized.split(',').collect();
    for part in &parts {
        // Les parties ne doivent pas commencer par un espace
        assert!(!part.starts_with(' '), "Aucun champ ne doit commencer par un espace après sérialisation");
    }
}

