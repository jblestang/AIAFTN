use serde::{Deserialize, Serialize};
use pest::Parser;
use pest_derive::Parser;
use crate::aftn::categories::MessageCategory;
use crate::aftn::error::AftnError;
use crate::aftn::submessages::SubMessage;

#[derive(Parser)]
#[grammar = "aftn/categories/meteorological/meteorological.pest"]
struct MetarParser;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetarMessage {
    /// Identifiant de l'aérodrome (ex: LFPG)
    pub station: Option<String>,
    
    /// Date et heure d'observation (format: DDHHMMZ)
    pub observation_time: Option<String>,
    
    /// Vent (format: DDDSSKT)
    pub wind: Option<String>,
    
    /// Visibilité
    pub visibility: Option<String>,
    
    /// Conditions météorologiques
    pub weather: Option<String>,
    
    /// Nuages
    pub clouds: Option<String>,
    
    /// Température / Point de rosée
    pub temperature: Option<String>,
    
    /// Pression QNH
    pub qnh: Option<String>,
    
    /// Indicateurs spéciaux (NOSIG, etc.)
    pub remarks: Option<String>,
    
    /// Corps brut du message
    pub raw: String,
}

impl SubMessage for MetarMessage {
    fn parse(body: &str) -> Result<Self, AftnError> {
        let mut pairs = MetarParser::parse(Rule::metar, body)
            .map_err(|e| AftnError::ParseError(format!("METAR parse error: {}", e)))?;
        
        let metar_pair = pairs.next().ok_or_else(|| {
            AftnError::ParseError("Empty METAR parse result".to_string())
        })?;
        
        Self::parse_metar_pair(metar_pair, body)
    }
    
    fn validate(&self) -> Result<(), AftnError> {
        // Validation basique : le message ne doit pas être vide
        if self.raw.trim().is_empty() {
            return Err(AftnError::InvalidFormat("METAR cannot be empty".to_string()));
        }
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        MessageCategory::Metar
    }
}

impl MetarMessage {
    fn parse_metar_pair(pair: pest::iterators::Pair<Rule>, raw: &str) -> Result<Self, AftnError> {
        let mut station = None;
        let mut observation_time = None;
        let mut wind = None;
        let mut visibility = None;
        let mut weather = None;
        let mut clouds = None;
        let mut temperature = None;
        let mut qnh = None;
        let mut remarks = None;
        
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::station => {
                    station = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::observation_time => {
                    observation_time = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::wind => {
                    wind = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::visibility => {
                    visibility = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::weather => {
                    weather = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::clouds => {
                    clouds = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::temperature => {
                    temperature = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::qnh => {
                    qnh = Some(inner_pair.as_str().trim().to_string());
                }
                Rule::remarks => {
                    remarks = Some(inner_pair.as_str().trim().to_string());
                }
                _ => {}
            }
        }
        
        Ok(MetarMessage {
            station,
            observation_time,
            wind,
            visibility,
            weather,
            clouds,
            temperature,
            qnh,
            remarks,
            raw: raw.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_metar() {
        let input = "METAR LFPG 151230Z 28015KT 9999 FEW030 12/08 Q1013 NOSIG";
        let result = MetarMessage::parse(input);
        assert!(result.is_ok());
        
        let metar = result.unwrap();
        assert_eq!(metar.station, Some("LFPG".to_string()));
    }
}

