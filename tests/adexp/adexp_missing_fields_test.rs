//! Test pour identifier les champs ADEXP manquants
//! Compare avec une liste de référence des champs ADEXP 3.4

use aftn::adexp::fields::AdexpFields;

/// Liste de référence des champs ADEXP 3.4 basée sur la spécification EUROCONTROL
/// Cette liste est utilisée pour vérifier que tous les champs importants sont implémentés
const REFERENCE_FIELDS: &[&str] = &[
    // Champs primaires - Adresses et aérodromes
    "ADDR", "ADEP", "ADES", "ALTRNT1", "ALTRNT2",
    // Identification du vol
    "ARCID", "ARCTYP", "CEQPT", "REG", "SEL",
    // Route et navigation
    "ROUTE", "SID", "STAR", "ATSRT", "ARRPROC", "DEPPROC",
    // Temps - Estimated
    "EOBD", "EOBT", "ETO", "ETA", "EDA", "EET",
    // Temps - Actual
    "ATOT", "ATD", "ATAD", "ATOD", "ATOA", "ATOTD", "ATOTA",
    "ACTARR", "ACTDEP",
    // Temps - Autres
    "AMANTIME", "TOM", "FILTIM", "TTLEET", "TTO",
    // Niveaux de vol
    "RFL", "CFL", "AFL", "TFL",
    // Vitesse
    "SPEED", "GROUNDSPEED", "TAS", "MACH",
    // Météorologie
    "WINDIR", "WINDSPEED", "AIRTEMP", "QNH", "QFE",
    // Performance et navigation
    "PBN", "FLTRUL", "FLTTYP", "NAV", "COM", "DAT", "SUR",
    // Coordination et identification
    "IFPLID", "ORIGIN", "NETWORKTYPE", "FAC", "TITLE", "SRC",
    // Statut
    "CDMSTATUS", "IFPSDISCREPANCY", "CSTAT",
    // Référence
    "REFDATA",
    // Points de route
    "RTEPTS", "PT", "PTID", "FL", "SEQPT",
    // Champs additionnels
    "DEPAPTYPE", "DEPARCTYP", "OBTLIMIT", "OBT",
    "PRF1", "PRF2", "PRF3", "PRF4",
    "TRACKANGLE", "WKTRC", "MFX", "PTDLE", "CMLTSP", "VEC",
    // Champs réservés et additionnels
    "RELDIST", "HEXADDR", "OPR", "OPRICAO", "PER", "PERICAO",
    "ALTN", "ALTT", "ALTZ", "ALTA", "ALTS", "ALTR",
    "RMK", "RMKS", "COMMENT", "TEXT", "CODE", "CODEICAO",
    // Champs de base
    "GEO", "GEONAME", "LAT", "LON", "ALT", "ALTNZ", "DIST",
    "NUM", "TIMEHHMM", "TIMEHHMMSS", "DATE", "TIME",
    "REASON", "AHEAD", "STATREASON",
    "SENDER", "RECVR",
];

#[test]
fn test_all_reference_fields_are_valid() {
    let mut missing_fields = Vec::new();
    
    for field in REFERENCE_FIELDS {
        if !AdexpFields::is_valid_field(field) {
            missing_fields.push(*field);
        }
    }
    
    if !missing_fields.is_empty() {
        panic!(
            "Champs ADEXP manquants dans l'implémentation: {:?}\n\
            Ces champs sont définis dans la spécification ADEXP 3.4 mais ne sont pas reconnus.",
            missing_fields
        );
    }
}

#[test]
fn test_actual_time_fields() {
    // Vérifier que tous les champs "actual" sont présents
    let actual_fields = &["ACTARR", "ACTDEP", "ATOT", "ATD", "ATAD", "ATOD", "ATOA", "ATOTD", "ATOTA"];
    
    for field in actual_fields {
        assert!(
            AdexpFields::is_valid_field(field),
            "Le champ {} devrait être reconnu comme valide",
            field
        );
    }
}

#[test]
fn test_estimated_time_fields() {
    // Vérifier que tous les champs "estimated" sont présents
    let estimated_fields = &["EOBD", "EOBT", "ETO", "ETA", "EDA", "EET"];
    
    for field in estimated_fields {
        assert!(
            AdexpFields::is_valid_field(field),
            "Le champ {} devrait être reconnu comme valide",
            field
        );
    }
}

#[test]
fn test_flight_level_fields() {
    // Vérifier que tous les champs de niveau de vol sont présents
    let fl_fields = &["RFL", "CFL", "AFL", "TFL"];
    
    for field in fl_fields {
        assert!(
            AdexpFields::is_valid_field(field),
            "Le champ {} devrait être reconnu comme valide",
            field
        );
    }
}

