//! Tests d'intégration pour NMEA 0183

mod nmea {
    mod gps_tests;
    mod ais_tests;
    mod parser_tests;
    mod validation_tests;
    mod dysfunctional_tests;
    mod large_dataset_tests;
    mod robustness_tests;
    mod checksum_validation_tests;
    mod whitespace_tests;
}

