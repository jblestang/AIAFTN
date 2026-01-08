//! Parser pour les messages ABI (Advance Boundary Information)

use serde::{Deserialize, Serialize};
use crate::categories::MessageCategory;
use crate::error::AftnError;
use crate::submessages::SubMessage;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbiMessage {
    /// Identifiant de l'aéronef
    pub aircraft_id: Option<String>,
    
    /// Code SSR (Mode S)
    pub ssr_code: Option<String>,
    
    /// Aérodrome de départ
    pub departure: Option<String>,
    
    /// Aérodrome de destination
    pub destination: Option<String>,
    
    /// Données estimées (position, temps, niveau de vol)
    pub estimated_data: Option<String>,
    
    /// Type d'aéronef
    pub aircraft_type: Option<String>,
    
    /// Corps brut du message
    pub raw: String,
}

impl SubMessage for AbiMessage {
    fn parse(body: &str) -> Result<Self, AftnError> {
        // Parsing simple pour ABI - peut être amélioré avec une grammaire PEST
        let parts: Vec<&str> = body.split_whitespace().collect();
        
        let mut aircraft_id = None;
        let mut ssr_code = None;
        let mut departure = None;
        let mut destination = None;
        let mut estimated_data = None;
        let mut aircraft_type = None;
        
        // Format typique: ABI AIRCRAFT_ID [SSR_CODE] DEP DEST [EST_DATA] [TYPE]
        let mut i = 0;
        while i < parts.len() {
            let part = parts[i];
            match part {
                p if p.len() == 4 && p.chars().all(|c| c.is_alphabetic()) => {
                    // Probablement un code d'aéroport
                    if departure.is_none() {
                        departure = Some(p.to_string());
                    } else if destination.is_none() {
                        destination = Some(p.to_string());
                    }
                }
                p if p.len() == 4 && p.chars().all(|c| c.is_alphanumeric()) => {
                    // Probablement un code SSR
                    if ssr_code.is_none() {
                        ssr_code = Some(p.to_string());
                    }
                }
                p if p.len() >= 3 && p.len() <= 7 && p.chars().all(|c| c.is_alphanumeric()) => {
                    // Probablement un identifiant d'aéronef
                    if aircraft_id.is_none() {
                        aircraft_id = Some(p.to_string());
                    }
                }
                _ => {}
            }
            i += 1;
        }
        
        Ok(AbiMessage {
            aircraft_id,
            ssr_code,
            departure,
            destination,
            estimated_data,
            aircraft_type,
            raw: body.to_string(),
        })
    }
    
    fn validate(&self) -> Result<(), AftnError> {
        if self.raw.trim().is_empty() {
            return Err(AftnError::InvalidFormat("ABI message cannot be empty".to_string()));
        }
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::AdvanceBoundaryInformation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_abi() {
        let input = "ABI ABC123 1234 LFPG KJFK 1200 F350 A320";
        let result = AbiMessage::parse(input);
        assert!(result.is_ok());
    }
}

