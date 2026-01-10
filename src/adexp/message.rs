use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::adexp::error::AdexpError;
use crate::adexp::types::MessageType;
use crate::adexp::validation;

/// Représente un message ADEXP complet
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdexpMessage {
    /// Type de message (déterminé depuis -TITLE)
    pub message_type: MessageType,
    
    /// Sections du message (clé = nom de section, valeur = champs)
    pub sections: HashMap<String, Section>,
    
    /// Données brutes du message (pour référence)
    pub raw: String,
}

/// Représente une section ADEXP avec ses champs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Section {
    /// Nom de la section
    pub name: String,
    
    /// Champs de la section (clé = nom du champ, valeur = valeur(s))
    pub fields: HashMap<String, Vec<String>>,
}

impl AdexpMessage {
    /// Crée un nouveau message ADEXP avec les données brutes.
    /// 
    /// Initialise un message vide (sans sections, type générique).
    /// Les sections et le type seront remplis lors du parsing.
    /// 
    /// # Arguments
    /// * `raw` - Message ADEXP brut (pour référence)
    /// 
    /// # Returns
    /// * Nouveau message ADEXP vide prêt à être parsé
    pub fn new(raw: String) -> Self {
        AdexpMessage {
            message_type: MessageType::Generic,
            sections: HashMap::new(),
            raw,
        }
    }
    
    /// Obtient une section par son nom.
    /// 
    /// La section vide ("") représente la section principale (hors blocs BEGIN/END).
    /// 
    /// # Arguments
    /// * `name` - Nom de la section (chaîne vide "" pour la section principale)
    /// 
    /// # Returns
    /// * `Some(&Section)` - Section trouvée
    /// * `None` - Section non trouvée
    pub fn get_section(&self, name: &str) -> Option<&Section> {
        self.sections.get(name)
    }
    
    /// Obtient un champ d'une section (peut avoir plusieurs valeurs).
    /// 
    /// Les champs ADEXP peuvent apparaître plusieurs fois avec différentes valeurs.
    /// Cette méthode retourne toutes les valeurs du champ.
    /// 
    /// # Arguments
    /// * `section` - Nom de la section ("" pour la section principale)
    /// * `field` - Nom du champ
    /// 
    /// # Returns
    /// * `Ok(Some(&Vec<String>))` - Champ trouvé avec ses valeurs
    /// * `Ok(None)` - Champ non trouvé
    /// * `Err(AdexpError::SectionNotFound)` - Section non trouvée
    pub fn get_field(&self, section: &str, field: &str) -> Result<Option<&Vec<String>>, AdexpError> {
        let section = self.sections.get(section)
            .ok_or_else(|| AdexpError::SectionNotFound(section.to_string()))?;
        Ok(section.fields.get(field))
    }
    
    /// Obtient la première valeur d'un champ.
    /// 
    /// Pour les champs qui apparaissent plusieurs fois, retourne uniquement
    /// la première occurrence.
    /// 
    /// # Arguments
    /// * `section` - Nom de la section ("" pour la section principale)
    /// * `field` - Nom du champ
    /// 
    /// # Returns
    /// * `Ok(Some(&String))` - Première valeur du champ trouvée
    /// * `Ok(None)` - Champ non trouvé
    /// * `Err(AdexpError::SectionNotFound)` - Section non trouvée
    pub fn get_field_value(&self, section: &str, field: &str) -> Result<Option<&String>, AdexpError> {
        match self.get_field(section, field)? {
            Some(values) => Ok(values.first()),
            None => Ok(None),
        }
    }
    
    /// Obtient le champ TITLE (requis pour tous les messages ADEXP).
    /// 
    /// Le TITLE détermine le type de message (FPL, CHG, CNL, DLA, etc.).
    /// 
    /// # Returns
    /// * `Ok(&String)` - Valeur du champ TITLE
    /// * `Err(AdexpError::MissingField)` - Champ TITLE manquant
    /// * `Err(AdexpError::SectionNotFound)` - Section principale non trouvée
    pub fn get_title(&self) -> Result<&String, AdexpError> {
        self.get_field_value("", "TITLE")
            .and_then(|v| v.ok_or_else(|| AdexpError::MissingField("TITLE".to_string())))
    }
    
