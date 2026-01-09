//! Tests avec un grand volume de données NMEA 0183
//! Utilise des fichiers contenant > 10K messages pour tester la robustesse et les performances

use aftn::{NmeaParser, NmeaError};

/// Test de parsing d'un grand volume de messages NMEA
/// Vérifie que le parser peut gérer un grand nombre de messages sans erreur
#[test]
#[ignore] // Ignorer par défaut car cela peut prendre du temps
fn test_parse_large_nmea_dataset() {
    let dataset_path = "tests/samples/nmea/nmea_large_dataset.txt";
    
    // Vérifier que le fichier existe
    if !std::path::Path::new(dataset_path).exists() {
        eprintln!("⚠ Fichier de dataset non trouvé: {}", dataset_path);
        eprintln!("  Exécutez: ./scripts/download_sbs_nmea_data.sh pour générer les données");
        return;
    }
    
    let content = std::fs::read_to_string(dataset_path)
        .expect("Impossible de lire le fichier de dataset");
    
    let lines: Vec<&str> = content.lines().collect();
    let total_messages = lines.len();
    
    assert!(total_messages >= 10000, "Le dataset doit contenir au moins 10000 messages, trouvé: {}", total_messages);
    
    let mut parsed_count = 0;
    let mut error_count = 0;
    let mut errors = Vec::new();
    
    for (line_num, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        
        match NmeaParser::parse_message(trimmed) {
            Ok(message) => {
                parsed_count += 1;
                // Optionnel: valider le message parsé
                if let Err(e) = message.validate() {
                    errors.push(format!("Ligne {}: validation échouée: {}", line_num + 1, e));
                    error_count += 1;
                }
            }
            Err(e) => {
                errors.push(format!("Ligne {}: parsing échoué: {}", line_num + 1, e));
                error_count += 1;
            }
        }
        
        // Afficher la progression tous les 1000 messages
        if (line_num + 1) % 1000 == 0 {
            eprintln!("Progression: {}/{} messages traités", line_num + 1, total_messages);
        }
    }
    
    eprintln!("\n=== Résultats ===");
    eprintln!("Total de messages: {}", total_messages);
    eprintln!("Parsés avec succès: {}", parsed_count);
    eprintln!("Erreurs: {}", error_count);
    
    // Calculer le taux de succès
    let success_rate = (parsed_count as f64 / total_messages as f64) * 100.0;
    eprintln!("Taux de succès: {:.2}%", success_rate);
    
    // Le taux de succès doit être > 95% pour un dataset valide
    assert!(
        success_rate >= 95.0,
        "Taux de succès trop faible: {:.2}% (attendu >= 95%)",
        success_rate
    );
    
    // Afficher les premières erreurs si présentes
    if !errors.is_empty() {
        eprintln!("\nPremières erreurs (max 10):");
        for error in errors.iter().take(10) {
            eprintln!("  {}", error);
        }
    }
}

/// Test de parsing avec validation sémantique
#[test]
#[ignore]
fn test_parse_and_validate_large_nmea_dataset() {
    let dataset_path = "tests/samples/nmea/nmea_large_dataset.txt";
    
    if !std::path::Path::new(dataset_path).exists() {
        eprintln!("⚠ Fichier de dataset non trouvé: {}", dataset_path);
        return;
    }
    
    let content = std::fs::read_to_string(dataset_path)
        .expect("Impossible de lire le fichier de dataset");
    
    let lines: Vec<&str> = content.lines().collect();
    let mut validated_count = 0;
    let mut validation_error_count = 0;
    
    for (line_num, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        
        if let Ok(message) = NmeaParser::parse_message(trimmed) {
            match message.validate() {
                Ok(_) => validated_count += 1,
                Err(_) => validation_error_count += 1,
            }
        }
        
        if (line_num + 1) % 2000 == 0 {
            eprintln!("Validation: {}/{} messages traités", line_num + 1, lines.len());
        }
    }
    
    eprintln!("\n=== Résultats de validation ===");
    eprintln!("Messages validés: {}", validated_count);
    eprintln!("Erreurs de validation: {}", validation_error_count);
    
    // Au moins 90% des messages parsés doivent être valides
    let total_parsed = validated_count + validation_error_count;
    if total_parsed > 0 {
        let validation_rate = (validated_count as f64 / total_parsed as f64) * 100.0;
        eprintln!("Taux de validation: {:.2}%", validation_rate);
        assert!(
            validation_rate >= 90.0,
            "Taux de validation trop faible: {:.2}% (attendu >= 90%)",
            validation_rate
        );
    }
}

/// Test de performance - mesure le temps de parsing
#[test]
#[ignore]
fn test_nmea_parsing_performance() {
    let dataset_path = "tests/samples/nmea/nmea_large_dataset.txt";
    
    if !std::path::Path::new(dataset_path).exists() {
        eprintln!("⚠ Fichier de dataset non trouvé: {}", dataset_path);
        return;
    }
    
    let content = std::fs::read_to_string(dataset_path)
        .expect("Impossible de lire le fichier de dataset");
    
    let lines: Vec<&str> = content.lines()
        .filter(|l| !l.trim().is_empty())
        .take(10000) // Limiter à 10K pour le test de performance
        .collect();
    
    let start = std::time::Instant::now();
    let mut parsed_count = 0;
    
    for line in &lines {
        if NmeaParser::parse_message(line.trim()).is_ok() {
            parsed_count += 1;
        }
    }
    
    let duration = start.elapsed();
    let messages_per_second = parsed_count as f64 / duration.as_secs_f64();
    
    eprintln!("\n=== Performance ===");
    eprintln!("Messages parsés: {}", parsed_count);
    eprintln!("Temps total: {:?}", duration);
    eprintln!("Messages/seconde: {:.2}", messages_per_second);
    
    // Le parser doit être capable de parser au moins 1000 messages/seconde
    assert!(
        messages_per_second >= 1000.0,
        "Performance insuffisante: {:.2} msg/s (attendu >= 1000 msg/s)",
        messages_per_second
    );
}

/// Test de parsing de différents types de messages NMEA
#[test]
#[ignore]
fn test_nmea_message_type_distribution() {
    let dataset_path = "tests/samples/nmea/nmea_large_dataset.txt";
    
    if !std::path::Path::new(dataset_path).exists() {
        eprintln!("⚠ Fichier de dataset non trouvé: {}", dataset_path);
        return;
    }
    
    let content = std::fs::read_to_string(dataset_path)
        .expect("Impossible de lire le fichier de dataset");
    
    let lines: Vec<&str> = content.lines()
        .filter(|l| !l.trim().is_empty())
        .collect();
    
    use std::collections::HashMap;
    let mut type_counts: HashMap<String, usize> = HashMap::new();
    let mut parsed_count = 0;
    
    for line in &lines {
        if let Ok(message) = NmeaParser::parse_message(line.trim()) {
            let type_name = message.message_type.identifier().to_string();
            *type_counts.entry(type_name).or_insert(0) += 1;
            parsed_count += 1;
        }
    }
    
    eprintln!("\n=== Distribution des types de messages ===");
    eprintln!("Total parsé: {}", parsed_count);
    for (msg_type, count) in &type_counts {
        let percentage = (*count as f64 / parsed_count as f64) * 100.0;
        eprintln!("  {}: {} ({:.2}%)", msg_type, count, percentage);
    }
    
    // Vérifier qu'on a plusieurs types de messages
    assert!(
        type_counts.len() >= 3,
        "Le dataset doit contenir au moins 3 types de messages différents, trouvé: {}",
        type_counts.len()
    );
}

