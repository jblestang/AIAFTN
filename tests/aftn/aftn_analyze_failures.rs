use aftn::AftnParser;
use std::fs;
use std::path::Path;
use std::collections::HashMap;

/// Analyse les messages qui échouent lors du parsing
#[test]
fn analyze_failed_messages() {
    let samples_dir = Path::new("tests/samples/large_dataset");
    
    if !samples_dir.exists() {
        eprintln!("Large dataset directory does not exist, skipping analysis");
        return;
    }
    
    let mut failed_messages = Vec::new();
    let mut error_counts: HashMap<String, usize> = HashMap::new();
    let mut total_checked = 0;
    
    let entries = fs::read_dir(samples_dir)
        .expect("Failed to read large dataset directory");
    
    for entry in entries {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("txt") {
            let content = match fs::read_to_string(&path) {
                Ok(c) => c,
                Err(_) => continue,
            };
            
            for (line_num, line) in content.lines().enumerate() {
                if line.trim().is_empty() {
                    continue;
                }
                
                total_checked += 1;
                
                match AftnParser::parse_message(line.trim()) {
                    Ok(msg) => {
                        // Vérifier aussi la validation
                        if let Err(e) = msg.validate() {
                            let error_key = format!("Validation: {}", e);
                            *error_counts.entry(error_key).or_insert(0) += 1;
                            failed_messages.push((path.file_name().unwrap().to_string_lossy().to_string(), 
                                                line_num + 1, 
                                                line.to_string(), 
                                                format!("Validation error: {}", e)));
                        }
                    }
                    Err(e) => {
                        let error_msg = format!("{}", e);
                        let error_key = error_msg.split('\n').next().unwrap_or(&error_msg).to_string();
                        *error_counts.entry(error_key.clone()).or_insert(0) += 1;
                        failed_messages.push((path.file_name().unwrap().to_string_lossy().to_string(), 
                                            line_num + 1, 
                                            line.to_string(), 
                                            error_msg));
                    }
                }
                
                // Limiter à 1000 échecs pour l'analyse
                if failed_messages.len() >= 1000 {
                    break;
                }
            }
            
            if failed_messages.len() >= 1000 {
                break;
            }
        }
    }
    
    println!("\n=== Analyse des échecs de parsing ===");
    println!("Total de messages vérifiés: {}", total_checked);
    println!("Messages échoués: {}", failed_messages.len());
    println!("\n=== Répartition des erreurs ===");
    
    let mut error_vec: Vec<_> = error_counts.iter().collect();
    error_vec.sort_by(|a, b| b.1.cmp(a.1));
    
    for (error, count) in error_vec.iter().take(20) {
        println!("  {}: {} occurrences", error, count);
    }
    
    println!("\n=== Exemples de messages échoués (premiers 50) ===");
    for (i, (file, line, msg, error)) in failed_messages.iter().take(50).enumerate() {
        println!("\n{}. Fichier: {}, Ligne: {}", i + 1, file, line);
        println!("   Message: {}", msg);
        println!("   Erreur: {}", error.lines().next().unwrap_or(error));
        println!("   Longueur: {} caractères", msg.len());
    }
    
    // Sauvegarder les messages échoués dans un fichier
    let failed_file = "tests/samples/failed_messages.txt";
    let mut failed_content = String::new();
    failed_content.push_str("=== Messages échoués lors du parsing ===\n\n");
    
    for (file, line, msg, error) in &failed_messages {
        failed_content.push_str(&format!("File: {}, Line: {}\n", file, line));
        failed_content.push_str(&format!("Message: {}\n", msg));
        failed_content.push_str(&format!("Error: {}\n", error.lines().next().unwrap_or(error)));
        failed_content.push_str("\n---\n\n");
    }
    
    fs::write(failed_file, failed_content)
        .expect("Failed to write failed messages file");
    
    println!("\nMessages échoués sauvegardés dans: {}", failed_file);
    
    // Analyser les patterns communs
    analyze_patterns(&failed_messages);
}

fn analyze_patterns(failed_messages: &[(String, usize, String, String)]) {
    println!("\n=== Analyse des patterns d'échec ===");
    
    let mut length_distribution: HashMap<usize, usize> = HashMap::new();
    let mut prefix_patterns: HashMap<String, usize> = HashMap::new();
    let mut missing_fields = Vec::new();
    
    for (_, _, msg, error) in failed_messages {
        // Distribution par longueur
        let len_bucket = (msg.len() / 50) * 50;
        *length_distribution.entry(len_bucket).or_insert(0) += 1;
        
        // Patterns de préfixe
        if msg.len() >= 10 {
            let prefix = &msg[..10.min(msg.len())];
            *prefix_patterns.entry(prefix.to_string()).or_insert(0) += 1;
        }
        
        // Identifier les champs manquants
        if error.contains("missing") || error.contains("Missing") {
            missing_fields.push((msg.clone(), error.clone()));
        }
    }
    
    println!("\nDistribution par longueur:");
    let mut len_vec: Vec<_> = length_distribution.iter().collect();
    len_vec.sort();
    for (len, count) in len_vec.iter().take(10) {
        println!("  {} chars: {} messages", len, count);
    }
    
    println!("\nPatterns de préfixe les plus fréquents:");
    let mut prefix_vec: Vec<_> = prefix_patterns.iter().collect();
    prefix_vec.sort_by(|a, b| b.1.cmp(a.1));
    for (prefix, count) in prefix_vec.iter().take(10) {
        println!("  '{}': {} occurrences", prefix, count);
    }
    
    if !missing_fields.is_empty() {
        println!("\nMessages avec champs manquants (premiers 10):");
        for (msg, error) in missing_fields.iter().take(10) {
            println!("  Message: {}", msg);
            println!("  Erreur: {}", error.lines().next().unwrap_or(error));
        }
    }
}

#[test]
fn test_specific_failed_messages() {
    // Tester des messages spécifiques qui ont échoué
    let failed_samples: Vec<String> = vec![
        // Ajouter ici des exemples de messages qui ont échoué
    ];
    
    for (i, msg) in failed_samples.iter().enumerate() {
        println!("\nTest message {}: {}", i + 1, msg);
        match AftnParser::parse_message(msg) {
            Ok(parsed) => {
                println!("  ✓ Parsé avec succès");
                if let Err(e) = parsed.validate() {
                    println!("  ✗ Validation échouée: {}", e);
                } else {
                    println!("  ✓ Validation réussie");
                }
            }
            Err(e) => {
                println!("  ✗ Parsing échoué: {}", e);
            }
        }
    }
}

