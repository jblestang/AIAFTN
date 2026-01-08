//! Parser pour les messages ABI (Advance Boundary Information)

use serde::{Deserialize, Serialize};
use pest_derive::Parser;
use crate::aftn::categories::MessageCategory;
use crate::aftn::error::AftnError;
use crate::aftn::submessages::SubMessage;
use crate::aftn::validation;

#[derive(Parser)]
#[grammar = "aftn/categories/coordination/coordination.pest"]
struct AbiParser;

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
        use pest::Parser;
        
        let result = AbiParser::parse(Rule::abi, body);
        
        if let Ok(mut pairs) = result {
            let abi_pair = pairs.next().ok_or_else(|| {
                AftnError::ParseError("Empty ABI parse result".to_string())
            })?;
            return Self::parse_abi_pair(abi_pair, body);
        }
        
        // Fallback: parsing manuel simple
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
        
        // Valider l'identifiant d'aéronef s'il est présent (peut être un callsign)
        if let Some(ref aircraft_id) = self.aircraft_id {
            // L'identifiant d'aéronef peut être un callsign ou un autre format
            if aircraft_id.len() <= 7 && aircraft_id.chars().next().map(|c| c.is_ascii_alphabetic()).unwrap_or(false) {
                validation::validate_callsign(aircraft_id)?;
            }
        }
        
        // Valider l'aérodrome de départ s'il est présent
        if let Some(ref departure) = self.departure {
            validation::validate_aerodrome_code(departure)?;
        }
        
        // Valider l'aérodrome de destination s'il est présent
        if let Some(ref destination) = self.destination {
            validation::validate_aerodrome_code(destination)?;
        }
        
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::AdvanceBoundaryInformation
    }
}

impl AbiMessage {
    fn parse_abi_pair(pair: pest::iterators::Pair<Rule>, raw: &str) -> Result<Self, AftnError> {
        let mut aircraft_id = None;
        let mut ssr_code = None;
        let mut departure = None;
        let mut destination = None;
        let mut estimated_data = None;
        let mut aircraft_type = None;
        
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::aircraft_id => {
                    aircraft_id = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::ssr_code => {
                    ssr_code = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::departure => {
                    departure = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::destination => {
                    destination = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::estimated_data => {
                    estimated_data = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::aircraft_type => {
                    aircraft_type = Some(inner_pair.as_str().trim().to_string());
                }
                _ => {}
            }
        }
        
        Ok(AbiMessage {
            aircraft_id,
            ssr_code,
            departure,
            destination,
            estimated_data,
            aircraft_type,
            raw: raw.to_string(),
        })
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

