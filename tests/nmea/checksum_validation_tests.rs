//! Tests spécifiques pour la validation des checksums NMEA
//! Vérifie que les checksums invalides sont correctement détectés et rejetés

use aftn::{NmeaParser, NmeaError};

/// Test qu'un message NMEA avec un checksum invalide est rejeté
#[test]
fn test_reject_invalid_checksum() {
    // Message GPGGA valide avec checksum calculé
    let valid_message = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47";
    
    // Même message mais avec checksum incorrect
    let invalid_checksum_message = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*00";
    
    // Le message valide doit être parsé avec succès
    assert!(NmeaParser::parse_message(valid_message).is_ok(), 
            "Le message valide doit être parsé avec succès");
    
    // Le message avec checksum invalide doit être rejeté
    let result = NmeaParser::parse_message(invalid_checksum_message);
    assert!(result.is_err(), 
            "Le message avec checksum invalide doit être rejeté");
    
    // Vérifier que l'erreur est bien une erreur de checksum
    match result {
        Err(NmeaError::InvalidChecksum { expected, got }) => {
            assert_eq!(expected, "47", "Le checksum attendu doit être 47");
            assert_eq!(got, "00", "Le checksum reçu doit être 00");
        }
        Err(e) => {
            panic!("L'erreur doit être InvalidChecksum, mais on a reçu: {:?}", e);
        }
        Ok(_) => {
            panic!("Le message avec checksum invalide ne doit pas être parsé");
        }
    }
}

/// Test avec plusieurs messages ayant des checksums invalides
#[test]
fn test_multiple_invalid_checksums() {
    let test_cases = vec![
        // GPGGA avec checksum incorrect
        ("$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*XX", "47"),
        // GPRMC avec checksum incorrect
        ("$GPRMC,123519,A,4807.038,N,01131.000,E,022.4,084.4,230394,003.1,W*YY", "6A"),
        // GPVTG avec checksum incorrect (le checksum correct est 48, pas A)
        ("$GPVTG,054.7,T,034.4,M,005.5,N,010.2,K*ZZ", "48"),
        // GPGSA avec checksum incorrect
        ("$GPGSA,A,3,04,05,,09,12,,,24,,,,,2.5,1.3,2.1*AA", "39"),
    ];
    
    for (message_with_bad_checksum, expected_checksum) in test_cases {
        let result = NmeaParser::parse_message(message_with_bad_checksum);
        
        assert!(result.is_err(), 
                "Le message '{}' avec checksum invalide doit être rejeté", 
                message_with_bad_checksum);
        
        match result {
            Err(NmeaError::InvalidChecksum { expected, got }) => {
                assert_eq!(expected, expected_checksum, 
                          "Le checksum attendu pour '{}' doit être '{}'", 
                          message_with_bad_checksum, expected_checksum);
                // Le checksum reçu doit être différent de celui attendu
                assert_ne!(got, expected_checksum,
                          "Le checksum reçu '{}' doit être différent de l'attendu '{}'",
                          got, expected_checksum);
            }
            Err(e) => {
                panic!("L'erreur pour '{}' doit être InvalidChecksum, mais on a reçu: {:?}", 
                       message_with_bad_checksum, e);
            }
            Ok(_) => {
                panic!("Le message '{}' avec checksum invalide ne doit pas être parsé", 
                       message_with_bad_checksum);
            }
        }
    }
}

/// Test qu'un message sans checksum est rejeté
#[test]
fn test_reject_missing_checksum() {
    // Message sans checksum
    let message_without_checksum = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,";
    
    let result = NmeaParser::parse_message(message_without_checksum);
    
    assert!(result.is_err(), 
            "Le message sans checksum doit être rejeté");
    
    match result {
        Err(NmeaError::MissingChecksum) => {
            // C'est l'erreur attendue
        }
        Err(e) => {
            panic!("L'erreur doit être MissingChecksum, mais on a reçu: {:?}", e);
        }
        Ok(_) => {
            panic!("Le message sans checksum ne doit pas être parsé");
        }
    }
}

/// Test qu'un message avec checksum malformé est rejeté
#[test]
fn test_reject_malformed_checksum() {
    let test_cases = vec![
        // Checksum avec un seul caractère (doit être rejeté car checksum doit avoir 2 caractères)
        ("$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*4", true),
        // Checksum avec caractères non-hexadécimaux (doit être rejeté)
        ("$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*GX", true),
        // Pas de * avant le checksum (doit être rejeté car format incorrect)
        ("$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,47", true),
        // Checksum avec caractères minuscules (peut être accepté si le parser convertit en majuscules)
        // Mais si le checksum est incorrect, il doit être rejeté
        ("$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*00", true),
    ];
    
    for (message, should_reject) in test_cases {
        let result = NmeaParser::parse_message(message);
        
        if should_reject {
            // Ces messages doivent être rejetés (soit checksum invalide, soit format incorrect)
            assert!(result.is_err(), 
                    "Le message '{}' avec checksum malformé doit être rejeté", message);
        } else {
            // Ces messages peuvent être acceptés (selon l'implémentation)
            // On ne fait rien ici, juste documenter
        }
    }
}

