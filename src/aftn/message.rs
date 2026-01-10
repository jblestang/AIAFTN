use serde::{Deserialize, Serialize};
use crate::aftn::error::AftnError;
use crate::aftn::categories::MessageCategory;

/// Représente un message AFTN complet
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AftnMessage {
    /// Priorité du message (GG, DD, FF, SS, etc.)
    pub priority: String,
    
    /// Adresses d'origine et de destination
    pub addresses: Addresses,
    
    /// Catégorie du message
    pub category: MessageCategory,
    
    /// Date et heure de transmission
    pub transmission_time: TransmissionTime,
    
    /// Corps du message
    pub body: String,
    
    /// Numéro de séquence (optionnel)
    pub sequence_number: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Addresses {
    /// Adresse d'origine (8 caractères)
    pub origin: String,
    
    /// Adresses de destination (8 caractères chacune, séparées par des espaces)
    pub destinations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransmissionTime {
    /// Jour du mois (01-31)
    pub day: u8,
    
    /// Heure (00-23)
    pub hour: u8,
    
    /// Minute (00-59)
    pub minute: u8,
}

impl AftnMessage {
    /// Valide la structure du message selon la spécification AFTN 3.4.
    /// 
    /// Vérifie:
    /// - La priorité est valide (GG, DD, FF, SS, KK, LL)
    /// - Les adresses d'origine et de destination ont 7-8 caractères
    /// - La date/heure est valide (jour 01-31, heure 00-23, minute 00-59)
    /// 
    /// # Returns
    /// * `Ok(())` - Message valide
    /// * `Err(AftnError)` - Erreur de validation (priorité, adresse, ou date/heure invalide)
    /// 
    /// # Exemples
    /// ```
    /// use aftn::{AftnParser, AftnMessage};
    /// let input = "GG LFPGYYYX LFPOYYYX 151230 NOTAM A1234/24";
    /// let message = AftnParser::parse_message(input)?;
    /// message.validate()?; // Valide la structure
    /// ```
    pub fn validate(&self) -> Result<(), AftnError> {
        // Validation de la priorité selon AFTN 3.4
        let valid_priorities = ["GG", "DD", "FF", "SS", "KK", "LL"];
        if !valid_priorities.contains(&self.priority.as_str()) {
            return Err(AftnError::InvalidPriority(self.priority.clone()));
        }
        
        // Validation de l'adresse d'origine (7-8 caractères selon spécification AFTN 3.4)
        if self.addresses.origin.len() < 7 || self.addresses.origin.len() > 8 {
            return Err(AftnError::InvalidAddress(format!(
                "Origin address must be 7-8 characters, got {}",
                self.addresses.origin.len()
            )));
        }
        
        // Validation des adresses de destination (7-8 caractères)
        for dest in &self.addresses.destinations {
            if dest.len() < 7 || dest.len() > 8 {
                return Err(AftnError::InvalidAddress(format!(
                    "Destination address must be 7-8 characters, got {}",
                    dest.len()
                )));
            }
        }
        
        // Validation de la date/heure
        if self.transmission_time.day > 31 {
            return Err(AftnError::InvalidDateTime(format!(
                "Day must be between 01-31, got {}",
                self.transmission_time.day
            )));
        }
        
        if self.transmission_time.hour > 23 {
            return Err(AftnError::InvalidDateTime(format!(
                "Hour must be between 00-23, got {}",
                self.transmission_time.hour
            )));
        }
        
        if self.transmission_time.minute > 59 {
            return Err(AftnError::InvalidDateTime(format!(
                "Minute must be between 00-59, got {}",
                self.transmission_time.minute
            )));
        }
        
        Ok(())
    }
    
    /// Sérialise le message AFTN en chaîne de caractères.
    /// 
    /// Reconstruit le message dans le format AFTN standard sans espaces/tabulations supplémentaires.
    /// Format: `[PRIORITY] [ORIGIN] [DEST1] [DEST2] ... [DDHHMM] [BODY] [/SEQ NUMBER]`
    /// 
    /// # Returns
    /// * `String` - Message AFTN sérialisé
    /// 
    /// # Exemples
    /// ```
    /// use aftn::{AftnParser, AftnMessage};
    /// let input = "GG LFPGYYYX LFPOYYYX 151230 NOTAM A1234/24";
    /// let message = AftnParser::parse_message(input)?;
    /// let serialized = message.serialize();
    /// assert_eq!(serialized, "GG LFPGYYYX LFPOYYYX 151230 NOTAM A1234/24");
    /// ```
    pub fn serialize(&self) -> String {
        let mut result = String::new();
        
        // Priorité
        result.push_str(&self.priority);
        result.push(' ');
        
        // Adresse d'origine
        result.push_str(&self.addresses.origin);
        result.push(' ');
        
        // Adresses de destination (séparées par des espaces)
        for (idx, dest) in self.addresses.destinations.iter().enumerate() {
            if idx > 0 {
                result.push(' ');
            }
            result.push_str(dest);
        }
        result.push(' ');
        
        // Date et heure (format: DDHHMM)
        result.push_str(&format!("{:02}{:02}{:02}", 
            self.transmission_time.day,
            self.transmission_time.hour,
            self.transmission_time.minute
        ));
        result.push(' ');
        
        // Corps du message
        result.push_str(&self.body.trim());
        
        // Numéro de séquence (optionnel)
        if let Some(seq) = &self.sequence_number {
            result.push_str(" /SEQ ");
            result.push_str(seq);
        }
        
        result
    }
}

