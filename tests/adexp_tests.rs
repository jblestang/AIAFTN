//! Tests d'intégration pour ADEXP 3.4
//! 
//! Ce fichier regroupe tous les tests liés au format ADEXP

#[path = "adexp/adexp_actarr_test.rs"]
mod actarr_test;

#[path = "adexp/adexp_begin_end_tests.rs"]
mod begin_end_tests;

#[path = "adexp/adexp_chgdep_test.rs"]
mod chgdep_test;

#[path = "adexp/adexp_compound_fields_tests.rs"]
mod compound_fields_tests;

#[path = "adexp/adexp_fields_tests.rs"]
mod fields_tests;

#[path = "adexp/adexp_integration_tests.rs"]
mod integration_tests;

#[path = "adexp/adexp_missing_fields_test.rs"]
mod missing_fields_test;

#[path = "adexp/adexp_reserved_titles_test.rs"]
mod reserved_titles_test;

#[path = "adexp/adexp_robustness_tests.rs"]
mod robustness_tests;

#[path = "adexp/adexp_specification_example_test.rs"]
mod specification_example_test;

#[path = "adexp/adexp_type_tests.rs"]
mod type_tests;

#[path = "adexp/adexp_validation_extended_tests.rs"]
mod validation_extended_tests;

#[path = "adexp/adexp_validation_tests.rs"]
mod validation_tests;