    /// Valide la structure du message et les valeurs sémantiques des champs.
    /// 
    /// Effectue une validation complète selon ADEXP 3.4:
    /// - Vérifie que TITLE existe
    /// - Valide sémantiquement tous les champs (dates, codes ICAO, etc.)
    /// - Valide les structures composées (ADDR, VEC, RTEPTS, etc.)
    /// - Valide selon le type de message (FPL, CHG, CNL, DLA ont des champs requis spécifiques)
    /// 
    /// # Returns
    /// * `Ok(())` - Message valide
    /// * `Err(AdexpError)` - Erreur de validation (champ manquant, format invalide, etc.)
    /// 
    /// # Exemples
    /// ```
    /// use aftn::{AdexpParser, AdexpMessage};
    /// let input = "-ADEXP\n-TITLE FPL\n-ARCID ABC123\n-ADEP LFPG\n-ADES LFPB";
    /// let message = AdexpParser::parse_message(input)?;
    /// message.validate()?; // Valide la structure et la sémantique
    /// ```
    pub fn validate(&self) -> Result<(), AdexpError> {
        // Vérifier que TITLE existe
        self.get_title()?;
        
        // Validation sémantique de tous les champs
        self.validate_all_fields()?;
        
        // Validation spécifique selon le type de message
        match self.message_type {
            MessageType::FlightPlan => self.validate_fpl()?,
            MessageType::Change => self.validate_chg()?,
            MessageType::Delay => self.validate_dla()?,
            MessageType::Cancel => self.validate_cnl()?,
            _ => {}
        }
        
        Ok(())
    }
    
    /// Valide sémantiquement tous les champs du message
    fn validate_all_fields(&self) -> Result<(), AdexpError> {
        // Valider les champs de la section principale (vide)
        if let Some(section) = self.sections.get("") {
            // Valider les champs simples
            for (field_name, values) in &section.fields {
                for value in values {
                    validation::validate_field(field_name, value)?;
                }
            }
            // Valider les structures composées dans la section principale
            validation::validate_compound_fields_in_section(section)?;
        }
        
        // Valider les champs de toutes les autres sections
        for (section_name, section) in &self.sections {
            if section_name != "" {
                // Valider les champs simples
                for (field_name, values) in &section.fields {
                    for value in values {
                        validation::validate_field(field_name, value)?;
                    }
                }
                // Valider les structures composées dans cette section
                validation::validate_compound_fields_in_section(section)?;
            }
        }
        
        Ok(())
    }
    
    fn validate_fpl(&self) -> Result<(), AdexpError> {
        // Pour un FPL, vérifier les champs requis
        let required_fields = vec!["ARCID", "ADEP", "ADES"];
        for field in required_fields {
            if self.get_field_value("", field)?.is_none() {
                return Err(AdexpError::MissingField(field.to_string()));
            }
        }
        Ok(())
    }
    
    fn validate_chg(&self) -> Result<(), AdexpError> {
        // CHG doit avoir un ARCID
        if self.get_field_value("", "ARCID")?.is_none() {
            return Err(AdexpError::MissingField("ARCID".to_string()));
        }
        Ok(())
    }
    
    fn validate_dla(&self) -> Result<(), AdexpError> {
        // DLA doit avoir un ARCID
        if self.get_field_value("", "ARCID")?.is_none() {
            return Err(AdexpError::MissingField("ARCID".to_string()));
        }
        Ok(())
    }
    
    fn validate_cnl(&self) -> Result<(), AdexpError> {
        // CNL doit avoir un ARCID
        if self.get_field_value("", "ARCID")?.is_none() {
            return Err(AdexpError::MissingField("ARCID".to_string()));
        }
        Ok(())
    }
    
