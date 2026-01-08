//! Test pour vérifier qu'il n'y a pas de catégories manquantes

use aftn::categories::MessageCategory;

/// Test que toutes les catégories importantes sont implémentées
#[test]
fn test_all_important_categories_implemented() {
    // Liste des catégories standard AFTN 3.4
    let standard_categories = vec![
        // Messages météorologiques
        ("NOT", MessageCategory::Notam),
        ("MET", MessageCategory::Metar),
        ("TAF", MessageCategory::Taf),
        ("SIG", MessageCategory::Sigmet),
        ("AIR", MessageCategory::Airmet),
        ("ATI", MessageCategory::Atis),
        ("VOL", MessageCategory::Volmet),
        
        // Messages de plan de vol
        ("FPL", MessageCategory::FlightPlan),
        ("CHG", MessageCategory::Change),
        ("CNL", MessageCategory::Cancel),
        ("DLA", MessageCategory::Delay),
        ("DEP", MessageCategory::Departure),
        ("ARR", MessageCategory::Arrival),
        ("EST", MessageCategory::Estimate),
        ("SPL", MessageCategory::SupplementaryFlightPlan),
        ("CPL", MessageCategory::CurrentFlightPlan),
        ("UPL", MessageCategory::UpdateFlightPlan),
        
        // Messages de coordination
        ("COF", MessageCategory::Coordination),
        ("CDN", MessageCategory::Coordination),
        ("ABI", MessageCategory::AdvanceBoundaryInformation),
        ("REQ", MessageCategory::Request),
        ("RQP", MessageCategory::RequestFlightPlan),
        ("RQS", MessageCategory::RequestSupplementaryFlightPlan),
        ("DEN", MessageCategory::Denial),
        ("RLS", MessageCategory::Release),
        ("RTN", MessageCategory::Return),
        ("ACP", MessageCategory::Acceptance),
        ("TCX", MessageCategory::TransferOfControl),
        
        // Messages de position
        ("POS", MessageCategory::PositionReport),
        ("APL", MessageCategory::AircraftPositionList),
        
        // Messages d'alerte
        ("ALR", MessageCategory::Alerting),
        ("URG", MessageCategory::Urgency),
        ("RCF", MessageCategory::RadioCommunicationFailure),
        
        // Messages spéciaux
        ("OCL", MessageCategory::OceanicClearance),
        ("INF", MessageCategory::Information),
        ("MAC", MessageCategory::MessageAcknowledgement),
    ];
    
    for (prefix, expected_category) in standard_categories {
        let category = MessageCategory::from_message_id(prefix).unwrap();
        assert_eq!(category, expected_category, 
            "Prefix {} should map to {:?}", prefix, expected_category);
    }
    
    // Test spécial pour AIREP (5 caractères)
    let airep_category = MessageCategory::from_message_id("AIREP").unwrap();
    assert_eq!(airep_category, MessageCategory::AirReport);
}

