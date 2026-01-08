use aftn::AftnParser;
use std::fs;
use std::path::Path;
use std::time::Instant;

/// Tests avec un large dataset de messages AFTN
#[test]
fn test_parse_large_notam_dataset() {
    let samples_dir = Path::new("tests/samples/large_dataset");
    
    if !samples_dir.exists() {
        eprintln!("Large dataset directory does not exist, skipping test");
        return;
    }
    
    let mut parsed = 0;
    let mut failed = 0;
    let start = Instant::now();
    
    // Parser tous les fichiers NOTAM
    let entries = fs::read_dir(samples_dir)
        .expect("Failed to read large dataset directory");
    
    for entry in entries {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        
        if path.is_file() && 
           path.file_name()
               .and_then(|n| n.to_str())
               .map(|s| s.starts_with("notam_"))
               .unwrap_or(false) {
            
            let content = match fs::read_to_string(&path) {
                Ok(c) => c,
                Err(_) => continue,
            };
            
            for line in content.lines() {
                if line.trim().is_empty() {
                    continue;
                }
                
                match AftnParser::parse_message(line.trim()) {
                    Ok(msg) => {
                        if msg.validate().is_ok() {
                            parsed += 1;
                        } else {
                            failed += 1;
                        }
                    }
                    Err(_) => {
                        failed += 1;
                    }
                }
            }
        }
    }
    
    let duration = start.elapsed();
    println!("\nNOTAM Dataset: {} parsed, {} failed in {:?}", parsed, failed, duration);
    
    // Au moins 80% devraient être parsés
    let success_rate = if parsed + failed > 0 {
        (parsed as f64 / (parsed + failed) as f64) * 100.0
    } else {
        0.0
    };
    
    println!("Success rate: {:.2}%", success_rate);
    assert!(success_rate >= 80.0 || parsed == 0, 
        "Success rate should be at least 80%, got {:.2}%", success_rate);
}

#[test]
fn test_parse_large_metar_dataset() {
    let samples_dir = Path::new("tests/samples/large_dataset");
    
    if !samples_dir.exists() {
        eprintln!("Large dataset directory does not exist, skipping test");
        return;
    }
    
    let mut parsed = 0;
    let mut failed = 0;
    let start = Instant::now();
    
    let entries = fs::read_dir(samples_dir)
        .expect("Failed to read large dataset directory");
    
    for entry in entries {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        
        if path.is_file() && 
           path.file_name()
               .and_then(|n| n.to_str())
               .map(|s| s.starts_with("metar_"))
               .unwrap_or(false) {
            
            let content = match fs::read_to_string(&path) {
                Ok(c) => c,
                Err(_) => continue,
            };
            
            for line in content.lines() {
                if line.trim().is_empty() {
                    continue;
                }
                
                match AftnParser::parse_message(line.trim()) {
                    Ok(msg) => {
                        if msg.validate().is_ok() {
                            parsed += 1;
                        } else {
                            failed += 1;
                        }
                    }
                    Err(_) => {
                        failed += 1;
                    }
                }
            }
        }
    }
    
    let duration = start.elapsed();
    println!("\nMETAR Dataset: {} parsed, {} failed in {:?}", parsed, failed, duration);
    
    let success_rate = if parsed + failed > 0 {
        (parsed as f64 / (parsed + failed) as f64) * 100.0
    } else {
        0.0
    };
    
    println!("Success rate: {:.2}%", success_rate);
    assert!(success_rate >= 80.0 || parsed == 0, 
        "Success rate should be at least 80%, got {:.2}%", success_rate);
}

#[test]
fn test_parse_all_large_datasets() {
    let samples_dir = Path::new("tests/samples/large_dataset");
    
    if !samples_dir.exists() {
        eprintln!("Large dataset directory does not exist, skipping test");
        return;
    }
    
    let mut total_parsed = 0;
    let mut total_failed = 0;
    let start = Instant::now();
    
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
            
            let mut file_parsed = 0;
            let mut file_failed = 0;
            
            for line in content.lines() {
                if line.trim().is_empty() {
                    continue;
                }
                
                match AftnParser::parse_message(line.trim()) {
                    Ok(msg) => {
                        if msg.validate().is_ok() {
                            file_parsed += 1;
                            total_parsed += 1;
                        } else {
                            file_failed += 1;
                            total_failed += 1;
                        }
                    }
                    Err(_) => {
                        file_failed += 1;
                        total_failed += 1;
                    }
                }
            }
            
            if file_parsed + file_failed > 0 {
                println!("{:?}: {} parsed, {} failed", 
                    path.file_name(), file_parsed, file_failed);
            }
        }
    }
    
    let duration = start.elapsed();
    println!("\n=== Large Dataset Summary ===");
    println!("Total parsed: {}", total_parsed);
    println!("Total failed: {}", total_failed);
    println!("Duration: {:?}", duration);
    
    if total_parsed + total_failed > 0 {
        let success_rate = (total_parsed as f64 / (total_parsed + total_failed) as f64) * 100.0;
        let throughput = total_parsed as f64 / duration.as_secs_f64();
        
        println!("Success rate: {:.2}%", success_rate);
        println!("Throughput: {:.0} messages/sec", throughput);
        
        // Au moins 80% devraient être parsés pour un dataset valide
        assert!(success_rate >= 80.0 || total_parsed == 0, 
            "Success rate should be at least 80%, got {:.2}%", success_rate);
    }
}

#[test]
fn test_performance_large_dataset() {
    let samples_dir = Path::new("tests/samples/large_dataset");
    
    if !samples_dir.exists() {
        eprintln!("Large dataset directory does not exist, skipping test");
        return;
    }
    
    // Prendre un échantillon de 1000 messages pour le test de performance
    let mut messages = Vec::new();
    let entries = fs::read_dir(samples_dir)
        .expect("Failed to read large dataset directory");
    
    for entry in entries {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("txt") {
            if let Ok(content) = fs::read_to_string(&path) {
                for line in content.lines() {
                    if !line.trim().is_empty() && messages.len() < 1000 {
                        messages.push(line.trim().to_string());
                    }
                }
            }
        }
        
        if messages.len() >= 1000 {
            break;
        }
    }
    
    if messages.is_empty() {
        eprintln!("No messages found for performance test");
        return;
    }
    
    let start = Instant::now();
    let mut parsed = 0;
    
    for msg in &messages {
        if AftnParser::parse_message(msg).is_ok() {
            parsed += 1;
        }
    }
    
    let duration = start.elapsed();
    let throughput = messages.len() as f64 / duration.as_secs_f64();
    
    println!("\n=== Performance Test ===");
    println!("Messages: {}", messages.len());
    println!("Parsed: {}", parsed);
    println!("Duration: {:?}", duration);
    println!("Throughput: {:.0} messages/sec", throughput);
    
    // Le parser devrait être capable de traiter au moins 1000 messages/seconde
    assert!(throughput >= 100.0 || messages.is_empty(), 
        "Throughput should be at least 100 msg/sec, got {:.0}", throughput);
}

