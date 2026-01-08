//! Catégories de messages AFTN et leurs implémentations

pub mod meteorological;
pub mod flight_plan;
pub mod coordination;
pub mod position;
pub mod alerting;
pub mod operational;

// Ré-exporter les types de messages
pub use meteorological::*;
pub use flight_plan::*;
pub use coordination::*;
pub use position::*;
pub use alerting::*;
pub use operational::*;

// Définir l'enum MessageCategory
mod category_enum;
pub use category_enum::MessageCategory;

