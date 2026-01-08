//! Tests complets pour tous les champs ADEXP selon la spécification 3.4

use aftn::AdexpParser;
use aftn::adexp::fields::AdexpFields;

/// Tests pour les champs primaires ADEXP
#[test]
fn test_primary_fields_validation() {
    // Champs d'adresse et aérodromes
    assert!(AdexpFields::is_primary_field("ADDR"));
    assert!(AdexpFields::is_primary_field("ADEP"));
    assert!(AdexpFields::is_primary_field("ADES"));
    assert!(AdexpFields::is_primary_field("ALTRNT1"));
    assert!(AdexpFields::is_primary_field("ALTRNT2"));
    
    // Identification du vol
    assert!(AdexpFields::is_primary_field("ARCID"));
    assert!(AdexpFields::is_primary_field("ARCTYP"));
    assert!(AdexpFields::is_primary_field("CEQPT"));
    assert!(AdexpFields::is_primary_field("REG"));
    assert!(AdexpFields::is_primary_field("SEL"));
    
    // Route et navigation
    assert!(AdexpFields::is_primary_field("ROUTE"));
    assert!(AdexpFields::is_primary_field("SID"));
    assert!(AdexpFields::is_primary_field("STAR"));
    assert!(AdexpFields::is_primary_field("ATSRT"));
    
    // Temps
    assert!(AdexpFields::is_primary_field("EOBD"));
    assert!(AdexpFields::is_primary_field("EOBT"));
    assert!(AdexpFields::is_primary_field("ETO"));
    assert!(AdexpFields::is_primary_field("ATOT"));
    assert!(AdexpFields::is_primary_field("ETA"));
    assert!(AdexpFields::is_primary_field("EDA"));
    
    // Niveaux de vol
    assert!(AdexpFields::is_primary_field("RFL"));
    assert!(AdexpFields::is_primary_field("CFL"));
    
    // Vitesse
    assert!(AdexpFields::is_primary_field("SPEED"));
    assert!(AdexpFields::is_primary_field("GROUNDSPEED"));
    
    // Performance
    assert!(AdexpFields::is_primary_field("PBN"));
    assert!(AdexpFields::is_primary_field("FLTRUL"));
    assert!(AdexpFields::is_primary_field("FLTTYP"));
    
    // Coordination
    assert!(AdexpFields::is_primary_field("IFPLID"));
    assert!(AdexpFields::is_primary_field("ORIGIN"));
    assert!(AdexpFields::is_primary_field("FAC"));
    assert!(AdexpFields::is_primary_field("TITLE"));
    
    // Champs invalides
    assert!(!AdexpFields::is_primary_field("INVALID"));
    assert!(!AdexpFields::is_primary_field(""));
}

/// Tests pour les champs de base ADEXP
#[test]
fn test_basic_fields_validation() {
    assert!(AdexpFields::is_basic_field("NUM"));
    assert!(AdexpFields::is_basic_field("PT"));
    assert!(AdexpFields::is_basic_field("TIMEHHMM"));
    assert!(AdexpFields::is_basic_field("TIMEHHMMSS"));
    assert!(AdexpFields::is_basic_field("DATE"));
    assert!(AdexpFields::is_basic_field("GEONAME"));
    assert!(AdexpFields::is_basic_field("LAT"));
    assert!(AdexpFields::is_basic_field("LON"));
    assert!(AdexpFields::is_basic_field("ALT"));
    assert!(AdexpFields::is_basic_field("DIST"));
    assert!(AdexpFields::is_basic_field("REASON"));
    assert!(AdexpFields::is_basic_field("TEXT"));
}

/// Tests pour les champs composés ADEXP
#[test]
fn test_compound_fields_validation() {
    assert!(AdexpFields::is_compound_field("ADDR"));
    assert!(AdexpFields::is_compound_field("REFDATA"));
    assert!(AdexpFields::is_compound_field("CSTAT"));
    assert!(AdexpFields::is_compound_field("VEC"));
    assert!(AdexpFields::is_compound_field("RTEPTS"));
}

/// Test de parsing avec tous les champs principaux d'un FPL
#[test]
fn test_complete_fpl_with_all_fields() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ARCTYP B738
-ADEP LFPG
-ADES LFPB
-ALTRNT1 LFPO
-ALTRNT2 LFRB
-EOBD 15
-EOBT 1200
-ETA 1400
-EDA 15
-ROUTE LFPG DCT LFPB
-SID RWY09L
-STAR ILS26R
-RFL 350
-CFL 360
-SPEED M080
-GROUNDSPEED 450
-PBN A1B1C1D1L1O1S1T1
-FLTRUL I
-FLTTYP S
-REG F-GABC
-SEL ABCD
-CEQPT SDFGHIJ2WXY
-WKTRC M
-IFPLID IFPL123456
-ORIGIN LFPG
-FAC LFPG
-NETWORKTYPE IFPS
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse successfully");
    
    assert_eq!(message.message_type, aftn::AdexpMessageType::FlightPlan);
    assert_eq!(message.get_field_value("", "ARCID").unwrap(), Some(&"ABC123".to_string()));
    assert_eq!(message.get_field_value("", "ARCTYP").unwrap(), Some(&"B738".to_string()));
    assert_eq!(message.get_field_value("", "ADEP").unwrap(), Some(&"LFPG".to_string()));
    assert_eq!(message.get_field_value("", "ADES").unwrap(), Some(&"LFPB".to_string()));
    assert_eq!(message.get_field_value("", "ALTRNT1").unwrap(), Some(&"LFPO".to_string()));
    assert_eq!(message.get_field_value("", "ALTRNT2").unwrap(), Some(&"LFRB".to_string()));
    assert_eq!(message.get_field_value("", "EOBD").unwrap(), Some(&"15".to_string()));
    assert_eq!(message.get_field_value("", "EOBT").unwrap(), Some(&"1200".to_string()));
    assert_eq!(message.get_field_value("", "RFL").unwrap(), Some(&"350".to_string()));
    assert_eq!(message.get_field_value("", "PBN").unwrap(), Some(&"A1B1C1D1L1O1S1T1".to_string()));
    assert_eq!(message.get_field_value("", "FLTRUL").unwrap(), Some(&"I".to_string()));
    assert_eq!(message.get_field_value("", "WKTRC").unwrap(), Some(&"M".to_string()));
}

