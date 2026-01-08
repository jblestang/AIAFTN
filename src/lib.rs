//! Bibliothèque pour le parsing de messages AFTN 3.4, ADEXP 3.4, NMEA 0183 et SBS (Mode-S/ADS-B)
//!
//! Cette bibliothèque fournit des parseurs pour quatre formats de messages aéronautiques :
//! - **AFTN 3.4** : Format de transmission réseau
//! - **ADEXP 3.4** : Format de présentation des données ATS
//! - **NMEA 0183** : Format de données de navigation
//! - **SBS (Mode-S/ADS-B)** : Format de données ADS-B

pub mod aftn;
pub mod adexp;
pub mod nmea;
pub mod sbs;

// Ré-exporter AFTN
pub use aftn::{AftnParser, AftnMessage, AftnError, MessageCategory};
// Ré-exporter le module submessages pour les tests
pub use aftn::submessages;

// Ré-exporter ADEXP
pub use adexp::{AdexpParser, AdexpMessage, AdexpError};
pub use adexp::types::MessageType as AdexpMessageType;

// Ré-exporter NMEA
pub use nmea::{NmeaParser, NmeaMessage, NmeaError};
pub use nmea::types::NmeaMessageType;
pub use nmea::{GgaMessage, RmcMessage, GsaMessage, VtgMessage};
pub use nmea::{AisMessage, AisDecodedData, AisMessageData};

// Ré-exporter SBS
pub use sbs::{SbsParser, SbsMessage, SbsError};
pub use sbs::types::SbsMessageType;

