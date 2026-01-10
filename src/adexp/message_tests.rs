//! Tests unitaires pour les structures de messages ADEXP

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adexp::message::{AdexpMessage, Section};
    use crate::adexp::types::MessageType;
    use crate::adexp::parser::AdexpParser;

    #[test]
    fn test_adexp_message_serialize() {
        let mut message = AdexpMessage::new(String::new());
        message.message_type = MessageType::FlightPlan;
        
        let mut main_section = Section::new(String::new());
        main_section.add_field("TITLE".to_string(), "FPL".to_string());
        main_section.add_field("ARCID".to_string(), "ABC123".to_string());
        main_section.add_field("ADEP".to_string(), "LFPG".to_string());
        main_section.add_field("ADES".to_string(), "LFPB".to_string());
        message.sections.insert(String::new(), main_section);

        let serialized = message.serialize();
        assert!(serialized.contains("-ADEXP"));
        assert!(serialized.contains("-TITLE"));
        assert!(serialized.contains("FPL"));
        assert!(serialized.contains("-ARCID"));
        assert!(serialized.contains("ABC123"));
    }

    #[test]
    fn test_adexp_message_serialize_with_sections() {
        let mut message = AdexpMessage::new(String::new());
        message.message_type = MessageType::FlightPlan;
        
        let mut main_section = Section::new(String::new());
        main_section.add_field("TITLE".to_string(), "FPL".to_string());
        main_section.add_field("ARCID".to_string(), "ABC123".to_string());
        message.sections.insert(String::new(), main_section);

        let mut rtepts_section = Section::new("RTEPTS".to_string());
        rtepts_section.add_field("PTID".to_string(), "WPT1".to_string());
        rtepts_section.add_field("PTID".to_string(), "WPT2".to_string());
        message.sections.insert("RTEPTS".to_string(), rtepts_section);

        let serialized = message.serialize();
        assert!(serialized.contains("-BEGIN"));
        assert!(serialized.contains("RTEPTS"));
        assert!(serialized.contains("-END"));
        assert!(serialized.contains("RTEPTS"));
    }

    #[test]
    fn test_adexp_message_serialize_round_trip() {
        let original = "-ADEXP\n-TITLE FPL\n-ARCID ABC123\n-ADEP LFPG\n-ADES LFPB";
        let message = AdexpParser::parse_message(original).unwrap();
        let serialized = message.serialize();
        let reparsed = AdexpParser::parse_message(&serialized).unwrap();

        assert_eq!(message.message_type, reparsed.message_type);
        assert!(reparsed.sections.contains_key(""));
    }
}

