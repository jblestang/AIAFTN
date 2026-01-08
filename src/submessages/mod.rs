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
pub mod abi;
pub mod chg;
pub mod cnl;
pub mod dla;
pub mod dep;
pub mod arr;
pub mod est;
pub mod spl;
pub mod req;
pub mod cof;
pub mod alr;

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
pub use abi::AbiMessage;
pub use chg::ChgMessage;
pub use cnl::CnlMessage;
pub use dla::DlaMessage;
pub use dep::DepMessage;
pub use arr::ArrMessage;
pub use est::EstMessage;
pub use spl::SplMessage;
pub use req::ReqMessage;
pub use cof::CofMessage;
pub use alr::AlrMessage;

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
        // Messages météorologiques
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
        
        // Messages de plan de vol
        MessageCategory::FlightPlan => {
            let msg = FplMessage::parse(body)?;
            Ok(Box::new(msg))
        }
        MessageCategory::Change => {
            let msg = ChgMessage::parse(body)?;
            Ok(Box::new(msg))
        }
        MessageCategory::Cancel => {
            let msg = CnlMessage::parse(body)?;
            Ok(Box::new(msg))
        }
        MessageCategory::Delay => {
            let msg = DlaMessage::parse(body)?;
            Ok(Box::new(msg))
        }
        MessageCategory::Departure => {
            let msg = DepMessage::parse(body)?;
            Ok(Box::new(msg))
        }
        MessageCategory::Arrival => {
            let msg = ArrMessage::parse(body)?;
            Ok(Box::new(msg))
        }
        MessageCategory::Estimate => {
            let msg = EstMessage::parse(body)?;
            Ok(Box::new(msg))
        }
        MessageCategory::AdvanceBoundaryInformation => {
            let msg = AbiMessage::parse(body)?;
            Ok(Box::new(msg))
        }
        MessageCategory::SupplementaryFlightPlan => {
            let msg = SplMessage::parse(body)?;
            Ok(Box::new(msg))
        }
        MessageCategory::CurrentFlightPlan
        | MessageCategory::UpdateFlightPlan => {
            // Utiliser le parser FPL pour les messages liés aux plans de vol
            // ou le parser opérationnel comme fallback
            match FplMessage::parse(body) {
                Ok(msg) => Ok(Box::new(msg)),
                Err(_) => {
                    let msg = OperationalMessage::parse(body)?;
                    Ok(Box::new(msg))
                }
            }
        }
        
        // Messages de coordination et autres
        MessageCategory::Coordination => {
            let msg = CofMessage::parse(body)?;
            Ok(Box::new(msg))
        }
        MessageCategory::Request => {
            let msg = ReqMessage::parse(body)?;
            Ok(Box::new(msg))
        }
        MessageCategory::Alerting => {
            let msg = AlrMessage::parse(body)?;
            Ok(Box::new(msg))
        }
        MessageCategory::RequestFlightPlan
        | MessageCategory::RequestFlightPlan
        | MessageCategory::RequestSupplementaryFlightPlan
        | MessageCategory::Denial
        | MessageCategory::Release
        | MessageCategory::Return
        | MessageCategory::AircraftPositionList
        | MessageCategory::Alerting
        | MessageCategory::Urgency
        | MessageCategory::RadioCommunicationFailure
        | MessageCategory::OceanicClearance
        | MessageCategory::Information
        | MessageCategory::MessageAcknowledgement
        | MessageCategory::Operational(_) => {
            let msg = OperationalMessage::parse(body)?;
            Ok(Box::new(msg))
        }
        
        MessageCategory::PositionReport => {
            let msg = PosMessage::parse(body)?;
            Ok(Box::new(msg))
        }
        
        MessageCategory::Generic => {
            let msg = GenericMessage::parse(body)?;
            Ok(Box::new(msg))
        }
    }
}

