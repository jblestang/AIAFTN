use aftn::AftnParser;
use aftn::categories::MessageCategory;

/// Tests pour les messages NOTAM
#[test]
fn test_notam_variants() {
    let notam1 = "GG LFPGYYYX LFPOYYYX 151230 NOTAM A1234/24 LFPG RWY 09/27 CLOSED";
    let msg1 = AftnParser::parse_message(notam1).unwrap();
    assert_eq!(msg1.category, MessageCategory::Notam);
    
    let notam2 = "DD LFPGYYYX LFPOYYYX 151230 NOF A1234/24 LFPG RWY 09/27 CLOSED";
    let msg2 = AftnParser::parse_message(notam2).unwrap();
    assert_eq!(msg2.category, MessageCategory::Notam);
}

/// Tests pour les messages METAR
#[test]
fn test_metar_variants() {
    let metar1 = "GG LFPGYYYX LFPOYYYX 151230 METAR LFPG 151230Z 28015KT 9999 FEW030 12/08 Q1013";
    let msg1 = AftnParser::parse_message(metar1).unwrap();
    assert_eq!(msg1.category, MessageCategory::Metar);
    
    let metar2 = "DD LFPGYYYX LFPOYYYX 151230 METAR LFPG 151230Z AUTO 28015KT 9999 FEW030 12/08 Q1013 NOSIG";
    let msg2 = AftnParser::parse_message(metar2).unwrap();
    assert_eq!(msg2.category, MessageCategory::Metar);
}

/// Tests pour les messages TAF
#[test]
fn test_taf_variants() {
    let taf1 = "DD LFPGYYYX LFPOYYYX 151200 TAF LFPG 151200Z 1512/1612 28015KT 9999 FEW030";
    let msg1 = AftnParser::parse_message(taf1).unwrap();
    assert_eq!(msg1.category, MessageCategory::Taf);
    
    let taf2 = "FF LFPGYYYX LFPOYYYX 151200 TAF AMD LFPG 151200Z 1512/1612 28015KT 9999 FEW030 BECMG 1518/1520 30020G35KT";
    let msg2 = AftnParser::parse_message(taf2).unwrap();
    assert_eq!(msg2.category, MessageCategory::Taf);
}

/// Tests pour les messages SIGMET
#[test]
fn test_sigmet_message() {
    let sigmet = "GG LFPGYYYX LFPOYYYX 151230 SIGMET VALID 151230/152030 LFPG";
    let msg = AftnParser::parse_message(sigmet).unwrap();
    assert_eq!(msg.category, MessageCategory::Sigmet);
}

/// Tests pour les messages AIRMET
#[test]
fn test_airmet_message() {
    let airmet = "DD LFPGYYYX LFPOYYYX 151230 AIRMET VALID 151230/152030 LFPG";
    let msg = AftnParser::parse_message(airmet).unwrap();
    assert_eq!(msg.category, MessageCategory::Airmet);
}

/// Tests pour les plans de vol
#[test]
fn test_flight_plan_variants() {
    let fpl1 = "SS LFPGYYYX LFPOYYYX 151200 FPL ABC123 V LFPG 151200 LFPB 1800";
    let msg1 = AftnParser::parse_message(fpl1).unwrap();
    assert_eq!(msg1.category, MessageCategory::FlightPlan);
    
    let fpl2 = "KK LFPGYYYX LFPOYYYX 151200 FPL ABC123 IFR LFPG 151200 LFPB 1800";
    let msg2 = AftnParser::parse_message(fpl2).unwrap();
    assert_eq!(msg2.category, MessageCategory::FlightPlan);
}

/// Tests pour les rapports de position
#[test]
fn test_position_report() {
    let pos = "LL LFPGYYYX LFPOYYYX 151230 POS ABC123 151230 N48.5 E2.5 FL350";
    let msg = AftnParser::parse_message(pos).unwrap();
    assert_eq!(msg.category, MessageCategory::PositionReport);
}

/// Tests pour les messages opérationnels
#[test]
fn test_operational_messages() {
    // DEP est maintenant une catégorie spécifique
    let dep = "GG LFPGYYYX LFPOYYYX 151230 DEP ABC123 LFPG 151230";
    let msg_dep = AftnParser::parse_message(dep).unwrap();
    assert_eq!(msg_dep.category, MessageCategory::Departure);
    
    // Test avec un vrai message opérationnel générique
    let op1 = "GG LFPGYYYX LFPOYYYX 151230 STA ABC123 LFPG";
    let msg1 = AftnParser::parse_message(op1).unwrap();
    match msg1.category {
        MessageCategory::Operational(_) => {},
        _ => panic!("Expected operational message"),
    }
}

/// Tests pour les messages génériques
#[test]
fn test_generic_messages() {
    let gen = "GG LFPGYYYX LFPOYYYX 151230 GEN MESSAGE CONTENT";
    let msg = AftnParser::parse_message(gen).unwrap();
    assert_eq!(msg.category, MessageCategory::Generic);
}