/// Test avec des champs de temps multiples
#[test]
fn test_time_fields() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-EOBD 15
-EOBT 1200
-ETO 1300
-ATOT 1305
-ETA 1400
-EDA 15
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse successfully");
    
    assert_eq!(message.get_field_value("", "EOBD").unwrap(), Some(&"15".to_string()));
    assert_eq!(message.get_field_value("", "EOBT").unwrap(), Some(&"1200".to_string()));
    assert_eq!(message.get_field_value("", "ETO").unwrap(), Some(&"1300".to_string()));
    assert_eq!(message.get_field_value("", "ATOT").unwrap(), Some(&"1305".to_string()));
    assert_eq!(message.get_field_value("", "ETA").unwrap(), Some(&"1400".to_string()));
}

/// Test avec des champs géographiques
#[test]
fn test_geographic_fields() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-GEONAME PARIS
-LAT N490012
-LON E0023200
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse successfully");
    
    assert_eq!(message.get_field_value("", "GEONAME").unwrap(), Some(&"PARIS".to_string()));
    assert_eq!(message.get_field_value("", "LAT").unwrap(), Some(&"N490012".to_string()));
    assert_eq!(message.get_field_value("", "LON").unwrap(), Some(&"E0023200".to_string()));
}

/// Test avec des champs de route et navigation
#[test]
fn test_route_navigation_fields() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-ROUTE LFPG DCT LFPB
-SID RWY09L
-STAR ILS26R
-ATSRT UY123
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse successfully");
    
    assert_eq!(message.get_field_value("", "ROUTE").unwrap(), Some(&"LFPG DCT LFPB".to_string()));
    assert_eq!(message.get_field_value("", "SID").unwrap(), Some(&"RWY09L".to_string()));
    assert_eq!(message.get_field_value("", "STAR").unwrap(), Some(&"ILS26R".to_string()));
    assert_eq!(message.get_field_value("", "ATSRT").unwrap(), Some(&"UY123".to_string()));
}

/// Test avec des champs météorologiques
#[test]
fn test_meteorological_fields() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-WINDIR 270
-WINDSPEED 25
-AIRTEMP -10
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse successfully");
    
    assert_eq!(message.get_field_value("", "WINDIR").unwrap(), Some(&"270".to_string()));
    assert_eq!(message.get_field_value("", "WINDSPEED").unwrap(), Some(&"25".to_string()));
    assert_eq!(message.get_field_value("", "AIRTEMP").unwrap(), Some(&"-10".to_string()));
}

/// Test avec des champs de performance
#[test]
fn test_performance_fields() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-PBN A1B1C1D1L1O1S1T1
-FLTRUL I
-FLTTYP S
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse successfully");
    
    assert_eq!(message.get_field_value("", "PBN").unwrap(), Some(&"A1B1C1D1L1O1S1T1".to_string()));
    assert_eq!(message.get_field_value("", "FLTRUL").unwrap(), Some(&"I".to_string()));
    assert_eq!(message.get_field_value("", "FLTTYP").unwrap(), Some(&"S".to_string()));
}

/// Test avec des sections et champs composés
#[test]
fn test_sections_with_compound_fields() {
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-RTEPTS
-PT LFPG
-FL 350
-ETO 1200
-PT LFPB
-FL 360
-ETO 1400
";
    
    let message = AdexpParser::parse_message(input).expect("Should parse successfully");
    
    // Les champs dans la section RTEPTS devraient être accessibles
    assert!(message.get_field_value("RTEPTS", "PT").is_ok());
}

/// Test de validation des champs
#[test]
fn test_field_validation() {
    // Tous les champs primaires devraient être valides
    let primary_fields = vec![
        "ADDR", "ADEP", "ADES", "ALTRNT1", "ALTRNT2",
        "ARCID", "ARCTYP", "CEQPT", "REG", "SEL",
        "ROUTE", "SID", "STAR", "ATSRT",
        "EOBD", "EOBT", "ETO", "ATOT", "ETA", "EDA",
        "RFL", "CFL",
        "SPEED", "GROUNDSPEED",
        "PBN", "FLTRUL", "FLTTYP",
        "IFPLID", "ORIGIN", "FAC", "TITLE",
    ];
    
    for field in primary_fields {
        assert!(AdexpFields::is_valid_field(field), "Field {} should be valid", field);
    }
    
    // Tous les champs de base devraient être valides
    let basic_fields = vec![
        "NUM", "PT", "TIMEHHMM", "TIMEHHMMSS", "DATE",
        "GEONAME", "LAT", "LON",
        "ALT", "DIST", "REASON",
    ];
    
    for field in basic_fields {
        assert!(AdexpFields::is_valid_field(field), "Field {} should be valid", field);
    }
}

