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
    ACTARR,    // Actual arrival time
    ACTDEP,    // Actual departure time
    
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

/// Structure pour un champ ADDR (Address compound field)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddrField {
    pub address: String,
    pub facility: Option<String>,
}

/// Structure pour un champ VEC (Vector compound field)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VecField {
    pub track_angle: Option<String>,
    pub ground_speed: Option<String>,
    pub altitude: Option<String>,
}

/// Structure pour un point de route (RTEPTS)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoutePoint {
    pub pt: Option<String>,           // Point identifier
    pub ptid: Option<String>,         // Point identifier alternative
    pub lat: Option<String>,          // Latitude
    pub lon: Option<String>,          // Longitude
    pub fl: Option<String>,           // Flight level
    pub eto: Option<String>,          // Estimated time over
    pub atot: Option<String>,         // Actual time over
    pub speed: Option<String>,        // Speed
    pub alt: Option<String>,          // Altitude
    pub dist: Option<String>,         // Distance
    pub reason: Option<String>,       // Reason
    pub ahead: Option<String>,        // Ahead
}

/// Structure pour un champ REFDATA (Reference data compound field)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RefDataField {
    pub ifplid: Option<String>,
    pub origin: Option<String>,
    pub fac: Option<String>,
    pub networktype: Option<String>,
}

/// Structure pour un champ CSTAT (Current status compound field)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CstatField {
    pub status: Option<String>,
    pub reason: Option<String>,
    pub statreason: Option<String>,
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
            "ROUTE" | "SID" | "STAR" | "ATSRT" | "ARRPROC" | "DEPPROC" |
            // Temps
            "EOBD" | "EOBT" | "ETO" | "ATOT" | "ETA" | "EDA" | "AMANTIME" | "TOM" |
            "ATD" | "ATAD" | "ATOD" | "ATOA" | "ATOTD" | "ATOTA" | "ACTARR" | "ACTDEP" |
            // Niveaux de vol
            "RFL" | "CFL" | "AFL" | "TFL" |
            // Vitesse
            "SPEED" | "GROUNDSPEED" | "TAS" | "MACH" |
            // Météorologie
            "WINDIR" | "WINDSPEED" | "AIRTEMP" | "QNH" | "QFE" |
            // Performance et navigation
            "PBN" | "FLTRUL" | "FLTTYP" | "NAV" | "COM" | "DAT" | "SUR" |
            // Coordination et identification
            "IFPLID" | "ORIGIN" | "NETWORKTYPE" | "FAC" | "TITLE" | "SRC" |
            // Codes SSR/Mode S
            "COD" |
            // Statut
            "CDMSTATUS" | "IFPSDISCREPANCY" | "CSTAT" |
            // Référence
            "REFDATA" |
            // Points de route
            "RTEPTS" | "PT" | "PTID" | "FL" | "SEQPT" |
            // Champs additionnels
            "DEPAPTYPE" | "DEPARCTYP" | "OBTLIMIT" | "OBT" |
            "PRF1" | "PRF2" | "PRF3" | "PRF4" |
            "TRACKANGLE" | "TTO" | "TTLEET" | "FILTIM" | "EET" |
            "WKTRC" | "MFX" | "PTDLE" | "CMLTSP" | "VEC" |
            // Champs réservés et additionnels de la spécification 3.4
            "RELDIST" | "HEXADDR" | "OPR" | "OPRICAO" | "PER" | "PERICAO" |
            "ALTN" | "ALTT" | "ALTZ" | "ALTA" | "ALTS" | "ALTR" |
            "RMK" | "RMKS" | "COMMENT" | "TEXT" | "CODE" | "CODEICAO" |
            "GEO" | "GEONAME" | "LAT" | "LON" | "ALT" | "ALTNZ" | "DIST" |
            "NUM" | "TIMEHHMM" | "TIMEHHMMSS" | "DATE" | "REASON" | "AHEAD" | "STATREASON"
        )
    }
    
    /// Vérifie si un nom de champ est un champ de base valide
    /// Selon la spécification ADEXP 3.4 d'EUROCONTROL
    pub fn is_basic_field(field_name: &str) -> bool {
        matches!(field_name,
            // Identifiants
            "NUM" | "PT" | "PTID" |
            // Temps
            "TIMEHHMM" | "TIMEHHMMSS" | "DATE" | "TIME" |
            // Géographie
            "GEONAME" | "GEO" | "LAT" | "LON" |
            // Altitude et distance
            "ALT" | "ALTNZ" | "DIST" | "RELDIST" | "AHEAD" |
            // Autres
            "REASON" | "STATREASON" | "TEXT" | "CODE" |
            // Champs additionnels
            "SENDER" | "RECVR" | "CMLTSP" | "COMMENT" | "RMK" | "RMKS"
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

