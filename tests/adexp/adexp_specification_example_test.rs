//! Test basé sur l'exemple de l'Annexe F de la spécification ADEXP 3.4
//! Cet exemple illustre un message IFPL (Initial Flight Plan) structuré selon le format ADEXP

use aftn::AdexpParser;
use aftn::adexp::types::MessageType;

#[test]
fn test_adexp_3_4_specification_example_ifpl() {
    // Exemple tiré de l'Annexe F de la spécification ADEXP 3.4
    // Message IFPL avec bloc BEGIN ADDR contenant plusieurs entrées FAC
    let input = "-ADEXP
-TITLE IFPL
-BEGIN ADDR
-FAC LIIRZEZX
-FAC CFMUTACT
-FAC EDDAYGCD
-FAC LSASFPLS
-FAC LSAZZQZX
-FAC LSAZZQZG
-FAC LIPPZQZX
-FAC LIIRZPZM
-FAC LIPPZEZA
-FAC EDDZYNYS
-FAC LYZZEDXX
-FAC EDDFYQYX
-FAC EDGGZQZA
-FAC LYZZEEXX
-FAC EDDAYGLZ
-FAC EDDXYIYT
-FAC LAAAZQZX
-FAC LDZOZQZY
-FAC LDZOZQZX
-FAC LDZOZQZQ
-FAC EDUUZQZA
-FAC LGMDZQZI
-FAC LGGGZQZQ
-FAC LGGGYKYX
-FAC LGTSZTZX
-FAC LGTSZAZX
-FAC LGGGZQZB
-FAC LGTSZPZX
-FAC LYZZCGXX
-FAC LYZZEBXX
-END ADDR
-ADEP EDDF
-ADES LGTS
";

    let message = AdexpParser::parse_message(input).expect("Should parse successfully");
    
    // Vérifier le type de message
    assert_eq!(message.message_type, MessageType::Ifpl);
    
    // Vérifier que la section ADDR existe
    let addr_section = message.get_section("ADDR");
    assert!(addr_section.is_some(), "ADDR section should exist");
    
    let addr = addr_section.unwrap();
    
    // Vérifier que le champ FAC existe
    assert!(addr.fields.contains_key("FAC"), "FAC field should exist in ADDR section");
    
    // Vérifier qu'il y a 30 entrées FAC (comptées dans l'exemple)
    let fac_values = addr.fields.get("FAC").unwrap();
    assert_eq!(fac_values.len(), 30, "Should have 30 FAC entries");
    
    // Vérifier quelques valeurs spécifiques
    assert!(fac_values.contains(&"LIIRZEZX".to_string()), "Should contain LIIRZEZX");
    assert!(fac_values.contains(&"EDDFYQYX".to_string()), "Should contain EDDFYQYX");
    assert!(fac_values.contains(&"LGTSZTZX".to_string()), "Should contain LGTSZTZX");
    assert!(fac_values.contains(&"LYZZEBXX".to_string()), "Should contain LYZZEBXX");
    
    // Vérifier les champs ADEP et ADES
    let adep = message.get_field_value("", "ADEP").expect("Should get ADEP");
    assert_eq!(adep, Some(&"EDDF".to_string()));
    
    let ades = message.get_field_value("", "ADES").expect("Should get ADES");
    assert_eq!(ades, Some(&"LGTS".to_string()));
}

