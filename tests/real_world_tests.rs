use aftn::AftnParser;
use aftn::categories::MessageCategory;
use std::fs;
use std::path::Path;

/// Tests avec des exemples de messages AFTN réels
#[test]
fn test_real_notam_messages() {
    let samples = vec![
        "tests/samples/notam_example1.txt",
        "tests/samples/notam_example2.txt",
    ];
    
    for sample_path in samples {
        if Path::new(sample_path).exists() {
            let content = fs::read_to_string(sample_path)
                .expect(&format!("Failed to read {}", sample_path));
            
            let result = AftnParser::parse_message(&content.trim());
            assert!(result.is_ok(), 
                "Failed to parse {}: {:?}", sample_path, result);
            
            let message = result.unwrap();
            assert_eq!(message.category, MessageCategory::Notam,
                "Message from {} should be NOTAM", sample_path);
            assert!(!message.body.is_empty(),
                "Message body should not be empty");
        }
    }
}

#[test]
fn test_real_metar_messages() {
    let samples = vec![
        "tests/samples/metar_example1.txt",
        "tests/samples/metar_example2.txt",
    ];
    
    for sample_path in samples {
        if Path::new(sample_path).exists() {
            let content = fs::read_to_string(sample_path)
                .expect(&format!("Failed to read {}", sample_path));
            
            let result = AftnParser::parse_message(&content.trim());
            assert!(result.is_ok(), 
                "Failed to parse {}: {:?}", sample_path, result);
            
            let message = result.unwrap();
            assert_eq!(message.category, MessageCategory::Metar,
                "Message from {} should be METAR", sample_path);
        }
    }
}

#[test]
fn test_real_taf_messages() {
    let samples = vec![
        "tests/samples/taf_example1.txt",
        "tests/samples/taf_example2.txt",
    ];
    
    for sample_path in samples {
        if Path::new(sample_path).exists() {
            let content = fs::read_to_string(sample_path)
                .expect(&format!("Failed to read {}", sample_path));
            
            let result = AftnParser::parse_message(&content.trim());
            assert!(result.is_ok(), 
                "Failed to parse {}: {:?}", sample_path, result);
            
            let message = result.unwrap();
            assert_eq!(message.category, MessageCategory::Taf,
                "Message from {} should be TAF", sample_path);
        }
    }
}

#[test]
fn test_real_fpl_messages() {
    let sample_path = "tests/samples/fpl_example1.txt";
    
    if Path::new(sample_path).exists() {
        let content = fs::read_to_string(sample_path)
            .expect(&format!("Failed to read {}", sample_path));
        
        let result = AftnParser::parse_message(&content.trim());
        assert!(result.is_ok(), 
            "Failed to parse {}: {:?}", sample_path, result);
        
        let message = result.unwrap();
        assert_eq!(message.category, MessageCategory::FlightPlan,
            "Message from {} should be FPL", sample_path);
    }
}

#[test]
fn test_real_messages_with_multiple_destinations() {
    let sample_path = "tests/samples/multiple_dest.txt";
    
    if Path::new(sample_path).exists() {
        let content = fs::read_to_string(sample_path)
            .expect(&format!("Failed to read {}", sample_path));
        
        let result = AftnParser::parse_message(&content.trim());
        assert!(result.is_ok(), 
            "Failed to parse {}: {:?}", sample_path, result);
        
        let message = result.unwrap();
        assert!(message.addresses.destinations.len() >= 2,
            "Message should have multiple destinations");
    }
}

#[test]
fn test_real_messages_with_sequence() {
    let sample_path = "tests/samples/with_sequence.txt";
    
    if Path::new(sample_path).exists() {
        let content = fs::read_to_string(sample_path)
            .expect(&format!("Failed to read {}", sample_path));
        
        let result = AftnParser::parse_message(&content.trim());
        assert!(result.is_ok(), 
            "Failed to parse {}: {:?}", sample_path, result);
        
        let message = result.unwrap();
        assert!(message.sequence_number.is_some(),
            "Message should have a sequence number");
    }
}

/// Test de validation de tous les exemples réels
#[test]
fn test_validate_all_real_samples() {
    let samples_dir = Path::new("tests/samples");
    
    if !samples_dir.exists() {
        eprintln!("Samples directory does not exist, skipping test");
        return;
    }
    
    let entries = fs::read_dir(samples_dir)
        .expect("Failed to read samples directory");
    
    let mut parsed_count = 0;
    let mut failed_count = 0;
    
    for entry in entries {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("txt") {
            let content = fs::read_to_string(&path)
                .expect(&format!("Failed to read {:?}", path));
            
            match AftnParser::parse_message(&content.trim()) {
                Ok(message) => {
                    // Valider le message
                    match message.validate() {
                        Ok(_) => {
                            parsed_count += 1;
                            println!("✓ Parsed and validated: {:?}", path.file_name());
                        }
                        Err(e) => {
                            failed_count += 1;
                            eprintln!("✗ Validation failed for {:?}: {}", path.file_name(), e);
                        }
                    }
                }
                Err(e) => {
                    failed_count += 1;
                    eprintln!("✗ Parse failed for {:?}: {}", path.file_name(), e);
                }
            }
        }
    }
    
    println!("\nSummary: {} parsed successfully, {} failed", parsed_count, failed_count);
    
    // Au moins quelques messages devraient être parsés
    assert!(parsed_count > 0, "At least one sample should be parsed successfully");
}

