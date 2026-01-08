use aftn::{AftnParser, AftnError};

fn main() -> Result<(), AftnError> {
    // Exemple de message AFTN
    let example = "GG LFPGYYYX LFPOYYYX 151230 NOTAM A1234/24 LFPG RWY 09/27 CLOSED";
    
    match AftnParser::parse_message(example) {
        Ok(message) => {
            println!("Message AFTN parsé avec succès:");
            println!("  Priorité: {}", message.priority);
            println!("  Origine: {}", message.addresses.origin);
            println!("  Destinations: {:?}", message.addresses.destinations);
            println!("  Date/Heure: {:02}/{:02}:{:02}", 
                message.transmission_time.day,
                message.transmission_time.hour,
                message.transmission_time.minute);
            println!("  Catégorie: {:?}", message.category);
            println!("  Corps: {}", message.body);
        }
        Err(e) => {
            eprintln!("Erreur de parsing: {}", e);
            return Err(e);
        }
    }
    
    Ok(())
}

