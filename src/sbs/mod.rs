//! Module pour le parsing de messages SBS (Mode-S/ADS-B)
//!
//! SBS (Mode-S/ADS-B) est un format utilisé pour les données ADS-B
//! (Automatic Dependent Surveillance-Broadcast) transmises par les avions.
//!
//! Format typique:
//! MSG,type,transmission_type,session_id,aircraft_id,hex_ident,flight_id,date_gen,time_gen,date_log,time_log,callsign,altitude,speed,track,lat,lon,vertical_rate,squawk,alert,emergency,spi,is_on_ground

pub mod error;
pub mod message;
pub mod parser;
pub mod types;
pub mod validation;

pub use error::SbsError;
pub use message::SbsMessage;
pub use parser::SbsParser;
pub use types::SbsMessageType;

