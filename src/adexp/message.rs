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
    /// Crée un nouveau message ADEXP
    pub fn new(raw: String) -> Self {
        AdexpMessage {
            message_type: MessageType::Generic,
            sections: HashMap::new(),
            raw,
        }
    }
    
    /// Obtient une section par son nom
    pub fn get_section(&self, name: &str) -> Option<&Section> {
        self.sections.get(name)
    }
    
    /// Obtient un champ d'une section
    pub fn get_field(&self, section: &str, field: &str) -> Result<Option<&Vec<String>>, AdexpError> {
        let section = self.sections.get(section)
            .ok_or_else(|| AdexpError::SectionNotFound(section.to_string()))?;
        Ok(section.fields.get(field))
    }
    
    /// Obtient la première valeur d'un champ
    pub fn get_field_value(&self, section: &str, field: &str) -> Result<Option<&String>, AdexpError> {
        match self.get_field(section, field)? {
            Some(values) => Ok(values.first()),
            None => Ok(None),
        }
    }
    
    /// Obtient le champ TITLE (requis)
    pub fn get_title(&self) -> Result<&String, AdexpError> {
        self.get_field_value("", "TITLE")
            .and_then(|v| v.ok_or_else(|| AdexpError::MissingField("TITLE".to_string())))
    }
    
    /// Valide la structure du message et les valeurs sémantiques des champs
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