#[test]
fn test_adexp_3_4_specification_example_fpl_complete() {
    // Exemple plus complet basé sur la spécification ADEXP 3.4
    // Message FPL avec plusieurs sections et champs
    let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ARCTYP B738
-ADEP LFPG
-ADES LFPB
-EOBD 15
-EOBT 1200
-RFL F350
-SPEED M082
-ROUTE LFPG DCT LFPB
-BEGIN RTEPTS
-PT LFPG -PTID LFPG -FL F004 -ETO 1200
-PT LFPB -PTID LFPB -FL F000 -ETO 1300
-END RTEPTS
";

    let message = AdexpParser::parse_message(input).expect("Should parse successfully");
    
    // Vérifier le type de message
    assert_eq!(message.message_type, MessageType::FlightPlan);
    
    // Vérifier les champs principaux
    assert_eq!(message.get_field_value("", "ARCID").unwrap(), Some(&"ABC123".to_string()));
    assert_eq!(message.get_field_value("", "ARCTYP").unwrap(), Some(&"B738".to_string()));
    assert_eq!(message.get_field_value("", "ADEP").unwrap(), Some(&"LFPG".to_string()));
    assert_eq!(message.get_field_value("", "ADES").unwrap(), Some(&"LFPB".to_string()));
    assert_eq!(message.get_field_value("", "EOBD").unwrap(), Some(&"15".to_string()));
    assert_eq!(message.get_field_value("", "EOBT").unwrap(), Some(&"1200".to_string()));
    assert_eq!(message.get_field_value("", "RFL").unwrap(), Some(&"F350".to_string()));
    assert_eq!(message.get_field_value("", "SPEED").unwrap(), Some(&"M082".to_string()));
    assert_eq!(message.get_field_value("", "ROUTE").unwrap(), Some(&"LFPG DCT LFPB".to_string()));
    
    // Vérifier la section RTEPTS
    let rtepts = message.get_section("RTEPTS").expect("RTEPTS section should exist");
    assert!(rtepts.fields.contains_key("PT"));
    assert!(rtepts.fields.contains_key("PTID"));
    assert!(rtepts.fields.contains_key("FL"));
    assert!(rtepts.fields.contains_key("ETO"));
    
    let pt_values = rtepts.fields.get("PT").unwrap();
    assert_eq!(pt_values.len(), 2);
    assert_eq!(pt_values[0], "LFPG");
    assert_eq!(pt_values[1], "LFPB");
    
    let fl_values = rtepts.fields.get("FL").unwrap();
    assert_eq!(fl_values.len(), 2);
    assert_eq!(fl_values[0], "F004");
    assert_eq!(fl_values[1], "F000");
}

#[test]
fn test_adexp_3_4_specification_example_with_compound_fields() {
    // Exemple avec des champs composés complexes selon ADEXP 3.4
    let input = "-ADEXP
-TITLE FPL
-ARCID TEST01
-ADEP EDDF
-ADES LGTS
-BEGIN ADDR
-ADDR LFPGYYYX -FAC LFPG
-ADDR LFPBYYYX -FAC LFPB
-END ADDR
-BEGIN VEC
-TRACKANGLE 180
-GROUNDSPEED 450
-ALT F350
-END VEC
-BEGIN REFDATA
-IFPLID ABC123DEF456
-ORIGIN EDDF
-FAC LFPG
-NETWORKTYPE IFPS
-END REFDATA
";

    let message = AdexpParser::parse_message(input).expect("Should parse successfully");
    
    // Vérifier la section ADDR
    let addr = message.get_section("ADDR").expect("ADDR section should exist");
    assert!(addr.fields.contains_key("ADDR"));
    assert!(addr.fields.contains_key("FAC"));
    let addr_values = addr.fields.get("ADDR").unwrap();
    assert_eq!(addr_values.len(), 2);
    
    // Vérifier la section VEC
    let vec = message.get_section("VEC").expect("VEC section should exist");
    assert!(vec.fields.contains_key("TRACKANGLE"));
    assert!(vec.fields.contains_key("GROUNDSPEED"));
    assert!(vec.fields.contains_key("ALT"));
    assert_eq!(vec.fields.get("TRACKANGLE").unwrap()[0], "180");
    assert_eq!(vec.fields.get("GROUNDSPEED").unwrap()[0], "450");
    assert_eq!(vec.fields.get("ALT").unwrap()[0], "F350");
    
    // Vérifier la section REFDATA
    let refdata = message.get_section("REFDATA").expect("REFDATA section should exist");
    assert!(refdata.fields.contains_key("IFPLID"));
    assert!(refdata.fields.contains_key("ORIGIN"));
    assert!(refdata.fields.contains_key("FAC"));
    assert!(refdata.fields.contains_key("NETWORKTYPE"));
    assert_eq!(refdata.fields.get("IFPLID").unwrap()[0], "ABC123DEF456");
    assert_eq!(refdata.fields.get("ORIGIN").unwrap()[0], "EDDF");
    assert_eq!(refdata.fields.get("NETWORKTYPE").unwrap()[0], "IFPS");
}

