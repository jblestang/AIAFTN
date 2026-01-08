//! Tests pour tous les titres de messages réservés ADEXP 3.4
//! Basé sur l'Annexe C de la spécification EUROCONTROL

use aftn::AdexpParser;
use aftn::AdexpMessageType;
use aftn::adexp::types::MessageType;

/// Liste des titres de messages réservés selon ADEXP 3.4
/// Référence: Annex C de la spécification EUROCONTROL
const RESERVED_TITLES: &[(&str, &str, &str)] = &[
    ("CHGDEP", "Changed Departure", "France"),
    ("CNLARR", "Cancel Arrival", "France"),
    ("CNLDEP", "Cancel Departure", "France"),
    ("CONFIDM", "Configuration Operational", "France"),
    ("DEC", "Departure", "France"),
    ("EATARR", "Estimated Actual Time Arrival", "France"),
    ("ENDPROC", "End Procedure", "France"),
    ("ANM", "ATFM Notification Message", "NM"),
    ("CDAFTX", "Departure Clearance", "France"),
];

#[test]
fn test_all_reserved_titles_are_recognized() {
    for (title, _description, _reserved_by) in RESERVED_TITLES {
        let msg_type = MessageType::from_title(title).unwrap();
        assert_ne!(
            msg_type,
            MessageType::Generic,
            "Le titre réservé {} devrait être reconnu",
            title
        );
        
        // Vérifier que le préfixe correspond
        assert_eq!(
            msg_type.prefix(),
            *title,
            "Le préfixe pour {} devrait être {}",
            title,
            title
        );
    }
}

#[test]
fn test_chgdep_parsing() {
    let input = "-ADEXP
-TITLE CHGDEP
-ARCID ABC123
";
    let msg = AdexpParser::parse_message(input).unwrap();
    assert_eq!(msg.message_type, AdexpMessageType::ChangedDeparture);
}

#[test]
fn test_cnlarr_parsing() {
    let input = "-ADEXP
-TITLE CNLARR
-ARCID ABC123
";
    let msg = AdexpParser::parse_message(input).unwrap();
    assert_eq!(msg.message_type, AdexpMessageType::CancelArrival);
}

#[test]
fn test_cnldep_parsing() {
    let input = "-ADEXP
-TITLE CNLDEP
-ARCID ABC123
";
    let msg = AdexpParser::parse_message(input).unwrap();
    assert_eq!(msg.message_type, AdexpMessageType::CancelDeparture);
}

#[test]
fn test_confidm_parsing() {
    let input = "-ADEXP
-TITLE CONFIDM
-ARCID ABC123
";
    let msg = AdexpParser::parse_message(input).unwrap();
    assert_eq!(msg.message_type, AdexpMessageType::ConfigurationOperational);
}

#[test]
fn test_dec_parsing() {
    let input = "-ADEXP
-TITLE DEC
-ARCID ABC123
";
    let msg = AdexpParser::parse_message(input).unwrap();
    assert_eq!(msg.message_type, AdexpMessageType::DepartureDec);
}

#[test]
fn test_eatarr_parsing() {
    let input = "-ADEXP
-TITLE EATARR
-ARCID ABC123
";
    let msg = AdexpParser::parse_message(input).unwrap();
    assert_eq!(msg.message_type, AdexpMessageType::EstimatedActualTimeArrival);
}

#[test]
fn test_endproc_parsing() {
    let input = "-ADEXP
-TITLE ENDPROC
-ARCID ABC123
";
    let msg = AdexpParser::parse_message(input).unwrap();
    assert_eq!(msg.message_type, AdexpMessageType::EndProcedure);
}

#[test]
fn test_anm_parsing() {
    let input = "-ADEXP
-TITLE ANM
-ARCID ABC123
";
    let msg = AdexpParser::parse_message(input).unwrap();
    assert_eq!(msg.message_type, AdexpMessageType::AtfmNotificationMessage);
}

#[test]
fn test_cdaftx_parsing() {
    let input = "-ADEXP
-TITLE CDAFTX
-ARCID ABC123
";
    let msg = AdexpParser::parse_message(input).unwrap();
    assert_eq!(msg.message_type, AdexpMessageType::DepartureClearance);
}

#[test]
fn test_reserved_titles_case_insensitive() {
    for (title, _, _) in RESERVED_TITLES {
        let title_lower = title.to_lowercase();
        let msg_type_lower = MessageType::from_title(&title_lower).unwrap();
        let msg_type_upper = MessageType::from_title(title).unwrap();
        
        assert_eq!(
            msg_type_lower, msg_type_upper,
            "Le titre {} devrait être reconnu de manière case-insensitive",
            title
        );
    }
}

#[test]
fn test_reserved_titles_with_fields() {
    let test_cases = vec![
        ("CNLARR", "-ADES LFPB\n-ARCID ABC123\n"),
        ("CNLDEP", "-ADEP LFPG\n-ARCID ABC123\n"),
        ("EATARR", "-ADES LFPB\n-ETA 1400\n-ARCID ABC123\n"),
        ("DEC", "-ADEP LFPG\n-ACTDEP 1200\n-ARCID ABC123\n"),
    ];
    
    for (title, fields) in test_cases {
        let input = format!("-ADEXP\n-TITLE {}\n{}", title, fields);
        let msg = AdexpParser::parse_message(&input).unwrap();
        
        // Vérifier que le message est parsé correctement
        assert_ne!(msg.message_type, AdexpMessageType::Generic);
        
        // Vérifier que les champs sont accessibles
        let arcid = msg.get_field_value("", "ARCID").unwrap();
        assert!(arcid.is_some());
    }
}

