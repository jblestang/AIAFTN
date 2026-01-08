# Hiérarchie et Relations des Formats AFTN et ADEXP

Ce document décrit la hiérarchie et les relations entre les différents formats de messages aéronautiques implémentés dans ce projet.

## Vue d'ensemble

Le système de communication aéronautique utilise deux formats principaux pour l'échange de messages :

1. **AFTN 3.4** (Aeronautical Fixed Telecommunication Network) - Format de transmission réseau
2. **ADEXP 3.4** (Aeronautical Data Exchange Protocol) - Format de présentation des données ATS

Ces deux formats sont complémentaires et peuvent représenter les mêmes types de messages, mais avec des structures différentes.

## Hiérarchie des Formats

```
┌─────────────────────────────────────────────────────────────┐
│                    Messages Aéronautiques                   │
└─────────────────────────────────────────────────────────────┘
                              │
                ┌─────────────┴─────────────┐
                │                           │
        ┌───────▼────────┐          ┌───────▼────────┐
        │   AFTN 3.4     │          │   ADEXP 3.4    │
        │  (Réseau)      │          │  (Présentation)│
        └───────┬────────┘          └───────┬────────┘
                │                           │
                │                           │
    ┌───────────┴───────────┐   ┌───────────┴───────────┐
    │   Catégories AFTN     │   │   Types ADEXP         │
    │   (MessageCategory)   │   │   (MessageType)       │
    └───────────┬───────────┘   └───────────┬───────────┘
                │                           │
                │                           │
    ┌───────────┴───────────┐   ┌───────────┴───────────┐
    │   Sous-messages       │   │   Sections & Champs   │
    │   (SubMessage)        │   │   (Section & Fields)  │
    └───────────┬───────────┘   └───────────────────────┘
                │
    ┌───────────┴───────────────────────────────────────────┐
    │                                                         │
    ├─ Meteorological (Messages Météorologiques)            │
    │  ├─ NotamMessage (NOTAM)                              │
    │  ├─ MetarMessage (METAR)                              │
    │  ├─ TafMessage (TAF)                                  │
    │  ├─ SigmetMessage (SIGMET)                            │
    │  ├─ AirmetMessage (AIRMET)                            │
    │  ├─ AtisMessage (ATIS)                                │
    │  └─ VolmetMessage (VOLMET)                             │
    │                                                         │
    ├─ Flight Plan (Messages de Plan de Vol)                │
    │  ├─ FplMessage (FPL)                                  │
    │  ├─ ChgMessage (CHG)                                  │
    │  ├─ CnlMessage (CNL)                                  │
    │  ├─ DlaMessage (DLA)                                  │
    │  ├─ DepMessage (DEP)                                  │
    │  ├─ ArrMessage (ARR)                                  │
    │  ├─ EstMessage (EST)                                  │
    │  └─ SplMessage (SPL)                                  │
    │                                                         │
    ├─ Coordination (Messages de Coordination)               │
    │  ├─ CofMessage (COF/CDN)                              │
    │  ├─ ReqMessage (REQ)                                  │
    │  └─ AbiMessage (ABI)                                  │
    │                                                         │
    ├─ Position (Messages de Position)                      │
    │  └─ PosMessage (POS)                                  │
    │                                                         │
    ├─ Alerting (Messages d'Alerte)                         │
    │  └─ AlrMessage (ALR)                                  │
    │                                                         │
    └─ Operational (Messages Opérationnels)                 │
       ├─ OperationalMessage                                │
       └─ GenericMessage                                    │
```

## Format AFTN 3.4

### Structure

AFTN est le format de **transmission réseau** utilisé pour envoyer des messages via le réseau AFTN. Il a une structure fixe :

```
[PRIORITÉ] [ADRESSE_ORIGINE] [ADRESSES_DESTINATION] [DATE_HEURE] [CORPS]
```

### Composants

