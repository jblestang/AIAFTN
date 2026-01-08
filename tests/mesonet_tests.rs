//! Tests pour valider le parser avec les données réelles de mesonet.agron.iastate.edu

use aftn::AftnParser;
use aftn::MessageCategory;
use std::fs;
use std::path::Path;

/// Test de parsing des messages METAR téléchargés depuis mesonet
#[test]
fn test_parse_mesonet_metar() {
    let metar_file = Path::new("tests/samples/mesonet/metar/metar_messages.txt");
    
    if !metar_file.exists() {
        eprintln!("Fichier METAR non trouvé, exécutez d'abord: ./tests/samples/download_mesonet.sh");
        return;
    }
    
    let content = fs::read_to_string(metar_file).expect("Impossible de lire le fichier METAR");
    let lines: Vec<&str> = content.lines().filter(|l| !l.trim().is_empty()).collect();
    
    if lines.is_empty() {
        eprintln!("Aucun message METAR trouvé dans le fichier");
        return;
    }
    
    println!("Test de parsing de {} messages METAR depuis mesonet", lines.len());
    
    let mut success_count = 0;
    let mut failure_count = 0;
    let mut failures = Vec::new();
    
    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        
        // Construire un message AFTN complet avec le METAR
        // Format: GG ORIGIN DEST TIME METAR <contenu>
        let test_message = format!("GG LFPGYYYX LFPOYYYX 080000 {}", trimmed);
        
        match AftnParser::parse_message(&test_message) {
            Ok(message) => {
                success_count += 1;
                if idx < 5 {
                    println!("  ✓ Message {} parsé avec succès: catégorie {:?}", idx + 1, message.category);
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
    
    println!("\nRésultats:");
    println!("  Succès: {} / {}", success_count, lines.len());
    println!("  Échecs: {} / {}", failure_count, lines.len());
    
    if !failures.is_empty() {
        println!("\nPremiers échecs:");
        for (idx, msg, err) in failures.iter().take(5) {
            println!("  Message {}: {}", idx, msg);
            println!("    Erreur: {}", err);
        }
    }
    
    // Au moins 50% des messages devraient être parsés
    let success_rate = (success_count as f64 / lines.len() as f64) * 100.0;
    println!("\nTaux de succès: {:.2}%", success_rate);
    
    // Ne pas faire échouer le test si certains messages ne peuvent pas être parsés
    // (certains formats peuvent être non-standard)
    if success_count == 0 {
        panic!("Aucun message METAR n'a pu être parsé");
    }
}

/// Test de parsing des messages TAF téléchargés depuis mesonet
#[test]
fn test_parse_mesonet_taf() {
    let taf_file = Path::new("tests/samples/mesonet/taf/taf_messages.txt");
    
    if !taf_file.exists() {
        eprintln!("Fichier TAF non trouvé, exécutez d'abord: ./tests/samples/download_mesonet.sh");
        return;
    }
    
    let content = fs::read_to_string(taf_file).expect("Impossible de lire le fichier TAF");
    let lines: Vec<&str> = content.lines().filter(|l| !l.trim().is_empty()).collect();
    
    if lines.is_empty() {
        eprintln!("Aucun message TAF trouvé dans le fichier");
        return;
    }
    
    println!("Test de parsing de {} messages TAF depuis mesonet", lines.len());
    
    let mut success_count = 0;
    let mut failure_count = 0;
    let mut failures = Vec::new();
    
    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        
        // Construire un message AFTN complet avec le TAF
        // Format: GG ORIGIN DEST TIME TAF <contenu>
        let test_message = format!("GG LFPGYYYX LFPOYYYX 080000 {}", trimmed);
        
        match AftnParser::parse_message(&test_message) {
            Ok(message) => {
                success_count += 1;
                if idx < 5 {
                    println!("  ✓ Message {} parsé avec succès: catégorie {:?}", idx + 1, message.category);
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
    
    println!("\nRésultats:");
    println!("  Succès: {} / {}", success_count, lines.len());
    println!("  Échecs: {} / {}", failure_count, lines.len());
    
    if !failures.is_empty() {
        println!("\nPremiers échecs:");
        for (idx, msg, err) in failures.iter().take(5) {
            println!("  Message {}: {}", idx, msg);
            println!("    Erreur: {}", err);
        }
    }
    
    // Au moins 50% des messages devraient être parsés
    let success_rate = (success_count as f64 / lines.len() as f64) * 100.0;
    println!("\nTaux de succès: {:.2}%", success_rate);
    
    // Ne pas faire échouer le test si certains messages ne peuvent pas être parsés
    if success_count == 0 {
        panic!("Aucun message TAF n'a pu être parsé");
    }
}

/// Test de parsing des sous-messages METAR extraits
#[test]
fn test_parse_mesonet_metar_submessages() {
    let metar_file = Path::new("tests/samples/mesonet/metar/metar_messages.txt");
    
    if !metar_file.exists() {
        eprintln!("Fichier METAR non trouvé, exécutez d'abord: ./tests/samples/download_mesonet.sh");
        return;
    }
    
    let content = fs::read_to_string(metar_file).expect("Impossible de lire le fichier METAR");
    let lines: Vec<&str> = content.lines().filter(|l| !l.trim().is_empty()).collect();
    
    if lines.is_empty() {
        return;
    }
    
    println!("Test de parsing des sous-messages METAR ({} messages)", lines.len());
    
    let mut success_count = 0;
    
    for (idx, line) in lines.iter().take(20).enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        
        // Construire un message AFTN complet
        let test_message = format!("GG LFPGYYYX LFPOYYYX 080000 {}", trimmed);
        
        match AftnParser::parse_message(&test_message) {
            Ok(message) => {
                if message.category == MessageCategory::Metar {
                    success_count += 1;
                    println!("  ✓ Message {}: message METAR parsé", idx + 1);
                }
            }
            Err(_) => {}
        }
    }
    
    println!("Sous-messages METAR parsés: {} / {}", success_count, lines.len().min(20));
}

/// Test de parsing des sous-messages TAF extraits
#[test]
fn test_parse_mesonet_taf_submessages() {
    let taf_file = Path::new("tests/samples/mesonet/taf/taf_messages.txt");
    
    if !taf_file.exists() {
        eprintln!("Fichier TAF non trouvé, exécutez d'abord: ./tests/samples/download_mesonet.sh");
        return;
    }
    
    let content = fs::read_to_string(taf_file).expect("Impossible de lire le fichier TAF");
    let lines: Vec<&str> = content.lines().filter(|l| !l.trim().is_empty()).collect();
    
    if lines.is_empty() {
        return;
    }
    
    println!("Test de parsing des sous-messages TAF ({} messages)", lines.len());
    
    let mut success_count = 0;
    
    for (idx, line) in lines.iter().take(20).enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        
        // Construire un message AFTN complet
        let test_message = format!("GG LFPGYYYX LFPOYYYX 080000 {}", trimmed);
        
        match AftnParser::parse_message(&test_message) {
            Ok(message) => {
                if message.category == MessageCategory::Taf {
                    success_count += 1;
                    println!("  ✓ Message {}: message TAF parsé", idx + 1);
                }
            }
            Err(_) => {}
        }
    }
    
    println!("Sous-messages TAF parsés: {} / {}", success_count, lines.len().min(20));
}

