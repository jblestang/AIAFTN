//! Tests d'intégration pour ADEXP 3.4
//! 
//! Ce module regroupe tous les tests liés au format ADEXP :
//! - Tests de parsing des messages ADEXP
//! - Tests de validation des types de messages
//! - Tests des champs ADEXP
//! - Tests de validation sémantique
//! - Tests des structures composées
//! - Tests de robustesse

mod actarr_test;
mod begin_end_tests;
mod chgdep_test;
mod compound_fields_tests;
mod fields_tests;
mod integration_tests;
mod missing_fields_test;
mod reserved_titles_test;
mod robustness_tests;
mod specification_example_test;
mod type_tests;
mod validation_extended_tests;
mod validation_tests;

