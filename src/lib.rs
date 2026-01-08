//! Bibliothèque pour le parsing de messages AFTN 3.4 et ADEXP 3.4
//!
//! Cette bibliothèque fournit des parseurs pour deux formats de messages aéronautiques :
//! - **AFTN 3.4** : Format de transmission réseau
//! - **ADEXP 3.4** : Format de présentation des données ATS

pub mod aftn;
pub mod adexp;

// Ré-exporter AFTN
pub use aftn::{AftnParser, AftnMessage, AftnError, MessageCategory};
// Ré-exporter le module submessages pour les tests
pub use aftn::submessages;

// Ré-exporter ADEXP
pub use adexp::{AdexpParser, AdexpMessage, AdexpError};
pub use adexp::types::MessageType as AdexpMessageType;

