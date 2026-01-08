//! Définition de tous les champs ADEXP selon la spécification 3.4 d'EUROCONTROL
//! Référence: https://www.eurocontrol.int/sites/default/files/2023-06/eurocontrol-released-specification-adexp-3-4.pdf

use serde::{Deserialize, Serialize};

/// Champs primaires ADEXP selon la spécification 3.4
/// Ces champs sont les principaux identifiants et informations de base
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PrimaryField {
    // Champs d'adresse et de communication
    ADDR,      // Address list
    ADEP,      // Aerodrome of departure
    ADES,      // Aerodrome of destination
    ALTRNT1,   // First alternate aerodrome
    ALTRNT2,   // Second alternate aerodrome
    
    // Champs d'identification du vol
    ARCID,     // Aircraft identification
    ARCTYP,    // Aircraft type
    CEQPT,     // Communication equipment
    REG,       // Registration marks
    SEL,       // SELCAL code
    
    // Champs de route et navigation
    ROUTE,     // Route
    SID,       // Standard Instrument Departure
    STAR,      // Standard Instrument Arrival
    ATSRT,     // ATS route
    
    // Champs de temps
    EOBD,      // Estimated off-block date
    EOBT,      // Estimated off-block time
    ETO,       // Estimated time over
    ATOT,      // Actual time over
    ETA,       // Estimated time of arrival
    EDA,       // Estimated date of arrival
    
    // Champs de niveau de vol
    RFL,       // Requested flight level
    CFL,       // Cleared flight level
    
    // Champs de vitesse
    SPEED,     // Speed
    GROUNDSPEED, // Ground speed
    
    // Champs météorologiques
    WINDIR,    // Wind direction
    WINDSPEED, // Wind speed
    AIRTEMP,   // Air temperature
    
    // Champs de performance
    PBN,       // Performance based navigation
    FLTRUL,    // Flight rules
    FLTTYP,    // Flight type
    
    // Champs de coordination
    IFPLID,    // IFPS flight plan identifier
    ORIGIN,    // Origin facility
    NETWORKTYPE, // Network type
    FAC,       // Facility
    
    // Champs de statut
    CDMSTATUS, // CDM status
    IFPSDISCREPANCY, // IFPS discrepancy
    
    // Champs de référence
    REFDATA,   // Reference data
    
    // Champs de points de route
    RTEPTS,    // Route points section
    PT,        // Point
    PTID,      // Point identifier
    FL,        // Flight level
    
    // Champs additionnels
    DEPAPTYPE, // Departure aerodrome type
    DEPARCTYP, // Departure aircraft type
    OBTLIMIT,  // Off-block time limit
    PRF1, PRF2, PRF3, PRF4, // Performance fields
    TRACKANGLE, // Track angle
    TTO,       // Time to object
    TTLEET,    // Total estimated elapsed time
    FILTIM,    // Filing time
    SEQPT,     // Sequence point
    WKTRC,     // Wake turbulence category
    SRC,       // Source
    MFX,       // Message fix
    PTDLE,     // Point delay
    CMLTSP,    // Communication list
}

/// Champs de base ADEXP (basic fields)
/// Ces champs ont une syntaxe simple et directe
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BasicField {
    // Identifiants
    NUM,       // Number
    PT,        // Point
    
    // Temps
    TIMEHHMM,  // Time in HHMM format
    TIMEHHMMSS, // Time in HHMMSS format
    DATE,      // Date
    
    // Géographie
    GEONAME,   // Geographical name
    LAT,       // Latitude
    LON,       // Longitude
    
    // Navigation
    ALT,       // Altitude
    ALTNZ,     // Altitude non-zero
    DIST,      // Distance
    
    // Autres
    REASON,    // Reason
    AHEAD,     // Ahead
    STATREASON, // Status reason
}

/// Champs composés ADEXP (compound fields)
/// Ces champs contiennent des sous-champs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CompoundField {
    ADDR,      // Address compound field
    REFDATA,   // Reference data compound field
    CSTAT,     // Current status compound field
    VEC,       // Vector compound field
    RTEPTS,    // Route points compound field
}

/// Liste complète des champs ADEXP selon la spécification 3.4
pub struct AdexpFields;

impl AdexpFields {
    /// Vérifie si un nom de champ est un champ primaire valide
    /// Selon la spécification ADEXP 3.4 d'EUROCONTROL
    pub fn is_primary_field(field_name: &str) -> bool {
        matches!(field_name, 
            // Adresses et aérodromes
            "ADDR" | "ADEP" | "ADES" | "ALTRNT1" | "ALTRNT2" |
            // Identification du vol
            "ARCID" | "ARCTYP" | "CEQPT" | "REG" | "SEL" |
            // Route et navigation
            "ROUTE" | "SID" | "STAR" | "ATSRT" | "ARRPROC" |
            // Temps
            "EOBD" | "EOBT" | "ETO" | "ATOT" | "ETA" | "EDA" | "AMANTIME" | "TOM" |
            // Niveaux de vol
            "RFL" | "CFL" |
            // Vitesse
            "SPEED" | "GROUNDSPEED" |
            // Météorologie
            "WINDIR" | "WINDSPEED" | "AIRTEMP" |
            // Performance et navigation
            "PBN" | "FLTRUL" | "FLTTYP" |
            // Coordination et identification
            "IFPLID" | "ORIGIN" | "NETWORKTYPE" | "FAC" | "TITLE" |
            // Statut
            "CDMSTATUS" | "IFPSDISCREPANCY" |
            // Référence
            "REFDATA" |
            // Points de route
            "RTEPTS" | "PT" | "PTID" | "FL" |
            // Champs additionnels
            "DEPAPTYPE" | "DEPARCTYP" | "OBTLIMIT" |
            "PRF1" | "PRF2" | "PRF3" | "PRF4" |
            "TRACKANGLE" | "TTO" | "TTLEET" | "FILTIM" |
            "SEQPT" | "WKTRC" | "SRC" | "MFX" | "PTDLE" | "CMLTSP" |
            // Champs réservés et additionnels de la spécification 3.4
            "RELDIST" | "HEXADDR"
        )
    }
    
    /// Vérifie si un nom de champ est un champ de base valide
    /// Selon la spécification ADEXP 3.4 d'EUROCONTROL
    pub fn is_basic_field(field_name: &str) -> bool {
        matches!(field_name,
            // Identifiants
            "NUM" | "PT" |
            // Temps
            "TIMEHHMM" | "TIMEHHMMSS" | "DATE" |
            // Géographie
            "GEONAME" | "LAT" | "LON" |
            // Altitude et distance
            "ALT" | "ALTNZ" | "DIST" | "RELDIST" |
            // Autres
            "REASON" | "AHEAD" | "STATREASON" |
            // Champs additionnels
            "SENDER" | "RECVR" | "CMLTSP"
        )
    }
    
    /// Vérifie si un nom de champ est un champ composé valide
    pub fn is_compound_field(field_name: &str) -> bool {
        matches!(field_name,
            "ADDR" | "REFDATA" | "CSTAT" | "VEC" | "RTEPTS"
        )
    }
    
    /// Vérifie si un nom de champ est un champ ADEXP valide
    pub fn is_valid_field(field_name: &str) -> bool {
        Self::is_primary_field(field_name) ||
        Self::is_basic_field(field_name) ||
        Self::is_compound_field(field_name)
    }
}

