pub mod notam;
pub mod metar;
pub mod taf;
pub mod sigmet;
pub mod airmet;
pub mod atis;
pub mod volmet;
pub mod fpl;
pub mod pos;
pub mod operational;
pub mod generic;

pub use notam::NotamMessage;
pub use metar::MetarMessage;
pub use taf::TafMessage;
pub use sigmet::SigmetMessage;
pub use airmet::AirmetMessage;
pub use atis::AtisMessage;
pub use volmet::VolmetMessage;
pub use fpl::FplMessage;
pub use pos::PosMessage;
pub use operational::OperationalMessage;
pub use generic::GenericMessage;

use crate::categories::MessageCategory;
use crate::error::AftnError;

/// Trait pour les sous-messages AFTN
pub trait SubMessage: std::fmt::Debug {
    /// Parse le contenu du message depuis le corps AFTN
    fn parse(body: &str) -> Result<Self, AftnError>
    where
        Self: Sized;
    
    /// Valide le message
    fn validate(&self) -> Result<(), AftnError>;
    
    /// Retourne la catégorie du message
    fn category(&self) -> MessageCategory;
}

/// Parse un sous-message selon sa catégorie
pub fn parse_submessage(category: &MessageCategory, body: &str) -> Result<Box<dyn SubMessage>, AftnError> {
    match category {
        MessageCategory::Notam => {
            let msg = NotamMessage::parse(body)?;
            Ok(Box::new(msg))
        }
        MessageCategory::Metar => {
            let msg = MetarMessage::parse(body)?;
            Ok(Box::new(msg))
        }
        MessageCategory::Taf => {
            let msg = TafMessage::parse(body)?;
            Ok(Box::new(msg))
        }
        MessageCategory::Sigmet => {
            let msg = SigmetMessage::parse(body)?;
            Ok(Box::new(msg))
        }
        MessageCategory::Airmet => {
            let msg = AirmetMessage::parse(body)?;
            Ok(Box::new(msg))
        }
        MessageCategory::Atis => {
            let msg = AtisMessage::parse(body)?;
            Ok(Box::new(msg))
        }
        MessageCategory::Volmet => {
            let msg = VolmetMessage::parse(body)?;
            Ok(Box::new(msg))
        }
        MessageCategory::FlightPlan => {
            let msg = FplMessage::parse(body)?;
            Ok(Box::new(msg))
        }
        MessageCategory::PositionReport => {
            let msg = PosMessage::parse(body)?;
            Ok(Box::new(msg))
        }
        MessageCategory::Operational(_) => {
            let msg = OperationalMessage::parse(body)?;
            Ok(Box::new(msg))
        }
        MessageCategory::Generic => {
            let msg = GenericMessage::parse(body)?;
            Ok(Box::new(msg))
        }
    }
}

