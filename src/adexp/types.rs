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
    
    /// Changed Departure (CHGDEP) - Reserved by France
    ChangedDeparture,
    
    /// Cancel Arrival (CNLARR) - Reserved by France
    CancelArrival,
    
    /// Cancel Departure (CNLDEP) - Reserved by France
    CancelDeparture,
    
    /// Configuration Operational (CONFIDM) - Reserved by France
    ConfigurationOperational,
    
    /// Departure (DEC) - Reserved by France
    DepartureDec,
    
    /// Estimated Actual Time Arrival (EATARR) - Reserved by France
    EstimatedActualTimeArrival,
    
    /// End Procedure (ENDPROC) - Reserved by France
    EndProcedure,
    
    /// ATFM Notification Message (ANM) - Reserved by NM
    AtfmNotificationMessage,
    
    /// Departure Clearance (CDAFTX) - Reserved by France (ARINC 620)
    DepartureClearance,
    
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
            // Reserved message titles
            "CHGDEP" => Ok(MessageType::ChangedDeparture),
            "CNLARR" => Ok(MessageType::CancelArrival),
            "CNLDEP" => Ok(MessageType::CancelDeparture),
            "CONFIDM" => Ok(MessageType::ConfigurationOperational),
            "DEC" => Ok(MessageType::DepartureDec),
            "EATARR" => Ok(MessageType::EstimatedActualTimeArrival),
            "ENDPROC" => Ok(MessageType::EndProcedure),
            "ANM" => Ok(MessageType::AtfmNotificationMessage),
            "CDAFTX" => Ok(MessageType::DepartureClearance),
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
            // Reserved message titles
            MessageType::ChangedDeparture => "CHGDEP",
            MessageType::CancelArrival => "CNLARR",
            MessageType::CancelDeparture => "CNLDEP",
            MessageType::ConfigurationOperational => "CONFIDM",
            MessageType::DepartureDec => "DEC",
            MessageType::EstimatedActualTimeArrival => "EATARR",
            MessageType::EndProcedure => "ENDPROC",
            MessageType::AtfmNotificationMessage => "ANM",
            MessageType::DepartureClearance => "CDAFTX",
            MessageType::Generic => "GEN",
        }
    }
}

