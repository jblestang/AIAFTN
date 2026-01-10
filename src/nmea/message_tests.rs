//! Tests unitaires pour les structures de messages NMEA

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nmea::message::NmeaMessage;
    use crate::nmea::types::NmeaMessageType;
    use crate::nmea::parser::NmeaParser;

    #[test]
    fn test_nmea_message_serialize() {
        let message = NmeaMessage::new(
            NmeaMessageType::GpGga,
            vec![
                "123519".to_string(),
                "4807.038".to_string(),
                "N".to_string(),
                "01131.000".to_string(),
                "E".to_string(),
                "1".to_string(),
                "08".to_string(),
                "0.9".to_string(),
                "545.4".to_string(),
                "M".to_string(),
                "46.9".to_string(),
                "M".to_string(),
                String::new(),
                String::new(),
            ],
            "47".to_string(),
            "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47".to_string(),
        );

        let serialized = message.serialize();
        assert!(serialized.starts_with('$'));
        assert!(serialized.contains("GPGGA"));
        assert!(serialized.contains('*'));
        assert!(serialized.len() > 20);
    }

    #[test]
    fn test_nmea_message_serialize_checksum_recalculation() {
        let original = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47";
        let message = NmeaParser::parse_message(original).unwrap();
        let serialized = message.serialize();

        // Le message sérialisé doit avoir un checksum valide
        let reparsed = NmeaParser::parse_message(&serialized);
        assert!(reparsed.is_ok(), "Le message sérialisé doit avoir un checksum valide");
    }

    #[test]
    fn test_nmea_message_serialize_round_trip() {
        let original = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47";
        let message = NmeaParser::parse_message(original).unwrap();
        let serialized = message.serialize();
        let reparsed = NmeaParser::parse_message(&serialized).unwrap();

        assert_eq!(message.message_type, reparsed.message_type);
        assert_eq!(message.fields.len(), reparsed.fields.len());
    }

    #[test]
    fn test_nmea_message_serialize_with_exclamation() {
        let message = NmeaMessage::new(
            NmeaMessageType::Aivdm,
            vec!["1".to_string(), "1".to_string(), String::new(), "A".to_string(), "13HOI:0P0000VOHLCnHQKwvL05Ip".to_string(), "0".to_string()],
            "XX".to_string(),
            "!AIVDM,1,1,,A,13HOI:0P0000VOHLCnHQKwvL05Ip,0*XX".to_string(),
        );

        let serialized = message.serialize();
        assert!(serialized.starts_with('!'));
    }
}