    /// Sérialise le message ADEXP en chaîne de caractères.
    /// 
    /// Reconstruit le message dans le format ADEXP standard sans espaces/tabulations supplémentaires.
    /// Format: `-ADEXP\n-TITLE [TITLE]\n-FIELD value\n...\n-BEGIN SECTION\n...\n-END`
    /// 
    /// # Returns
    /// * `String` - Message ADEXP sérialisé
    /// 
    /// # Exemples
    /// ```
    /// use aftn::{AdexpParser, AdexpMessage};
    /// let input = "-ADEXP\n-TITLE FPL\n-ARCID ABC123\n-ADEP LFPG\n-ADES LFPB";
    /// let message = AdexpParser::parse_message(input)?;
    /// let serialized = message.serialize();
    /// ```
    pub fn serialize(&self) -> String {
        let mut result = String::new();
        
        // Marqueur ADEXP
        result.push_str("-ADEXP\n");
        
        // Section principale (section vide "")
        if let Some(main_section) = self.sections.get("") {
            // Trier les champs pour un ordre cohérent (TITLE en premier si présent)
            let mut fields: Vec<(&String, &Vec<String>)> = main_section.fields.iter().collect();
            
            // Trier: TITLE en premier, puis les autres par ordre alphabétique
            fields.sort_by(|a, b| {
                if a.0 == "TITLE" {
                    std::cmp::Ordering::Less
                } else if b.0 == "TITLE" {
                    std::cmp::Ordering::Greater
                } else {
                    a.0.cmp(b.0)
                }
            });
            
            // Écrire les champs de la section principale
            for (field_name, values) in fields {
                for value in values {
                    result.push_str("-");
                    result.push_str(field_name);
                    if !value.is_empty() {
                        result.push(' ');
                        result.push_str(value.trim());
                    }
                    result.push('\n');
                }
            }
        }
        
        // Sections avec BEGIN/END (toutes les sections sauf la principale)
        let mut array_sections: Vec<(&String, &Section)> = self.sections.iter()
            .filter(|(name, _)| !name.is_empty())
            .collect();
        
        // Trier les sections par nom pour un ordre cohérent
        array_sections.sort_by(|a, b| a.0.cmp(b.0));
        
        for (section_name, section) in array_sections {
            // Marqueur BEGIN
            result.push_str("-BEGIN ");
            result.push_str(section_name);
            result.push('\n');
            
            // Trier les champs par nom
            let mut fields: Vec<(&String, &Vec<String>)> = section.fields.iter().collect();
            fields.sort_by(|a, b| a.0.cmp(b.0));
            
            // Écrire les champs de la section
            for (field_name, values) in fields {
                for value in values {
                    result.push_str("-");
                    result.push_str(field_name);
                    if !value.is_empty() {
                        result.push(' ');
                        result.push_str(value.trim());
                    }
                    result.push('\n');
                }
            }
            
            // Marqueur END avec nom de section
            result.push_str("-END ");
            result.push_str(section_name);
            result.push('\n');
        }
        
        result.trim_end().to_string()
    }
}

impl Section {
    /// Sérialise une section ADEXP en chaîne de caractères.
    /// 
    /// Utilisé pour sérialiser les sections dans les blocs BEGIN/END.
    /// 
    /// # Returns
    /// * `String` - Section sérialisée
    pub fn serialize(&self) -> String {
        let mut result = String::new();
        
        // Trier les champs par nom pour un ordre cohérent
        let mut fields: Vec<(&String, &Vec<String>)> = self.fields.iter().collect();
        fields.sort_by(|a, b| a.0.cmp(b.0));
        
        // Écrire les champs
        for (field_name, values) in fields {
            for value in values {
                result.push_str("-");
                result.push_str(field_name);
                if !value.is_empty() {
                    result.push(' ');
                    result.push_str(value.trim());
                }
                result.push('\n');
            }
        }
        
        result.trim_end().to_string()
    }
}

impl Section {
    /// Crée une nouvelle section
    pub fn new(name: String) -> Self {
        Section {
            name,
            fields: HashMap::new(),
        }
    }
    
    /// Ajoute un champ à la section
    pub fn add_field(&mut self, name: String, value: String) {
        self.fields.entry(name).or_insert_with(Vec::new).push(value);
    }
}

