//! Tests pour les messages GPS NMEA

use aftn::{NmeaParser, GgaMessage, RmcMessage, GsaMessage, VtgMessage};

#[test]
fn test_parse_gga_message() {
    let input = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47";
    let message = NmeaParser::parse_message(input).expect("Should parse");
    
    assert_eq!(message.message_type.identifier(), "GPGGA");
    assert_eq!(message.fields.len(), 14);
    
    let gga = GgaMessage::from_nmea(&message).expect("Should parse GGA");
    assert_eq!(gga.time, Some("123519".to_string()));
    assert_eq!(gga.latitude, Some(4807.038));
    assert_eq!(gga.latitude_direction, Some('N'));
    assert_eq!(gga.longitude, Some(1131.000));
    assert_eq!(gga.longitude_direction, Some('E'));
    assert_eq!(gga.quality, Some(1));
    assert_eq!(gga.satellites, Some(8));
    assert_eq!(gga.altitude, Some(545.4));
    
    // Test conversion en degrés décimaux
    let lat_dec = gga.latitude_decimal();
    assert!(lat_dec.is_some());
    assert!((lat_dec.unwrap() - 48.1173).abs() < 0.01);
}

#[test]
fn test_parse_rmc_message() {
    let input = "$GPRMC,123519,A,4807.038,N,01131.000,E,022.4,084.4,230394,003.1,W*6A";
    let message = NmeaParser::parse_message(input).expect("Should parse");
    
    assert_eq!(message.message_type.identifier(), "GPRMC");
    
    let rmc = RmcMessage::from_nmea(&message).expect("Should parse RMC");
    assert_eq!(rmc.time, Some("123519".to_string()));
    assert_eq!(rmc.status, Some('A'));
    assert_eq!(rmc.latitude, Some(4807.038));
    assert_eq!(rmc.speed, Some(22.4));
    assert_eq!(rmc.course, Some(84.4));
    assert_eq!(rmc.date, Some("230394".to_string()));
}

#[test]
fn test_parse_gsa_message() {
    let input = "$GPGSA,A,3,04,05,,09,12,,,24,,,,,2.5,1.3,2.1*39";
    let message = NmeaParser::parse_message(input).expect("Should parse");
    
    assert_eq!(message.message_type.identifier(), "GPGSA");
    
    let gsa = GsaMessage::from_nmea(&message).expect("Should parse GSA");
    assert_eq!(gsa.selection_mode, Some('A'));
    assert_eq!(gsa.fix_mode, Some(3));
    assert_eq!(gsa.pdop, Some(2.5));
    assert_eq!(gsa.hdop, Some(1.3));
    assert_eq!(gsa.vdop, Some(2.1));
}

#[test]
fn test_parse_vtg_message() {
    let input = "$GPVTG,054.7,T,034.4,M,005.5,N,010.2,K*48";
    let message = NmeaParser::parse_message(input).expect("Should parse");
    
    assert_eq!(message.message_type.identifier(), "GPVTG");
    
    let vtg = VtgMessage::from_nmea(&message).expect("Should parse VTG");
    assert_eq!(vtg.course_true, Some(54.7));
    assert_eq!(vtg.course_magnetic, Some(34.4));
    assert_eq!(vtg.speed_knots, Some(5.5));
    assert_eq!(vtg.speed_kmh, Some(10.2));
}

#[test]
fn test_gga_validation() {
    let input = "$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47";
    let message = NmeaParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    assert!(result.is_ok(), "Valid GGA message should pass validation");
}

#[test]
fn test_rmc_validation() {
    let input = "$GPRMC,123519,A,4807.038,N,01131.000,E,022.4,084.4,230394,003.1,W*6A";
    let message = NmeaParser::parse_message(input).expect("Should parse");
    let result = message.validate();
    assert!(result.is_ok(), "Valid RMC message should pass validation");
}

