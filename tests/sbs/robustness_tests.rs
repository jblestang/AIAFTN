//! Tests de robustesse avec des datasets invalides SBS
//! Vérifie que le parser rejette correctement les messages invalides

use aftn::SbsParser;

/// Test que le parser rejette correctement un grand volume de messages invalides
/// Vérifie que le taux de rejet est élevé pour un dataset invalide
#[test]
#[ignore] // Ignorer par défaut car cela peut prendre du temps
fn test_reject_invalid_sbs_dataset() {
    let dataset_path = "tests/samples/sbs/sbs_invalid_dataset.txt";
    
    // Vérifier que le fichier existe
    if !std::path::Path::new(dataset_path).exists() {
        eprintln!("⚠ Fichier de dataset invalide non trouvé: {}", dataset_path);
        eprintln!("  Exécutez: ./scripts/download_sbs_nmea_data.sh pour générer les données");
        return;
    }
    
    let content = std::fs::read_to_string(dataset_path)
        .expect("Impossible de lire le fichier de dataset invalide");
    
    let lines: Vec<&str> = content.lines().collect();
    let total_messages = lines.len();
    
    assert!(total_messages >= 1000, "Le dataset invalide doit contenir au moins 1000 messages, trouvé: {}", total_messages);
    
    let mut parsed_count = 0;
    let mut rejected_count = 0;
    let mut validation_failed_count = 0;
    
    for (line_num, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        
        match SbsParser::parse_message(trimmed) {
            Ok(message) => {
                parsed_count += 1;
                // Même si le parsing réussit, la validation devrait échouer
                if let Err(_) = message.validate() {
                    validation_failed_count += 1;
                }
            }
            Err(_) => {
                rejected_count += 1;
            }
        }
        
        // Afficher la progression tous les 1000 messages
        if (line_num + 1) % 1000 == 0 {
            eprintln!("Progression: {}/{} messages traités", line_num + 1, total_messages);
        }
    }
    
    eprintln!("\n=== Résultats de robustesse ===");
    eprintln!("Total de messages invalides: {}", total_messages);
    eprintln!("Rejetés au parsing: {}", rejected_count);
    eprintln!("Parsés mais validation échouée: {}", validation_failed_count);
    eprintln!("Parsés avec succès: {}", parsed_count);
    
    // Calculer le taux de rejet (parsing ou validation)
    let total_rejected = rejected_count + validation_failed_count;
    let rejection_rate = (total_rejected as f64 / total_messages as f64) * 100.0;
    eprintln!("Taux de rejet: {:.2}%", rejection_rate);
    
    // Le taux de rejet doit être > 80% pour un dataset invalide
    // (certains messages peuvent être parsés mais invalides sémantiquement)
    assert!(
        rejection_rate >= 80.0,
        "Taux de rejet trop faible: {:.2}% (attendu >= 80% pour un dataset invalide)",
        rejection_rate
    );
    
    // Le nombre de messages parsés avec succès doit être faible
    let success_rate = (parsed_count as f64 / total_messages as f64) * 100.0;
    assert!(
        success_rate <= 20.0,
        "Trop de messages invalides parsés avec succès: {:.2}% (attendu <= 20%)",
        success_rate
    );
}

/// Test de catégories d'erreurs spécifiques
#[test]
#[ignore]
fn test_invalid_sbs_error_categories() {
    let dataset_path = "tests/samples/sbs/sbs_invalid_dataset.txt";
    
    if !std::path::Path::new(dataset_path).exists() {
        eprintln!("⚠ Fichier de dataset invalide non trouvé: {}", dataset_path);
        return;
    }
    
    let content = std::fs::read_to_string(dataset_path)
        .expect("Impossible de lire le fichier de dataset invalide");
    
    let lines: Vec<&str> = content.lines()
        .filter(|l| !l.trim().is_empty())
        .collect();
    
    use std::collections::HashMap;
    let mut error_categories: HashMap<String, usize> = HashMap::new();
    let mut parsed_count = 0;
    
    for line in &lines {
        match SbsParser::parse_message(line.trim()) {
            Ok(_) => {
                parsed_count += 1;
                error_categories.entry("parsed_but_invalid".to_string()).or_insert(0);
            }
            Err(e) => {
                let error_type = format!("{:?}", e);
                *error_categories.entry(error_type).or_insert(0) += 1;
            }
        }
    }
    
    eprintln!("\n=== Catégories d'erreurs ===");
    eprintln!("Total de messages: {}", lines.len());
    eprintln!("Parsés: {}", parsed_count);
    eprintln!("Erreurs par catégorie:");
    for (category, count) in &error_categories {
        let percentage = (*count as f64 / lines.len() as f64) * 100.0;
        eprintln!("  {}: {} ({:.2}%)", category, count, percentage);
    }
    
    // Vérifier qu'on a plusieurs types d'erreurs
    assert!(
        error_categories.len() >= 2,
        "Le dataset invalide doit générer au moins 2 types d'erreurs différents, trouvé: {}",
        error_categories.len()
    );
}

/// Test de performance avec messages invalides
/// Vérifie que le parser gère rapidement les erreurs
#[test]
#[ignore]
fn test_invalid_sbs_performance() {
    let dataset_path = "tests/samples/sbs/sbs_invalid_dataset.txt";
    
    if !std::path::Path::new(dataset_path).exists() {
        eprintln!("⚠ Fichier de dataset invalide non trouvé: {}", dataset_path);
        return;
    }
    
    let content = std::fs::read_to_string(dataset_path)
        .expect("Impossible de lire le fichier de dataset invalide");
    
    let lines: Vec<&str> = content.lines()
        .filter(|l| !l.trim().is_empty())
        .take(5000) // Limiter à 5K pour le test de performance
        .collect();
    
    let start = std::time::Instant::now();
    let mut rejected_count = 0;
    
    for line in &lines {
        if SbsParser::parse_message(line.trim()).is_err() {
            rejected_count += 1;
        }
    }
    
    let duration = start.elapsed();
    let messages_per_second = lines.len() as f64 / duration.as_secs_f64();
    
    eprintln!("\n=== Performance avec messages invalides ===");
    eprintln!("Messages traités: {}", lines.len());
    eprintln!("Rejetés: {}", rejected_count);
    eprintln!("Temps total: {:?}", duration);
    eprintln!("Messages/seconde: {:.2}", messages_per_second);
    
    // Le parser doit être capable de traiter au moins 1000 messages/seconde
    // même avec des erreurs
    assert!(
        messages_per_second >= 1000.0,
        "Performance insuffisante avec messages invalides: {:.2} msg/s (attendu >= 1000 msg/s)",
        messages_per_second
    );
}

