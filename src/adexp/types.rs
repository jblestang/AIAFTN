use serde::{Deserialize, Serialize};
use crate::adexp::error::AdexpError;

/// Types de messages ADEXP selon la spécification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MessageType {
    /// Flight Plan (FPL)
    FlightPlan,
    
    /// Change (CHG)
    Change,
    
    /// Delay (DLA)
    Delay,
    
    /// Cancel (CNL)
    Cancel,
    
    /// Departure (DEP)
    Departure,
    
    /// Arrival (ARR)
    Arrival,
    
    /// Coordination (COF)
    Coordination,
    
    /// Request (REQ)
    Request,
    
    /// Estimate (EST)
    Estimate,
    
    /// Position (POS)
    Position,
    
    /// Logon (LOG)
    Logon,
    
    /// Logoff (LOF)
    Logoff,
    
    /// Message générique non catégorisé
    Generic,
}

impl MessageType {
    /// Parse un type depuis le champ TITLE
    pub fn from_title(title: &str) -> Result<Self, AdexpError> {
        let title_upper = title.to_uppercase();
        match title_upper.as_str() {
            "FPL" => Ok(MessageType::FlightPlan),
            "CHG" => Ok(MessageType::Change),
            "DLA" => Ok(MessageType::Delay),
            "CNL" => Ok(MessageType::Cancel),
            "DEP" => Ok(MessageType::Departure),
            "ARR" => Ok(MessageType::Arrival),
            "COF" => Ok(MessageType::Coordination),
            "REQ" => Ok(MessageType::Request),
            "EST" => Ok(MessageType::Estimate),
            "POS" => Ok(MessageType::Position),
            "LOG" => Ok(MessageType::Logon),
            "LOF" => Ok(MessageType::Logoff),
            _ => Ok(MessageType::Generic),
        }
    }
    
    /// Retourne le préfixe du type
    pub fn prefix(&self) -> &str {
        match self {
            MessageType::FlightPlan => "FPL",
            MessageType::Change => "CHG",
            MessageType::Delay => "DLA",
            MessageType::Cancel => "CNL",
            MessageType::Departure => "DEP",
            MessageType::Arrival => "ARR",
            MessageType::Coordination => "COF",
            MessageType::Request => "REQ",
            MessageType::Estimate => "EST",
            MessageType::Position => "POS",
            MessageType::Logon => "LOG",
            MessageType::Logoff => "LOF",
            MessageType::Generic => "GEN",
        }
    }
}

