//! Tests pour les structures de messages SBS

use aftn::{SbsParser, SbsMessageType};

#[test]
fn test_message_type_from_number() {
    assert_eq!(SbsMessageType::from_msg_number(1).unwrap(), SbsMessageType::Identification);
    assert_eq!(SbsMessageType::from_msg_number(2).unwrap(), SbsMessageType::SurfacePosition);
    assert_eq!(SbsMessageType::from_msg_number(3).unwrap(), SbsMessageType::AirbornePosition);
    assert_eq!(SbsMessageType::from_msg_number(4).unwrap(), SbsMessageType::AirborneVelocity);
    assert_eq!(SbsMessageType::from_msg_number(5).unwrap(), SbsMessageType::SurveillanceAltitude);
    assert_eq!(SbsMessageType::from_msg_number(6).unwrap(), SbsMessageType::SurveillanceId);
    assert_eq!(SbsMessageType::from_msg_number(7).unwrap(), SbsMessageType::AirToAir);
    assert_eq!(SbsMessageType::from_msg_number(8).unwrap(), SbsMessageType::AllCallReply);
}

#[test]
fn test_message_type_msg_number() {
    assert_eq!(SbsMessageType::Identification.msg_number(), 1);
    assert_eq!(SbsMessageType::SurfacePosition.msg_number(), 2);
    assert_eq!(SbsMessageType::AirbornePosition.msg_number(), 3);
    assert_eq!(SbsMessageType::AirborneVelocity.msg_number(), 4);
    assert_eq!(SbsMessageType::SurveillanceAltitude.msg_number(), 5);
    assert_eq!(SbsMessageType::SurveillanceId.msg_number(), 6);
    assert_eq!(SbsMessageType::AirToAir.msg_number(), 7);
    assert_eq!(SbsMessageType::AllCallReply.msg_number(), 8);
}

#[test]
fn test_parse_with_all_fields() {
    let input = "MSG,3,145,29315,4CA2E6,27215,2015/02/05,14:53:22.734,2015/02/05,14:53:22.734,BAW1425,37025,1035.0,295.6,51.4703,-0.4543,-3200,7500,1,0,1,0";
    let message = SbsParser::parse_message(input).expect("Should parse");
    
    assert_eq!(message.callsign, Some("BAW1425".to_string()));
    assert_eq!(message.altitude, Some(37025));
    assert_eq!(message.ground_speed, Some(1035.0));
    assert_eq!(message.track, Some(295.6));
    assert_eq!(message.latitude, Some(51.4703));
    assert_eq!(message.longitude, Some(-0.4543));
    assert_eq!(message.vertical_rate, Some(-3200));
    assert_eq!(message.squawk, Some("7500".to_string()));
    assert_eq!(message.alert, Some(true));
    assert_eq!(message.emergency, Some(false));
    assert_eq!(message.spi, Some(true));
    assert_eq!(message.is_on_ground, Some(false));
}