1. **Priorité** : Code de 2 lettres (GG, DD, FF, SS, KK, LL)
2. **Adresses** : Codes de 7-8 caractères (origine + destinations)
3. **Date/Heure** : Format JJHHMM
4. **Corps** : Contenu du message avec préfixe de catégorie

### Catégories de Messages AFTN

Les messages AFTN sont organisés en **catégories** (`MessageCategory`) :

#### Messages Météorologiques
- `NOTAM` - Notice to Airmen
- `METAR` - Meteorological Aerodrome Report
- `TAF` - Terminal Aerodrome Forecast
- `SIGMET` - Significant Meteorological Information
- `AIRMET` - Airmen's Meteorological Information
- `ATIS` - Automatic Terminal Information Service
- `VOLMET` - Meteorological Information for Aircraft in Flight

#### Messages de Plan de Vol
- `FlightPlan` (FPL) - Flight Plan
- `Change` (CHG) - Change (modification)
- `Cancel` (CNL) - Cancel (annulation)
- `Delay` (DLA) - Delay (retard)
- `Departure` (DEP) - Departure (départ)
- `Arrival` (ARR) - Arrival (arrivée)
- `Estimate` (EST) - Estimate (estimation)
- `SupplementaryFlightPlan` (SPL) - Supplementary Flight Plan
- `CurrentFlightPlan` (CPL) - Current Flight Plan
- `UpdateFlightPlan` (UPL) - Update Flight Plan

#### Messages de Coordination
- `Coordination` (COF/CDN) - Coordination
- `AdvanceBoundaryInformation` (ABI) - Advance Boundary Information
- `Request` (REQ) - Request (demande)
- `RequestFlightPlan` (RQP) - Request Flight Plan
- `RequestSupplementaryFlightPlan` (RQS) - Request Supplementary Flight Plan
- `Denial` (DEN) - Denial (refus)
- `Release` (RLS) - Release (libération)
- `Return` (RTN) - Return (retour)

#### Messages de Position et Rapports
- `Position` (POS) - Position Report
- `AircraftPositionList` (APL) - Aircraft Position List

#### Messages d'Alerte et d'Urgence
- `Alerting` (ALR) - Alerting
- `Urgency` (URG) - Urgency
- `RadioCommunicationFailure` (RCF) - Radio Communication Failure

#### Messages Spéciaux
- `OceanicClearance` (OCL) - Oceanic Clearance
- `Information` (INF) - Information
- `MessageAcknowledgement` (MAC) - Message Acknowledgement
- `Acceptance` (ACP) - Acceptance
- `TransferOfControl` (TCX) - Transfer of Control
- `AirReport` (AIREP) - Air Report

### Sous-messages AFTN

Chaque catégorie AFTN a un **sous-message** (`SubMessage`) qui parse et valide le corps du message. Les sous-messages sont organisés par catégorie :

#### Messages Météorologiques
- `NotamMessage` - Parse les messages NOTAM
- `MetarMessage` - Parse les messages METAR
- `TafMessage` - Parse les messages TAF
- `SigmetMessage` - Parse les messages SIGMET
- `AirmetMessage` - Parse les messages AIRMET
- `AtisMessage` - Parse les messages ATIS
- `VolmetMessage` - Parse les messages VOLMET

#### Messages de Plan de Vol
- `FplMessage` - Parse les messages FPL (Flight Plan)
- `ChgMessage` - Parse les messages CHG (Change)
- `CnlMessage` - Parse les messages CNL (Cancel)
- `DlaMessage` - Parse les messages DLA (Delay)
- `DepMessage` - Parse les messages DEP (Departure)
- `ArrMessage` - Parse les messages ARR (Arrival)
- `EstMessage` - Parse les messages EST (Estimate)
- `SplMessage` - Parse les messages SPL (Supplementary Flight Plan)

#### Messages de Coordination
- `CofMessage` - Parse les messages COF/CDN (Coordination)
- `ReqMessage` - Parse les messages REQ (Request)
- `AbiMessage` - Parse les messages ABI (Advance Boundary Information)

