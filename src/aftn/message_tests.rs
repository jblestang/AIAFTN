//! Tests unitaires pour les structures de messages AFTN

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aftn::message::{AftnMessage, Addresses, TransmissionTime};
    use crate::aftn::categories::MessageCategory;
    use crate::aftn::parser::AftnParser;

    #[test]
    fn test_aftn_message_serialize() {
        let message = AftnMessage {
            priority: "GG".to_string(),
            addresses: Addresses {
                origin: "LFPGYYYX".to_string(),
                destinations: vec!["LFPOYYYX".to_string()],
            },
            category: MessageCategory::Notam,
            transmission_time: TransmissionTime {
                day: 15,
                hour: 12,
                minute: 30,
            },
            body: "NOTAM A1234/24 LFPG RWY 09/27 CLOSED".to_string(),
            sequence_number: None,
        };

        let serialized = message.serialize();
        assert!(serialized.contains("GG"));
        assert!(serialized.contains("LFPGYYYX"));
        assert!(serialized.contains("LFPOYYYX"));
        assert!(serialized.contains("151230"));
        assert!(serialized.contains("NOTAM"));
    }

    #[test]
    fn test_aftn_message_serialize_with_sequence() {
        let message = AftnMessage {
            priority: "FF".to_string(),
            addresses: Addresses {
                origin: "LFPGYYYX".to_string(),
                destinations: vec!["LFPOYYYX".to_string()],
            },
            category: MessageCategory::Notam,
            transmission_time: TransmissionTime {
                day: 15,
                hour: 12,
                minute: 30,
            },
            body: "NOTAM A1234/24".to_string(),
            sequence_number: Some("001".to_string()),
        };

        let serialized = message.serialize();
        assert!(serialized.contains("/SEQ"));
        assert!(serialized.contains("001"));
    }

    #[test]
    fn test_aftn_message_serialize_multiple_destinations() {
        let message = AftnMessage {
            priority: "DD".to_string(),
            addresses: Addresses {
                origin: "LFPGYYYX".to_string(),
                destinations: vec!["LFPOYYYX".to_string(), "LFPBYYYX".to_string()],
            },
            category: MessageCategory::Metar,
            transmission_time: TransmissionTime {
                day: 20,
                hour: 15,
                minute: 30,
            },
            body: "METAR LFPG 201530Z".to_string(),
            sequence_number: None,
        };

        let serialized = message.serialize();
        assert!(serialized.contains("LFPOYYYX"));
        assert!(serialized.contains("LFPBYYYX"));
    }

    #[test]
    fn test_aftn_message_serialize_round_trip() {
        let original = "GG LFPGYYYX LFPOYYYX 151230 NOTAM A1234/24";
        let message = AftnParser::parse_message(original).unwrap();
        let serialized = message.serialize();
        let reparsed = AftnParser::parse_message(&serialized).unwrap();

        assert_eq!(message.priority, reparsed.priority);
        assert_eq!(message.addresses.origin, reparsed.addresses.origin);
        assert_eq!(message.addresses.destinations, reparsed.addresses.destinations);
        assert_eq!(message.transmission_time.day, reparsed.transmission_time.day);
        assert_eq!(message.transmission_time.hour, reparsed.transmission_time.hour);
        assert_eq!(message.transmission_time.minute, reparsed.transmission_time.minute);
    }
}

