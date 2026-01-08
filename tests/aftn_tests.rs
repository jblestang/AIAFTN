//! Tests d'intégration pour AFTN 3.4
//! 
//! Ce fichier regroupe tous les tests liés au format AFTN

#[path = "aftn/aftn_address_length_tests.rs"]
mod address_length_tests;

#[path = "aftn/aftn_fpl_database_tests.rs"]
mod aftn_fpl_database_tests;

#[path = "aftn/aftn_all_categories_tests.rs"]
mod all_categories_tests;

#[path = "aftn/aftn_analyze_failures.rs"]
mod analyze_failures;

#[path = "aftn/aftn_category_tests.rs"]
mod category_tests;

#[path = "aftn/aftn_comprehensive_category_tests.rs"]
mod comprehensive_category_tests;

#[path = "aftn/aftn_integration_tests.rs"]
mod integration_tests;

#[path = "aftn/aftn_large_dataset_tests.rs"]
mod large_dataset_tests;

#[path = "aftn/aftn_mesonet_tests.rs"]
mod mesonet_tests;

#[path = "aftn/aftn_missing_categories_test.rs"]
mod missing_categories_test;

#[path = "aftn/aftn_real_world_tests.rs"]
mod real_world_tests;

#[path = "aftn/aftn_robustness_tests.rs"]
mod robustness_tests;

#[path = "aftn/aftn_submessage_tests.rs"]
mod submessage_tests;

