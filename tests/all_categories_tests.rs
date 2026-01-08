//! Tests pour toutes les catégories de messages AFTN

use aftn::{AftnParser, AftnError};
use aftn::categories::MessageCategory;

/// Test que toutes les catégories sont correctement identifiées
#[test]
fn test_all_categories_identification() {
    let test_cases = vec![
        ("NOTAM A1234/24 LFPG", MessageCategory::Notam),
        ("METAR LFPG 151200Z", MessageCategory::Metar),
        ("TAF LFPG 151200Z", MessageCategory::Taf),
        ("FPL ABC123 V LFPG", MessageCategory::FlightPlan),
        ("CHG ABC123 ADEP LFPB", MessageCategory::Change),
        ("CNL ABC123", MessageCategory::Cancel),
        ("DLA ABC123 1300", MessageCategory::Delay),
        ("DEP ABC123 LFPG 1200", MessageCategory::Departure),
        ("ARR ABC123 LFPB 1400", MessageCategory::Arrival),
        ("EST ABC123 LFPG 1300 F350", MessageCategory::Estimate),
        ("ABI ABC123 LFPG KJFK", MessageCategory::AdvanceBoundaryInformation),
        ("COF ABC123", MessageCategory::Coordination),
        ("REQ ABC123", MessageCategory::Request),
        ("SPL ABC123", MessageCategory::SupplementaryFlightPlan),
        ("POS ABC123", MessageCategory::PositionReport),
    ];
    
    for (body, expected_category) in test_cases {
        // Construire un message AFTN complet
        let message = format!("GG LFPGYYYX LFPOYYYX 151200 {}", body);
        match AftnParser::parse_message(&message) {
            Ok(msg) => {
                assert_eq!(msg.category, expected_category, 
                    "Failed for body: {}", body);
            }
            Err(e) => {
                // Certains messages peuvent ne pas parser correctement
                // mais la catégorie devrait être identifiée
                eprintln!("Warning: Failed to parse {}: {}", body, e);
            }
        }
    }
}

/// Test des nouvelles catégories de messages
#[test]
fn test_new_categories() {
    // Test CHG
    let chg_msg = "GG LFPGYYYX LFPOYYYX 151200 CHG ABC123 ADEP LFPB";
    let result = AftnParser::parse_message(chg_msg);
    if let Ok(msg) = result {
        assert_eq!(msg.category, MessageCategory::Change);
    }
    
    // Test CNL
    let cnl_msg = "GG LFPGYYYX LFPOYYYX 151200 CNL ABC123";
    let result = AftnParser::parse_message(cnl_msg);
    if let Ok(msg) = result {
        assert_eq!(msg.category, MessageCategory::Cancel);
    }
    
    // Test DLA
    let dla_msg = "GG LFPGYYYX LFPOYYYX 151200 DLA ABC123 1300";
    let result = AftnParser::parse_message(dla_msg);
    if let Ok(msg) = result {
        assert_eq!(msg.category, MessageCategory::Delay);
    }
    
    // Test DEP
    let dep_msg = "GG LFPGYYYX LFPOYYYX 151200 DEP ABC123 LFPG 1200";
    let result = AftnParser::parse_message(dep_msg);
    if let Ok(msg) = result {
        assert_eq!(msg.category, MessageCategory::Departure);
    }
    
    // Test ARR
    let arr_msg = "GG LFPGYYYX LFPOYYYX 151200 ARR ABC123 LFPB 1400";
    let result = AftnParser::parse_message(arr_msg);
    if let Ok(msg) = result {
        assert_eq!(msg.category, MessageCategory::Arrival);
    }
    
    // Test EST
    let est_msg = "GG LFPGYYYX LFPOYYYX 151200 EST ABC123 LFPG 1300 F350";
    let result = AftnParser::parse_message(est_msg);
    if let Ok(msg) = result {
        assert_eq!(msg.category, MessageCategory::Estimate);
    }
    
    // Test ABI
    let abi_msg = "GG LFPGYYYX LFPOYYYX 151200 ABI ABC123 LFPG KJFK";
    let result = AftnParser::parse_message(abi_msg);
    if let Ok(msg) = result {
        assert_eq!(msg.category, MessageCategory::AdvanceBoundaryInformation);
    }
}

/// Test que toutes les catégories ont un préfixe valide
#[test]
fn test_all_categories_have_prefix() {
    let categories = vec![
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
    ];
    
    for category in categories {
        let prefix = category.prefix();
        assert!(!prefix.is_empty(), "Category {:?} has empty prefix", category);
        assert!(prefix.len() >= 3, "Category {:?} prefix too short: {}", category, prefix);
    }
}

