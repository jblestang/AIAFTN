//! Module AFTN (Aeronautical Fixed Telecommunication Network) 3.4
//! 
//! Ce module contient tous les composants liés au format AFTN :
//! - Parser pour les messages AFTN
//! - Structures de données pour les messages
//! - Catégories de messages
//! - Sous-messages spécifiques par catégorie
//! - Gestion des erreurs

pub mod parser;
pub mod message;
pub mod error;
pub mod categories;
pub mod submessages;
pub mod validation;

pub use error::AftnError;
pub use message::AftnMessage;
pub use parser::AftnParser;
pub use categories::MessageCategory;