#### Messages de Position
- `PosMessage` - Parse les messages POS (Position Report)

#### Messages d'Alerte
- `AlrMessage` - Parse les messages ALR (Alerting)

#### Messages Opérationnels
- `OperationalMessage` - Parse les messages opérationnels génériques
- `GenericMessage` - Parse les messages génériques non catégorisés

Chaque sous-message implémente le trait `SubMessage` avec :
- `parse(body: &str)` - Parse le corps du message
- `validate()` - Valide le contenu sémantiquement (callsign, aérodromes, temps, etc.)
- `category()` - Retourne la catégorie du message

## Format ADEXP 3.4

### Structure

ADEXP est le format de **présentation des données ATS** (Air Traffic Services). Il utilise une structure hiérarchique avec sections et champs :

```
-ADEXP
-TITLE [TYPE_MESSAGE]
[SECTIONS]
[CHAMPS]
[-END]
```

### Composants

1. **Marqueur de début** : `-ADEXP`
2. **Titre** : `-TITLE` suivi du type de message
3. **Sections** : Groupes de champs optionnels
4. **Champs** : Données au format `-FIELD_NAME VALUE`
5. **Marqueur de fin** : `-END` (optionnel)

### Types de Messages ADEXP

Les messages ADEXP sont organisés en **types** (`MessageType`) :

#### Types Standards
- `FlightPlan` (FPL) - Flight Plan
- `Change` (CHG) - Change
- `Delay` (DLA) - Delay
- `Cancel` (CNL) - Cancel
- `Departure` (DEP) - Departure
- `Arrival` (ARR) - Arrival
- `Coordination` (COF) - Coordination
- `Request` (REQ) - Request
- `Estimate` (EST) - Estimate
- `Position` (POS) - Position
- `Logon` (LOG) - Logon
- `Logoff` (LOF) - Logoff

#### Types Spéciaux
- `Ifpl` (IFPL) - Initial Flight Plan
- `Abi` (ABI) - Advance Boundary Information
- `Ack` (ACK) - Acknowledge
- `Acp` (ACP) - Acceptance
- `Act` (ACT) - Activation
- Et 87 types standards au total

#### Titres Réservés (Annexe C)

141 titres de messages réservés par différents pays/organisations :
- **France** : CHGDEP, CNLARR, CNLDEP, CONFIDM, DEC, EATARR, ENDPROC, CDAFTX, CTARR, ACTDEP, etc.
- **Allemagne** : CHGMSG, CNLMSG, EVENT, FPLMSG, RWYMSG, TTIME
- **NM (Network Manager)** : ANM, CNLCOND, CNLREG, EXCOND, FSR, IFPDQ, etc.

### Sections ADEXP

Les sections permettent de regrouper des champs liés :

- **Section par défaut** : Champs sans section explicite
- **Sections nommées** : `-SECTION_NAME` suivi de champs
- **Sections de tableaux** : `-BEGIN SECTION_NAME` ... `-END SECTION_NAME`

### Champs ADEXP

Les champs ADEXP sont organisés par catégories :

#### Champs d'Identification
- `TITLE` - Type de message (requis)
- `ARCID` - Identifiant d'aéronef (callsign)
- `ARCTYP` - Type d'aéronef
- `REG` - Immatriculation
- `SEL` - Code SELCAL

#### Champs d'Aérodromes
- `ADEP` - Aérodrome de départ
- `ADES` - Aérodrome de destination
- `ALTRNT1`, `ALTRNT2` - Aérodromes alternatifs

#### Champs de Route
- `ROUTE` - Route complète
- `SID` - Standard Instrument Departure
- `STAR` - Standard Terminal Arrival Route
- `ARRPROC` - Procédure d'arrivée
- `DEPPROC` - Procédure de départ

