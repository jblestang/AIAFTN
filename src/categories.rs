use serde::{Deserialize, Serialize};
use crate::error::AftnError;

/// Catégories de messages AFTN selon la spécification 3.4
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MessageCategory {
    // Messages météorologiques
    /// NOTAM (Notice to Airmen)
    Notam,
    
    /// METAR (Meteorological Aerodrome Report)
    Metar,
    
    /// TAF (Terminal Aerodrome Forecast)
    Taf,
    
    /// SIGMET (Significant Meteorological Information)
    Sigmet,
    
    /// AIRMET (Airmen's Meteorological Information)
    Airmet,
    
    /// ATIS (Automatic Terminal Information Service)
    Atis,
    
    /// VOLMET (Meteorological Information for Aircraft in Flight)
    Volmet,
    
    // Messages de plan de vol
    /// Flight Plan
    FlightPlan,
    
    /// Change (modification de plan de vol)
    Change,
    
    /// Cancel (annulation de plan de vol)
    Cancel,
    
    /// Delay (retard)
    Delay,
    
    /// Departure (départ)
    Departure,
    
    /// Arrival (arrivée)
    Arrival,
    
    /// Estimate (estimation)
    Estimate,
    
    /// Supplementary Flight Plan
    SupplementaryFlightPlan,
    
    /// Current Flight Plan
    CurrentFlightPlan,
    
    /// Update Flight Plan
    UpdateFlightPlan,
    
    // Messages de coordination
    /// Coordination
    Coordination,
    
    /// Advance Boundary Information
    AdvanceBoundaryInformation,
    
    /// Request (demande)
    Request,
    
    /// Request Flight Plan
    RequestFlightPlan,
    
    /// Request Supplementary Flight Plan
    RequestSupplementaryFlightPlan,
    
    /// Denial (refus)
    Denial,
    
    /// Release (libération)
    Release,
    
    /// Return (retour)
    Return,
    
    // Messages de position et rapports
    /// Position Report
    PositionReport,
    
    /// Aircraft Position List
    AircraftPositionList,
    
    // Messages d'alerte et d'urgence
    /// Alerting
    Alerting,
    
    /// Urgency
    Urgency,
    
    /// Radio Communication Failure
    RadioCommunicationFailure,
    
    // Messages spéciaux
    /// Oceanic Clearance
    OceanicClearance,
    
    /// Information
    Information,
    
    /// Message Acknowledgement
    MessageAcknowledgement,
    
    // Messages additionnels selon spécification AFTN
    /// Acceptance (acceptation de plan de vol)
    Acceptance,
    
    /// Transfer of Control
    TransferOfControl,
    
    /// Air Report (rapport aérien)
    AirReport,
    
    /// Divers messages opérationnels
    Operational(String),
    
    /// Message générique non catégorisé
    Generic,
}

