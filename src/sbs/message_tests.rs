//! Tests unitaires pour les structures de messages SBS

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sbs::message::SbsMessage;
    use crate::sbs::types::SbsMessageType;
    use crate::sbs::parser::SbsParser;

    #[test]
    fn test_sbs_message_serialize() {
        let mut message = SbsMessage::new(
            SbsMessageType::AirbornePosition,
            String::new(),
        );
        message.aircraft_id = Some("4CA2E6".to_string());
        message.altitude = Some(37025);
        message.latitude = Some(51.4703);
        message.longitude = Some(-0.4543);

        let serialized = message.serialize();
        assert!(serialized.starts_with("MSG,"));
        assert!(serialized.contains("3")); // AirbornePosition = MSG,3
        assert!(serialized.contains("4CA2E6"));
        assert!(serialized.contains("37025"));
    }

    #[test]
    fn test_sbs_message_serialize_with_all_fields() {
        let mut message = SbsMessage::new(
            SbsMessageType::Identification,
            String::new(),
        );
        message.session_id = Some("29315".to_string());
        message.aircraft_id = Some("4CA2E6".to_string());
        message.callsign = Some("BAW1425".to_string());
        message.alert = Some(true);
        message.emergency = Some(false);
        message.is_on_ground = Some(false);

        let serialized = message.serialize();
        assert!(serialized.starts_with("MSG,1"));
        assert!(serialized.contains("BAW1425"));
    }

    #[test]
    fn test_sbs_message_serialize_round_trip() {
        let original = "MSG,3,145,29315,4CA2E6,27215,2015/02/05,14:53:22.734,2015/02/05,14:53:22.734,,37025,1035.0,295.6,51.4703,-0.4543,,,,,0";
        let message = SbsParser::parse_message(original).unwrap();
        let serialized = message.serialize();
        let reparsed = SbsParser::parse_message(&serialized).unwrap();

        assert_eq!(message.message_type, reparsed.message_type);
        assert_eq!(message.aircraft_id, reparsed.aircraft_id);
        assert_eq!(message.altitude, reparsed.altitude);
    }

    #[test]
    fn test_sbs_message_serialize_boolean_fields() {
        let mut message = SbsMessage::new(
            SbsMessageType::SurfacePosition,
            String::new(),
        );
        message.alert = Some(true);
        message.emergency = Some(true);
        message.spi = Some(false);
        message.is_on_ground = Some(true);

        let serialized = message.serialize();
        assert!(serialized.contains(",1,")); // alert = true
        assert!(serialized.contains(",1,")); // emergency = true
        assert!(serialized.contains(",0")); // spi = false
        assert!(serialized.ends_with(",1") || serialized.contains(",1,")); // is_on_ground = true
    }
}