#### Champs Temporels
- `EOBD` - Estimated Off-Block Date
- `EOBT` - Estimated Off-Block Time
- `ETO` - Estimated Time Over
- `ATOT` - Actual Time Over
- `ETA` - Estimated Time of Arrival
- `ACTARR` - Actual Arrival Time
- `ACTDEP` - Actual Departure Time

#### Champs de Niveau et Vitesse
- `RFL` - Requested Flight Level
- `CFL` - Current Flight Level
- `SPEED` - Vitesse
- `GROUNDSPEED` - Vitesse sol

#### Champs de Performance
- `PBN` - Performance Based Navigation
- `NAV` - Navigation equipment
- `COM` - Communication equipment
- `DAT` - Data link equipment
- `SUR` - Surveillance equipment

#### Champs Composés

Certains champs contiennent des sous-structures :

- **ADDR** : Adresses avec sous-champs `ADDR` et `FAC`
- **VEC** : Vecteur avec `TRACKANGLE`, `GROUNDSPEED`, `ALT`
- **RTEPTS** : Points de route avec `PT`, `PTID`, `LAT`, `LON`, `FL`, `ETO`, etc.
- **REFDATA** : Données de référence avec `IFPLID`, `ORIGIN`, `FAC`, `NETWORKTYPE`
- **ROUTE** : Route structurée

### Validation Sémantique ADEXP

Tous les champs ADEXP sont validés sémantiquement selon la spécification 3.4 :

- **Dates** : Format DDMMYY (6 chiffres)
- **Temps** : Format HHMM ou HHMMSS
- **Codes ICAO** : 4 lettres majuscules
- **Callsigns** : 1-7 caractères alphanumériques, commence par une lettre
- **Niveaux de vol** : Format F### (ex: F350)
- **Vitesses** : Format M### (Mach) ou N### (Nœuds)
- Et bien d'autres...

## Relations entre AFTN et ADEXP

### Correspondance des Types

| AFTN Category | ADEXP Type | Description |
|---------------|-----------|-------------|
| `FlightPlan` | `FlightPlan` | Plan de vol |
| `Change` | `Change` | Modification |
| `Cancel` | `Cancel` | Annulation |
| `Delay` | `Delay` | Retard |
| `Departure` | `Departure` | Départ |
| `Arrival` | `Arrival` | Arrivée |
| `Estimate` | `Estimate` | Estimation |
| `Coordination` | `Coordination` | Coordination |
| `Request` | `Request` | Demande |
| `Position` | `Position` | Position |

### Conversion

Un même message peut être représenté dans les deux formats :

**AFTN Format :**
```
GG LFPGYYYX LFPBYYYX 151200
FPL ABC123 IFR LFPG 1200 F350 M082 DCT LFPB
```

**ADEXP Format équivalent :**
```
-ADEXP
-TITLE FPL
-ARCID ABC123
-FLTRUL IFR
-ADEP LFPG
-ADES LFPB
-EOBT 1200
-RFL F350
-SPEED M082
-ROUTE DCT
```

## Hiérarchie d'Implémentation

### Structure du Code

