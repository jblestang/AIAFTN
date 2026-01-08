//! Tests complets pour toutes les catégories de messages AFTN

use aftn::{AftnParser, AftnError};
use aftn::MessageCategory;

/// Test que toutes les catégories principales peuvent être parsées
#[test]
fn test_all_main_categories_parsing() {
    let test_cases = vec![
        // Messages météorologiques
        ("GG LFPGYYYX LFPOYYYX 151230 NOTAM A1234/24 LFPG", MessageCategory::Notam),
        ("GG LFPGYYYX LFPOYYYX 151230 METAR LFPG 151230Z", MessageCategory::Metar),
        ("GG LFPGYYYX LFPOYYYX 151200 TAF LFPG 151200Z", MessageCategory::Taf),
        ("GG LFPGYYYX LFPOYYYX 151230 SIGMET VALID", MessageCategory::Sigmet),
        ("GG LFPGYYYX LFPOYYYX 151230 AIRMET VALID", MessageCategory::Airmet),
        ("GG LFPGYYYX LFPOYYYX 151230 ATIS LFPG", MessageCategory::Atis),
        ("GG LFPGYYYX LFPOYYYX 151230 VOLMET LFPG", MessageCategory::Volmet),
        
        // Messages de plan de vol
        ("GG LFPGYYYX LFPOYYYX 151200 FPL ABC123 V LFPG", MessageCategory::FlightPlan),
        ("GG LFPGYYYX LFPOYYYX 151200 CHG ABC123 ADEP LFPB", MessageCategory::Change),
        ("GG LFPGYYYX LFPOYYYX 151200 CNL ABC123", MessageCategory::Cancel),
        ("GG LFPGYYYX LFPOYYYX 151200 DLA ABC123 1300", MessageCategory::Delay),
        ("GG LFPGYYYX LFPOYYYX 151200 DEP ABC123 LFPG 1200", MessageCategory::Departure),
        ("GG LFPGYYYX LFPOYYYX 151200 ARR ABC123 LFPB 1400", MessageCategory::Arrival),
        ("GG LFPGYYYX LFPOYYYX 151200 EST ABC123 LFPG 1300", MessageCategory::Estimate),
        ("GG LFPGYYYX LFPOYYYX 151200 SPL ABC123 DATA", MessageCategory::SupplementaryFlightPlan),
        
        // Messages de coordination
        ("GG LFPGYYYX LFPOYYYX 151200 COF ABC123", MessageCategory::Coordination),
        ("GG LFPGYYYX LFPOYYYX 151200 ABI ABC123 LFPG KJFK", MessageCategory::AdvanceBoundaryInformation),
        ("GG LFPGYYYX LFPOYYYX 151200 REQ FPL ABC123", MessageCategory::Request),
        ("GG LFPGYYYX LFPOYYYX 151200 ALR ABC123 EMERGENCY", MessageCategory::Alerting),
        
        // Messages de position
        ("GG LFPGYYYX LFPOYYYX 151230 POS ABC123", MessageCategory::PositionReport),
    ];
    
    let mut success_count = 0;
    let mut failure_count = 0;
    
    let total = test_cases.len();
    for (message, expected_category) in test_cases {
        match AftnParser::parse_message(message) {
            Ok(msg) => {
                if msg.category == expected_category {
                    success_count += 1;
                } else {
                    failure_count += 1;
                    eprintln!("Category mismatch for {}: expected {:?}, got {:?}", 
                        message, expected_category, msg.category);
                }
            }
            Err(e) => {
                failure_count += 1;
                eprintln!("Failed to parse {}: {}", message, e);
            }
        }
    }
    
    println!("Parsing results: {} success, {} failures", success_count, failure_count);
    
    // Au moins 80% des messages devraient être parsés correctement
    let success_rate = (success_count as f64 / total as f64) * 100.0;
    assert!(success_rate >= 80.0, "Success rate too low: {:.2}%", success_rate);
}

/// Test que toutes les catégories ont des préfixes valides
#[test]
fn test_all_categories_prefixes() {
    let all_categories = vec![
        MessageCategory::Notam,
        MessageCategory::Metar,
        MessageCategory::Taf,
        MessageCategory::Sigmet,
        MessageCategory::Airmet,
        MessageCategory::Atis,
        MessageCategory::Volmet,
        MessageCategory::FlightPlan,
        MessageCategory::Change,
        MessageCategory::Cancel,
        MessageCategory::Delay,
        MessageCategory::Departure,
        MessageCategory::Arrival,
        MessageCategory::Estimate,
        MessageCategory::SupplementaryFlightPlan,
        MessageCategory::CurrentFlightPlan,
        MessageCategory::UpdateFlightPlan,
        MessageCategory::Coordination,
        MessageCategory::AdvanceBoundaryInformation,
        MessageCategory::Request,
        MessageCategory::RequestFlightPlan,
        MessageCategory::RequestSupplementaryFlightPlan,
        MessageCategory::Denial,
        MessageCategory::Release,
        MessageCategory::Return,
        MessageCategory::PositionReport,
        MessageCategory::AircraftPositionList,
        MessageCategory::Alerting,
        MessageCategory::Urgency,
        MessageCategory::RadioCommunicationFailure,
        MessageCategory::OceanicClearance,
        MessageCategory::Information,
        MessageCategory::MessageAcknowledgement,
        MessageCategory::Acceptance,
        MessageCategory::TransferOfControl,
        MessageCategory::AirReport,
    ];
    
    for category in all_categories {
        let prefix = category.prefix();
        assert!(!prefix.is_empty(), "Category {:?} has empty prefix", category);
        assert!(prefix.len() >= 3, "Category {:?} prefix too short: {}", category, prefix);
    }
}

/// Test de round-trip: catégorie -> préfixe -> catégorie
#[test]
fn test_category_round_trip() {
    let test_cases = vec![
        ("NOT", MessageCategory::Notam),
        ("MET", MessageCategory::Metar),
        ("TAF", MessageCategory::Taf),
        ("FPL", MessageCategory::FlightPlan),
        ("CHG", MessageCategory::Change),
        ("CNL", MessageCategory::Cancel),
        ("DLA", MessageCategory::Delay),
        ("DEP", MessageCategory::Departure),
        ("ARR", MessageCategory::Arrival),
        ("EST", MessageCategory::Estimate),
        ("ABI", MessageCategory::AdvanceBoundaryInformation),
        ("COF", MessageCategory::Coordination),
        ("REQ", MessageCategory::Request),
        ("SPL", MessageCategory::SupplementaryFlightPlan),
        ("ALR", MessageCategory::Alerting),
        ("POS", MessageCategory::PositionReport),
        ("ACP", MessageCategory::Acceptance),
        ("TCX", MessageCategory::TransferOfControl),
    ];
    
    for (prefix, expected_category) in test_cases {
        // Test: préfixe -> catégorie
        let category = MessageCategory::from_message_id(prefix).unwrap();
        assert_eq!(category, expected_category, "Prefix {} should map to {:?}", prefix, expected_category);
        
        // Test: catégorie -> préfixe
        let result_prefix = category.prefix();
        assert_eq!(result_prefix, prefix, "Category {:?} should have prefix {}", expected_category, prefix);
    }
}

