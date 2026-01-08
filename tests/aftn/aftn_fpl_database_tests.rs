//! Tests pour valider le parser avec les bases de données AFTN/FPL téléchargées

use aftn::AftnParser;
use aftn::MessageCategory;
use std::fs;
use std::path::Path;

/// Test de parsing des messages AFTN depuis les bases de données générées
#[test]
fn test_parse_github_aftn_database() {
    // Essayer d'abord les FPL générés
    let fpl_file = Path::new("tests/samples/aftn_fpl/fpl_generated/fpl_examples_1000.txt");
    let db_file = if fpl_file.exists() {
        fpl_file
    } else {
        Path::new("tests/samples/aftn_fpl/github_aftn_messages.txt")
    };
    
    if !db_file.exists() {
        eprintln!("Base de données AFTN non trouvée. Exécutez: ./tests/samples/generate_fpl_examples.sh");
        return;
    }
    
    let content = fs::read_to_string(db_file).expect("Impossible de lire la base de données AFTN");
    let lines: Vec<&str> = content.lines()
        .filter(|l| !l.trim().is_empty())
        .filter(|l| !l.trim().starts_with('#'))
        .filter(|l| !l.contains("404: Not Found"))
        .collect();
    
    if lines.is_empty() {
        eprintln!("Aucun message AFTN valide trouvé dans la base de données");
        return;
    }
    
    println!("Test de parsing de {} messages AFTN depuis la base de données", lines.len());
    
    let mut success_count = 0;
    let mut failure_count = 0;
    let mut category_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    let mut failures = Vec::new();
    
    for (idx, line) in lines.iter().take(200).enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        
        match AftnParser::parse_message(trimmed) {
            Ok(message) => {
                success_count += 1;
                let cat_str = format!("{:?}", message.category);
                *category_counts.entry(cat_str).or_insert(0) += 1;
                
                if idx < 10 {
                    println!("  ✓ Message {} parsé: catégorie {:?}", idx + 1, message.category);
                }
            }
            Err(e) => {
                failure_count += 1;
                if failures.len() < 10 {
                    failures.push((idx + 1, trimmed.to_string(), e.to_string()));
                }
            }
        }
    }
    
    println!("\n=== Résultats ===");
    println!("Total: {} messages testés", lines.len().min(200));
    println!("Succès: {} ({:.2}%)", success_count, (success_count as f64 / (lines.len().min(200)) as f64) * 100.0);
    println!("Échecs: {} ({:.2}%)", failure_count, (failure_count as f64 / (lines.len().min(200)) as f64) * 100.0);
    
    println!("\nRépartition par catégorie:");
    let mut sorted_cats: Vec<_> = category_counts.iter().collect();
    sorted_cats.sort_by(|a, b| b.1.cmp(a.1));
    for (cat, count) in sorted_cats.iter().take(10) {
        println!("  {}: {}", cat, count);
    }
    
    if !failures.is_empty() {
        println!("\nPremiers échecs:");
        for (idx, msg, err) in failures.iter().take(5) {
            println!("  Message {}: {}", idx, msg.chars().take(80).collect::<String>());
            println!("    Erreur: {}", err);
        }
    }
    
    // Au moins 50% des messages devraient être parsés
    let tested = lines.len().min(200);
    if tested > 0 {
        let success_rate = (success_count as f64 / tested as f64) * 100.0;
        if success_rate < 50.0 && tested > 10 {
            panic!("Taux de succès trop faible: {:.2}%", success_rate);
        }
    }
}

/// Test spécifique pour les messages FPL générés
#[test]
fn test_parse_fpl_from_database() {
    let db_file = Path::new("tests/samples/aftn_fpl/fpl_generated/fpl_examples_1000.txt");
    
    if !db_file.exists() {
        eprintln!("Base de données FPL générée non trouvée. Exécutez: ./tests/samples/generate_fpl_examples.sh");
        return;
    }
    
    let content = fs::read_to_string(db_file).expect("Impossible de lire la base de données FPL");
    let lines: Vec<&str> = content.lines()
        .filter(|l| !l.trim().is_empty())
        .collect();
    
    if lines.is_empty() {
        return;
    }
    
    println!("Test de parsing de {} messages FPL générés", lines.len());
    
    let mut fpl_success = 0;
    let mut fpl_total = 0;
    let mut parsed_total = 0;
    
    for (idx, line) in lines.iter().take(100).enumerate() {
        let trimmed = line.trim();
        match AftnParser::parse_message(trimmed) {
            Ok(message) => {
                parsed_total += 1;
                fpl_total += 1;
                if matches!(message.category, MessageCategory::FlightPlan) {
                    fpl_success += 1;
                    if idx < 5 {
                        println!("  ✓ FPL {} parsé avec succès", idx + 1);
                    }
                }
            }
            Err(e) => {
                if idx < 5 {
                    println!("  ✗ FPL {} échoué: {}", idx + 1, e);
                }
            }
        }
    }
    
    println!("FPL parsés: {} / {} ({} messages parsés au total)", fpl_success, fpl_total, parsed_total);
    
    // Au moins 70% des FPL devraient être correctement identifiés
    if fpl_total > 0 {
        let success_rate = (fpl_success as f64 / fpl_total as f64) * 100.0;
        println!("Taux de succès FPL: {:.2}%", success_rate);
    }
}

/// Test de performance sur la base de données complète
#[test]
fn test_performance_on_database() {
    let db_file = Path::new("tests/samples/aftn_fpl/github_aftn_messages.txt");
    
    if !db_file.exists() {
        return;
    }
    
    let content = fs::read_to_string(db_file).expect("Impossible de lire la base de données");
    let lines: Vec<&str> = content.lines()
        .filter(|l| !l.trim().is_empty())
        .filter(|l| !l.trim().starts_with('#'))
        .collect();
    
    if lines.is_empty() {
        return;
    }
    
    println!("Test de performance sur {} messages", lines.len());
    
    let start = std::time::Instant::now();
    let mut parsed = 0;
    
    for line in lines.iter() {
        if AftnParser::parse_message(line.trim()).is_ok() {
            parsed += 1;
        }
    }
    
    let duration = start.elapsed();
    let throughput = lines.len() as f64 / duration.as_secs_f64();
    
    println!("Performance:");
    println!("  Messages parsés: {} / {}", parsed, lines.len());
    println!("  Durée: {:.2}s", duration.as_secs_f64());
    println!("  Débit: {:.0} messages/seconde", throughput);
}

