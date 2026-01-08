use serde::{Deserialize, Serialize};
use pest::Parser;
use pest_derive::Parser;
use crate::aftn::categories::MessageCategory;
use crate::aftn::error::AftnError;
use crate::aftn::submessages::SubMessage;

#[derive(Parser)]
#[grammar = "aftn/categories/operational/operational.pest"]
struct OperationalParser;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OperationalMessage {
    /// Type opérationnel
    pub op_type: String,
    
    /// Contenu du message
    pub content: String,
    
    /// Corps brut du message
    pub raw: String,
}

impl SubMessage for OperationalMessage {
    fn parse(body: &str) -> Result<Self, AftnError> {
        let mut pairs = OperationalParser::parse(Rule::operational, body)
            .map_err(|e| AftnError::ParseError(format!("Operational parse error: {}", e)))?;
        
        let op_pair = pairs.next().ok_or_else(|| {
            AftnError::ParseError("Empty operational parse result".to_string())
        })?;
        
        Self::parse_operational_pair(op_pair, body)
    }
    
    fn validate(&self) -> Result<(), AftnError> {
        if self.raw.trim().is_empty() {
            return Err(AftnError::InvalidFormat("Operational message cannot be empty".to_string()));
        }
        Ok(())
    }
    
    fn category(&self) -> MessageCategory {
        // Essayer de déterminer la catégorie depuis le type opérationnel
        let op_upper = self.op_type.to_uppercase();
        match op_upper.as_str() {
            "CHG" => MessageCategory::Change,
            "CNL" => MessageCategory::Cancel,
            "DLA" => MessageCategory::Delay,
            "DEP" => MessageCategory::Departure,
            "ARR" => MessageCategory::Arrival,
            "EST" => MessageCategory::Estimate,
            "SPL" => MessageCategory::SupplementaryFlightPlan,
            "CPL" => MessageCategory::CurrentFlightPlan,
            "UPL" => MessageCategory::UpdateFlightPlan,
            "COF" | "CDN" => MessageCategory::Coordination,
            "ABI" => MessageCategory::AdvanceBoundaryInformation,
            "REQ" => MessageCategory::Request,
            "RQP" => MessageCategory::RequestFlightPlan,
            "RQS" => MessageCategory::RequestSupplementaryFlightPlan,
            "DEN" => MessageCategory::Denial,
            "RLS" => MessageCategory::Release,
            "RTN" => MessageCategory::Return,
            "APL" => MessageCategory::AircraftPositionList,
            "ALR" => MessageCategory::Alerting,
            "URG" => MessageCategory::Urgency,
            "RCF" => MessageCategory::RadioCommunicationFailure,
            "OCL" => MessageCategory::OceanicClearance,
            "INF" => MessageCategory::Information,
            "MAC" => MessageCategory::MessageAcknowledgement,
            _ => MessageCategory::Operational(self.op_type.clone()),
        }
    }
}

impl OperationalMessage {
    fn parse_operational_pair(pair: pest::iterators::Pair<Rule>, raw: &str) -> Result<Self, AftnError> {
        let mut op_type = String::new();
        let mut content = String::new();
        
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::operational_prefix => {
                    op_type = inner_pair.as_str().trim().to_uppercase();
                }
                Rule::operational_content => {
                    content = inner_pair.as_str().trim().to_string();
                }
                _ => {}
            }
        }
        
        if op_type.is_empty() && raw.len() >= 3 {
            op_type = raw[..3].to_uppercase();
        }
        
        if content.is_empty() {
            content = raw.to_string();
        }
        
        Ok(OperationalMessage {
            op_type,
            content,
            raw: raw.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_operational() {
        let input = "DEP ABC123 LFPG 151230";
        let result = OperationalMessage::parse(input);
        assert!(result.is_ok());
    }
}

