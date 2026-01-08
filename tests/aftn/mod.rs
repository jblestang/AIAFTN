//! Tests d'intégration pour AFTN 3.4
//! 
//! Ce module regroupe tous les tests liés au format AFTN :
//! - Tests de parsing des messages AFTN
//! - Tests de validation des catégories
//! - Tests des sous-messages
//! - Tests de robustesse
//! - Tests avec données réelles

mod address_length_tests;
mod aftn_fpl_database_tests;
mod all_categories_tests;
mod analyze_failures;
mod category_tests;
mod comprehensive_category_tests;
mod integration_tests;
mod large_dataset_tests;
mod mesonet_tests;
mod missing_categories_test;
mod real_world_tests;
mod robustness_tests;
mod submessage_tests;

