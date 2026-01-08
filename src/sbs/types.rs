//! Types de messages SBS (Mode-S/ADS-B)

use serde::{Deserialize, Serialize};
use crate::sbs::error::SbsError;

/// Types de messages SBS supportés
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SbsMessageType {
    /// MSG,1: Transmission (ES Identification and Category)
    Identification,
    /// MSG,2: Transmission (ES Surface Position)
    SurfacePosition,
    /// MSG,3: Transmission (ES Airborne Position)
    AirbornePosition,
    /// MSG,4: Transmission (ES Airborne Velocity)
    AirborneVelocity,
    /// MSG,5: Transmission (Surveillance Altitude)
    SurveillanceAltitude,
    /// MSG,6: Transmission (Surveillance ID)
    SurveillanceId,
    /// MSG,7: Transmission (Air to Air)
    AirToAir,
    /// MSG,8: Transmission (All Call Reply)
    AllCallReply,
    /// Type générique (pour types non encore implémentés)
    Generic(u8),
}

impl SbsMessageType {
    /// Détermine le type de message depuis le numéro MSG
    pub fn from_msg_number(msg_number: u8) -> Result<Self, SbsError> {
        match msg_number {
            1 => Ok(SbsMessageType::Identification),
            2 => Ok(SbsMessageType::SurfacePosition),
            3 => Ok(SbsMessageType::AirbornePosition),
            4 => Ok(SbsMessageType::AirborneVelocity),
            5 => Ok(SbsMessageType::SurveillanceAltitude),
            6 => Ok(SbsMessageType::SurveillanceId),
            7 => Ok(SbsMessageType::AirToAir),
            8 => Ok(SbsMessageType::AllCallReply),
            n => Ok(SbsMessageType::Generic(n)),
        }
    }
    
    /// Retourne le numéro MSG du type de message
    pub fn msg_number(&self) -> u8 {
        match self {
            SbsMessageType::Identification => 1,
            SbsMessageType::SurfacePosition => 2,
            SbsMessageType::AirbornePosition => 3,
            SbsMessageType::AirborneVelocity => 4,
            SbsMessageType::SurveillanceAltitude => 5,
            SbsMessageType::SurveillanceId => 6,
            SbsMessageType::AirToAir => 7,
            SbsMessageType::AllCallReply => 8,
            SbsMessageType::Generic(n) => *n,
        }
    }
}

