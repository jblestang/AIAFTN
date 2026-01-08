//! Module pour le parsing de messages NMEA 0183
//!
//! NMEA 0183 est un protocole de communication standard utilisé pour
//! la transmission de données de navigation entre équipements maritimes et aéronautiques.
//!
//! Format typique: $MESSAGE_TYPE,field1,field2,...,fieldN*CHECKSUM
//!
//! Supporte:
//! - Messages GPS (GPGGA, GPRMC, GPGSA, GPVTG, etc.)
//! - Messages AIS (AIVDM, AIVDO) avec décodage 6-bit

pub mod error;
pub mod message;
pub mod parser;
pub mod types;
pub mod validation;
pub mod gps;
pub mod ais;

pub use error::NmeaError;
pub use message::NmeaMessage;
pub use parser::NmeaParser;
pub use types::NmeaMessageType;
pub use gps::{GgaMessage, RmcMessage, GsaMessage, VtgMessage};
pub use ais::{AisMessage, AisDecodedData, AisMessageData};

