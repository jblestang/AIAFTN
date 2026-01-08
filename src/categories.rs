use serde::{Deserialize, Serialize};
use crate::error::AftnError;

/// Catégories de messages AFTN selon la spécification 3.4
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MessageCategory {
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
    
    /// Flight Plan
    FlightPlan,
    
    /// Position Report
    PositionReport,
    
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
            "NOT" | "NOF" => Ok(MessageCategory::Notam),
            "MET" => Ok(MessageCategory::Metar),
            "TAF" => Ok(MessageCategory::Taf),
            "SIG" => Ok(MessageCategory::Sigmet),
            "AIR" => Ok(MessageCategory::Airmet),
            "ATI" => Ok(MessageCategory::Atis),
            "VOL" => Ok(MessageCategory::Volmet),
            "FPL" => Ok(MessageCategory::FlightPlan),
            "POS" => Ok(MessageCategory::PositionReport),
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
            MessageCategory::Notam => "NOT",
            MessageCategory::Metar => "MET",
            MessageCategory::Taf => "TAF",
            MessageCategory::Sigmet => "SIG",
            MessageCategory::Airmet => "AIR",
            MessageCategory::Atis => "ATI",
            MessageCategory::Volmet => "VOL",
            MessageCategory::FlightPlan => "FPL",
            MessageCategory::PositionReport => "POS",
            MessageCategory::Operational(s) => s,
            MessageCategory::Generic => "GEN",
        }
    }
}