impl MessageCategory {
    /// Parse une catégorie depuis un identifiant de message
    pub fn from_message_id(id: &str) -> Result<Self, AftnError> {
        if id.len() < 3 {
            return Ok(MessageCategory::Generic);
        }
        
        let prefix = &id[..3].to_uppercase();
        
        match prefix.as_str() {
            // Messages météorologiques
            "NOT" | "NOF" => Ok(MessageCategory::Notam),
            "MET" => Ok(MessageCategory::Metar),
            "TAF" => Ok(MessageCategory::Taf),
            "SIG" => Ok(MessageCategory::Sigmet),
            "AIR" => Ok(MessageCategory::Airmet),
            "ATI" => Ok(MessageCategory::Atis),
            "VOL" => Ok(MessageCategory::Volmet),
            
            // Messages de plan de vol
            "FPL" => Ok(MessageCategory::FlightPlan),
            "CHG" => Ok(MessageCategory::Change),
            "CNL" => Ok(MessageCategory::Cancel),
            "DLA" => Ok(MessageCategory::Delay),
            "DEP" => Ok(MessageCategory::Departure),
            "ARR" => Ok(MessageCategory::Arrival),
            "EST" => Ok(MessageCategory::Estimate),
            "SPL" => Ok(MessageCategory::SupplementaryFlightPlan),
            "CPL" => Ok(MessageCategory::CurrentFlightPlan),
            "UPL" => Ok(MessageCategory::UpdateFlightPlan),
            
            // Messages de coordination
            "COF" | "CDN" => Ok(MessageCategory::Coordination),
            "ABI" => Ok(MessageCategory::AdvanceBoundaryInformation),
            "REQ" => Ok(MessageCategory::Request),
            "RQP" => Ok(MessageCategory::RequestFlightPlan),
            "RQS" => Ok(MessageCategory::RequestSupplementaryFlightPlan),
            "DEN" => Ok(MessageCategory::Denial),
            "RLS" => Ok(MessageCategory::Release),
            "RTN" => Ok(MessageCategory::Return),
            
            // Messages de position et rapports
            "POS" => Ok(MessageCategory::PositionReport),
            "APL" => Ok(MessageCategory::AircraftPositionList),
            
            // Messages d'alerte et d'urgence
            "ALR" => Ok(MessageCategory::Alerting),
            "URG" => Ok(MessageCategory::Urgency),
            "RCF" => Ok(MessageCategory::RadioCommunicationFailure),
            
            // Messages spéciaux
            "OCL" => Ok(MessageCategory::OceanicClearance),
            "INF" => Ok(MessageCategory::Information),
            "MAC" => Ok(MessageCategory::MessageAcknowledgement),
            
            // Messages additionnels
            "ACP" => Ok(MessageCategory::Acceptance),
            "TCX" => Ok(MessageCategory::TransferOfControl),
            // AIREP est détecté comme "AIR" + "EP", mais on vérifie d'abord AIRMET
            // Si le message commence par "AIREP", c'est un Air Report
            _ if id.len() >= 5 && &id[..5].to_uppercase() == "AIREP" => Ok(MessageCategory::AirReport),
            
            _ => {
                // Si le préfixe est "GEN", c'est un message générique
                if prefix == "GEN" {
                    Ok(MessageCategory::Generic)
                } else if id.len() >= 3 {
                    // Sinon, c'est un message opérationnel
                    Ok(MessageCategory::Operational(prefix.to_string()))
                } else {
                    Ok(MessageCategory::Generic)
                }
            }
        }
    }
    
    /// Retourne le préfixe de catégorie
    pub fn prefix(&self) -> &str {
        match self {
            // Messages météorologiques
            MessageCategory::Notam => "NOT",
            MessageCategory::Metar => "MET",
            MessageCategory::Taf => "TAF",
            MessageCategory::Sigmet => "SIG",
            MessageCategory::Airmet => "AIR",
            MessageCategory::Atis => "ATI",
            MessageCategory::Volmet => "VOL",
            
            // Messages de plan de vol
            MessageCategory::FlightPlan => "FPL",
            MessageCategory::Change => "CHG",
            MessageCategory::Cancel => "CNL",
            MessageCategory::Delay => "DLA",
            MessageCategory::Departure => "DEP",
            MessageCategory::Arrival => "ARR",
            MessageCategory::Estimate => "EST",
            MessageCategory::SupplementaryFlightPlan => "SPL",
            MessageCategory::CurrentFlightPlan => "CPL",
            MessageCategory::UpdateFlightPlan => "UPL",
            
            // Messages de coordination
            MessageCategory::Coordination => "COF",
            MessageCategory::AdvanceBoundaryInformation => "ABI",
            MessageCategory::Request => "REQ",
            MessageCategory::RequestFlightPlan => "RQP",
            MessageCategory::RequestSupplementaryFlightPlan => "RQS",
            MessageCategory::Denial => "DEN",
            MessageCategory::Release => "RLS",
            MessageCategory::Return => "RTN",
            
            // Messages de position et rapports
            MessageCategory::PositionReport => "POS",
            MessageCategory::AircraftPositionList => "APL",
            
            // Messages d'alerte et d'urgence
            MessageCategory::Alerting => "ALR",
            MessageCategory::Urgency => "URG",
            MessageCategory::RadioCommunicationFailure => "RCF",
            
            // Messages spéciaux
            MessageCategory::OceanicClearance => "OCL",
            MessageCategory::Information => "INF",
            MessageCategory::MessageAcknowledgement => "MAC",
            
            // Messages additionnels
            MessageCategory::Acceptance => "ACP",
            MessageCategory::TransferOfControl => "TCX",
            MessageCategory::AirReport => "AIREP",
            
            MessageCategory::Operational(s) => s,
            MessageCategory::Generic => "GEN",
        }
    }
}