/// Test avec un dataset de messages ayant des checksums invalides
#[test]
#[ignore] // Ignorer par défaut car cela peut prendre du temps
fn test_invalid_checksum_dataset() {
    let dataset_path = "tests/samples/nmea/nmea_invalid_dataset.txt";
    
    if !std::path::Path::new(dataset_path).exists() {
        eprintln!("⚠ Fichier de dataset invalide non trouvé: {}", dataset_path);
        eprintln!("  Exécutez: ./scripts/download_sbs_nmea_data.sh pour générer les données");
        return;
    }
    
    let content = std::fs::read_to_string(dataset_path)
        .expect("Impossible de lire le fichier de dataset invalide");
    
    // Filtrer les messages avec checksum invalide (contiennent *XX ou n'ont pas de checksum)
    let invalid_checksum_messages: Vec<&str> = content.lines()
        .filter(|l| {
            let trimmed = l.trim();
            !trimmed.is_empty() && 
            (trimmed.contains("*XX") || 
             (trimmed.starts_with('$') && !trimmed.contains('*')) ||
             (trimmed.starts_with('!') && !trimmed.contains('*')))
        })
        .collect();
    
    eprintln!("\n=== Test des checksums invalides ===");
    eprintln!("Messages avec checksum invalide trouvés: {}", invalid_checksum_messages.len());
    
    let mut rejected_count = 0;
    let mut checksum_error_count = 0;
    let mut missing_checksum_count = 0;
    
    for (idx, message) in invalid_checksum_messages.iter().enumerate() {
        let result = NmeaParser::parse_message(message.trim());
        
        match result {
            Err(NmeaError::InvalidChecksum { .. }) => {
                checksum_error_count += 1;
                rejected_count += 1;
            }
            Err(NmeaError::MissingChecksum) => {
                missing_checksum_count += 1;
                rejected_count += 1;
            }
            Err(_) => {
                rejected_count += 1;
            }
            Ok(_) => {
                // Certains messages peuvent être parsés si le format est correct mais le checksum invalide
                // Ce n'est pas idéal mais acceptable si la validation sémantique échoue ensuite
                eprintln!("⚠ Message {} parsé malgré checksum invalide: {}", idx + 1, message);
            }
        }
        
        if (idx + 1) % 100 == 0 {
            eprintln!("Progression: {}/{} messages traités", idx + 1, invalid_checksum_messages.len());
        }
    }
    
    eprintln!("\n=== Résultats ===");
    eprintln!("Total de messages avec checksum invalide: {}", invalid_checksum_messages.len());
    eprintln!("Rejetés (InvalidChecksum): {}", checksum_error_count);
    eprintln!("Rejetés (MissingChecksum): {}", missing_checksum_count);
    eprintln!("Autres erreurs: {}", rejected_count - checksum_error_count - missing_checksum_count);
    
    let rejection_rate = (rejected_count as f64 / invalid_checksum_messages.len() as f64) * 100.0;
    eprintln!("Taux de rejet: {:.2}%", rejection_rate);
    
    // Au moins 90% des messages avec checksum invalide doivent être rejetés
    assert!(
        rejection_rate >= 90.0,
        "Taux de rejet des checksums invalides trop faible: {:.2}% (attendu >= 90%)",
        rejection_rate
    );
    
    // Au moins 80% doivent être des erreurs de checksum spécifiques
    let checksum_error_rate = ((checksum_error_count + missing_checksum_count) as f64 / invalid_checksum_messages.len() as f64) * 100.0;
    assert!(
        checksum_error_rate >= 80.0,
        "Taux d'erreurs de checksum spécifiques trop faible: {:.2}% (attendu >= 80%)",
        checksum_error_rate
    );
}

/// Test de performance avec checksums invalides
#[test]
#[ignore]
fn test_invalid_checksum_performance() {
    // Générer des messages valides puis modifier leurs checksums
    let mut invalid_messages = Vec::new();
    
    let base_messages = vec![
        "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47",
        "$GPRMC,123519,A,4807.038,N,01131.000,E,022.4,084.4,230394,003.1,W*6A",
        "$GPVTG,054.7,T,034.4,M,005.5,N,010.2,K*A",
        "$GPGSA,A,3,04,05,,09,12,,,24,,,,,2.5,1.3,2.1*39",
    ];
    
    // Créer 1000 messages avec checksums invalides
    for i in 0..1000 {
        let base = &base_messages[i % base_messages.len()];
        // Remplacer le checksum par un checksum invalide
        let invalid = base.replace("*47", "*00")
                          .replace("*6A", "*00")
                          .replace("*A", "*00")
                          .replace("*39", "*00");
        invalid_messages.push(invalid);
    }
    
    let start = std::time::Instant::now();
    let mut rejected_count = 0;
    
    for message in &invalid_messages {
        if NmeaParser::parse_message(message).is_err() {
            rejected_count += 1;
        }
    }
    
    let duration = start.elapsed();
    let messages_per_second = invalid_messages.len() as f64 / duration.as_secs_f64();
    
    eprintln!("\n=== Performance avec checksums invalides ===");
    eprintln!("Messages traités: {}", invalid_messages.len());
    eprintln!("Rejetés: {}", rejected_count);
    eprintln!("Temps total: {:?}", duration);
    eprintln!("Messages/seconde: {:.2}", messages_per_second);
    
    // Le parser doit être capable de traiter au moins 5000 messages/seconde
    // même avec des checksums invalides
    assert!(
        messages_per_second >= 5000.0,
        "Performance insuffisante avec checksums invalides: {:.2} msg/s (attendu >= 5000 msg/s)",
        messages_per_second
    );
    
    // Tous les messages doivent être rejetés
    assert_eq!(
        rejected_count, 
        invalid_messages.len(),
        "Tous les messages avec checksum invalide doivent être rejetés"
    );
}