```
src/
├── aftn/                  # Module AFTN
│   ├── message.rs        # Structure AftnMessage
│   ├── parser.rs         # Parser AFTN
│   ├── validation.rs     # Validation sémantique AFTN
│   ├── submessages.rs    # Trait SubMessage et dispatcher
│   ├── aftn.pest         # Grammaire PEST AFTN
│   │
│   └── categories/       # Catégories et sous-messages
│       ├── mod.rs        # Module principal
│       ├── category_enum.rs  # Enum MessageCategory
│       │
│       ├── meteorological/  # Messages météorologiques
│       │   ├── mod.rs
│       │   ├── notam.rs + .pest
│       │   ├── metar.rs + .pest
│       │   ├── taf.rs + .pest
│       │   ├── sigmet.rs + .pest
│       │   ├── airmet.rs + .pest
│       │   ├── atis.rs + .pest
│       │   └── volmet.rs + .pest
│       │
│       ├── flight_plan/   # Messages de plan de vol
│       │   ├── mod.rs
│       │   ├── fpl.rs + .pest
│       │   ├── chg.rs
│       │   ├── cnl.rs
│       │   ├── dla.rs
│       │   ├── dep.rs
│       │   ├── arr.rs
│       │   ├── est.rs
│       │   └── spl.rs
│       │
│       ├── coordination/ # Messages de coordination
│       │   ├── mod.rs
│       │   ├── cof.rs
│       │   ├── req.rs
│       │   └── abi.rs
│       │
│       ├── position/     # Messages de position
│       │   ├── mod.rs
│       │   └── pos.rs + .pest
│       │
│       ├── alerting/     # Messages d'alerte
│       │   ├── mod.rs
│       │   └── alr.rs
│       │
│       └── operational/  # Messages opérationnels
│           ├── mod.rs
│           ├── operational.rs + .pest
│           └── generic.rs
│
└── adexp/                # Format ADEXP
    ├── message.rs        # Structure AdexpMessage
    ├── types.rs          # MessageType (ADEXP)
    ├── parser.rs         # Parser ADEXP
    ├── fields.rs         # Définition des champs
    ├── validation.rs     # Validation sémantique
    ├── adexp.pest        # Grammaire PEST ADEXP
    └── error.rs          # Erreurs ADEXP
```

### Flux de Parsing

#### AFTN
```
Input String
    ↓
AftnParser::parse_message()
    ↓
Parse header (priorité, adresses, date/heure)
    ↓
Parse body → MessageCategory::from_message_id()
    ↓
SubMessage::parse() (selon catégorie)
    ↓
SubMessage::validate()
    ↓
AftnMessage
```

#### ADEXP
```
Input String
    ↓
AdexpParser::parse_message()
    ↓
Extract BEGIN/END blocks (parsing manuel)
    ↓
Parse avec PEST grammar
    ↓
Parse sections et champs
    ↓
MessageType::from_title()
    ↓
AdexpMessage::validate()
    ↓
AdexpMessage::validate_all_fields() (validation sémantique)
    ↓
AdexpMessage
```

## Exemples de Correspondance

### Message FPL

**AFTN :**
```
GG LFPGYYYX LFPBYYYX 151200
FPL ABC123 IFR LFPG 1200 F350 M082 DCT LFPB
```

**ADEXP équivalent :**
```
-ADEXP
-TITLE FPL
-ARCID ABC123
-FLTRUL IFR
-ADEP LFPG
-ADES LFPB
-EOBD 15
-EOBT 1200
-RFL F350
-SPEED M082
-ROUTE DCT
```

### Message CNL

**AFTN :**
```
GG LFPGYYYX LFPBYYYX 151200
CNL ABC123
```

**ADEXP équivalent :**
```
-ADEXP
-TITLE CNL
-ARCID ABC123
```

## Validation

### AFTN
- Validation de la structure (priorité, adresses, date/heure)
- Validation du format du corps par catégorie
- Validation sémantique par sous-message (ex: callsign dans CNL)

### ADEXP
- Validation de la structure (marqueurs, sections)
- Validation sémantique de tous les champs
- Validation des champs composés (ADDR, VEC, RTEPTS, etc.)
- Validation spécifique par type de message

## Références

- **AFTN 3.4** : Spécification AFTN version 3.4
- **ADEXP 3.4** : [EUROCONTROL ADEXP 3.4 Specification](https://www.eurocontrol.int/sites/default/files/2023-06/eurocontrol-released-specification-adexp-3-4.pdf)
- **Annexe C** : Liste des titres de messages réservés ADEXP 3.4

## Conclusion

AFTN et ADEXP sont deux formats complémentaires :
- **AFTN** : Format de transmission réseau avec structure fixe
- **ADEXP** : Format de présentation avec structure flexible et hiérarchique

Les deux formats peuvent représenter les mêmes types de messages, mais avec des structures différentes adaptées à leurs usages respectifs.

