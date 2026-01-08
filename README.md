# AFTN 3.4 et ADEXP Parser en Rust

Implémentation progressive de parseurs AFTN (Aeronautical Fixed Telecommunication Network) version 3.4 et ADEXP (Aeronautical Data Exchange Protocol) en Rust utilisant PEST.

## Structure du projet

Le projet est organisé de manière modulaire pour gérer la complexité des différents types de messages :

### AFTN
- `src/parser.rs` : Parser principal utilisant PEST
- `src/message.rs` : Structures de données pour les messages AFTN
- `src/categories.rs` : Gestion des différentes catégories de messages
- `src/error.rs` : Gestion des erreurs
- `src/aftn.pest` : Grammaire PEST pour AFTN 3.4

### ADEXP
- `src/adexp/parser.rs` : Parser ADEXP utilisant PEST
- `src/adexp/message.rs` : Structures de données pour les messages ADEXP
- `src/adexp/types.rs` : Gestion des différents types de messages ADEXP
- `src/adexp/error.rs` : Gestion des erreurs ADEXP
- `src/adexp/adexp.pest` : Grammaire PEST pour ADEXP

## Catégories de messages supportées

### Messages météorologiques
- **NOTAM** : Notice to Airmen
- **METAR** : Meteorological Aerodrome Report
- **TAF** : Terminal Aerodrome Forecast
- **SIGMET** : Significant Meteorological Information
- **AIRMET** : Airmen's Meteorological Information
- **ATIS** : Automatic Terminal Information Service
- **VOLMET** : Meteorological Information for Aircraft in Flight

### Messages de plan de vol
- **FPL** : Flight Plan
- **CHG** : Change (modification de plan de vol)
- **CNL** : Cancel (annulation de plan de vol)
- **DLA** : Delay (retard)
- **DEP** : Departure (départ)
- **ARR** : Arrival (arrivée)
- **EST** : Estimate (estimation)
- **SPL** : Supplementary Flight Plan
- **CPL** : Current Flight Plan
- **UPL** : Update Flight Plan

### Messages de coordination
- **COF/CDN** : Coordination
- **ABI** : Advance Boundary Information
- **REQ** : Request (demande)
- **RQP** : Request Flight Plan
- **RQS** : Request Supplementary Flight Plan
- **DEN** : Denial (refus)
- **RLS** : Release (libération)
- **RTN** : Return (retour)

### Messages de position et rapports
- **POS** : Position Report
- **APL** : Aircraft Position List

### Messages d'alerte et d'urgence
- **ALR** : Alerting
- **URG** : Urgency
- **RCF** : Radio Communication Failure

### Messages spéciaux
- **OCL** : Oceanic Clearance
- **INF** : Information
- **MAC** : Message Acknowledgement
- **ACP** : Acceptance (acceptation de plan de vol)
- **TCX** : Transfer of Control
- **AIREP** : Air Report (rapport aérien)

### Messages génériques
- **Operational** : Messages opérationnels divers
- **Generic** : Messages génériques non catégorisés

## Format des messages AFTN

Un message AFTN suit généralement cette structure :

```
[PRIORITÉ] [ORIGINE] [DESTINATIONS...] [JJHHMM] [CORPS] [/SEQ NUMÉRO]
```

Exemple :
```
GG LFPGYYYX LFPOYYYX 151230 NOTAM A1234/24 LFPG RWY 09/27 CLOSED
```

### Priorités valides
- GG : Général
- DD : Urgent
- FF : Flash
- SS : Sécurité
- KK : Opérationnel
- LL : Local

## Tests

### Tests unitaires

```bash
cargo test --lib
```

### Tests d'intégration

```bash
cargo test --test integration_tests
cargo test --test category_tests
cargo test --test robustness_tests
```

### Tous les tests

```bash
cargo test
```

## Fuzzing

Le projet inclut une configuration pour le fuzzing agressif du parseur afin de tester sa robustesse.

### Installation de cargo-fuzz

```bash
cargo install cargo-fuzz
```

### Exécution du fuzzing

#### Fuzzing AFTN
```bash
cd fuzz
cargo fuzz run fuzz_parser
```

#### Fuzzing ADEXP
```bash
cd fuzz
cargo fuzz run fuzz_adexp_parser
```

#### Fuzzing des deux
```bash
make fuzz-all
```

### Fuzzing avec corpus personnalisé

- **AFTN** : Créez un dossier `fuzz/corpus/fuzz_parser/` et placez-y des exemples de messages AFTN (valides et invalides)
- **ADEXP** : Créez un dossier `fuzz/corpus/fuzz_adexp_parser/` et placez-y des exemples de messages ADEXP (valides et invalides)

Le script `scripts/generate_fuzz_corpus.sh` génère automatiquement un corpus de base pour les deux formats.

## Utilisation

### Exemple AFTN

```rust
use aftn::AftnParser;

let input = "GG LFPGYYYX LFPOYYYX 151230 NOTAM A1234/24 LFPG RWY 09/27 CLOSED";
match AftnParser::parse_message(input) {
    Ok(message) => {
        println!("Priorité: {}", message.priority);
        println!("Origine: {}", message.addresses.origin);
        println!("Catégorie: {:?}", message.category);
    }
    Err(e) => eprintln!("Erreur: {}", e),
}
```

### Exemple ADEXP

```rust
use aftn::AdexpParser;

let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
";
match AdexpParser::parse_message(input) {
    Ok(message) => {
        println!("Type: {:?}", message.message_type);
        if let Ok(Some(arcid)) = message.get_field_value("", "ARCID") {
            println!("ARCID: {}", arcid);
        }
    }
    Err(e) => eprintln!("Erreur: {}", e),
}
```

## Types de messages ADEXP supportés

- **FPL** : Flight Plan
- **CHG** : Change
- **DLA** : Delay
- **CNL** : Cancel
- **DEP** : Departure
- **ARR** : Arrival
- **COF** : Coordination
- **REQ** : Request
- **EST** : Estimate
- **POS** : Position
- **LOG** : Logon
- **LOF** : Logoff
- **Generic** : Messages génériques non catégorisés

### Titres de messages réservés (Annexe C)

**Tous les 141 titres de messages réservés** selon l'Annexe C de la spécification ADEXP 3.4 sont implémentés, incluant :

- **Titres réservés par la France** (majorité) : CHGDEP, CNLARR, CNLDEP, CONFIDM, DEC, EATARR, ENDPROC, CDAFTX, CTARR, ACTDEP, et 120+ autres
- **Titres réservés par l'Allemagne** : CHGMSG, CNLMSG, EVENT, FPLMSG, RWYMSG, TTIME
- **Titres réservés par NM (Network Manager)** : ANM, CNLCOND, CNLREG, EXCOND, FSR, IFPDQ, IFPDQR, IFPDSQ, IFPDSQR, IRQS, ISPL, MODCOND, MODREG, MRA, MRCNL, MRMOD, MRR, NEWREG, NTA, NTACNL, NTAMOD, OLRA, OLRCNL, OLRMOD, RFPDQ, RFPDQR, RFPDSQ, RFPDSQR, RRA, RREC, RRN, RSUS

Pour la liste complète, consultez `src/adexp/types.rs` ou l'Annexe C de la spécification ADEXP 3.4 d'EUROCONTROL.

## Validation sémantique

Le parser ADEXP inclut une validation sémantique complète des champs selon la spécification ADEXP 3.4 :

### Types de validation implémentés

- **Dates** : Format DDMMYY, validation des jours/mois valides
- **Timestamps** : Formats HHMM et HHMMSS, validation des heures/minutes/secondes
- **Codes aérodrome** : Format ICAO (4 lettres majuscules)
- **Identifiants d'aéronef** : 1-7 caractères alphanumériques, commençant par une lettre
- **Niveaux de vol** : Format FLXXX ou XXX (000-999)
- **Vitesses** : Formats numériques, MACH (MXXX), avec unités
- **Coordonnées géographiques** : Latitude (-90 à +90), Longitude (-180 à +180)
- **Codes SSR** : Format octal (4 chiffres 0-7) ou Mode S (6 hexadécimaux)
- **Immatriculations** : Format variable selon pays
- **Codes SELCAL** : 4 lettres (A-S sauf I, N, O, Q)
- **Flight Rules** (FLTRUL) : I, V, Y, Z
- **Flight Type** (FLTTYP) : S, N, G, M, X
- **PBN** : Codes Performance Based Navigation (A1, B1-B6, C1-C4, D1-D4, L1, O1-O4, S1-S2, T1-T2)
- **Equipment codes** : NAV, COM, DAT, SUR (codes ICAO)
- **Aircraft Type** (ARCTYP) : Code ICAO (2-4 caractères)
- **Wind Direction** (WINDIR) : 000-360
- **Wind Speed** (WINDSPEED) : Format numérique
- **Pressure** (QNH, QFE) : 800-1100 hPa
- **Air Temperature** (AIRTEMP) : -80 à +60°C
- **Track Angle** (TRACKANGLE) : 001-360
- **Altitude** (ALT) : Format numérique ou FL
- **Distance** (DIST) : Format numérique
- **Hex Address** (HEXADDR) : 6 caractères hexadécimaux (Mode S)
- **ICAO 3-letter codes** (OPRICAO, PERICAO) : 3 lettres majuscules
- **Wake Turbulence** (WKTRC) : L, M, H, J
- **IFPS Flight Plan ID** (IFPLID) : Format alphanumérique
- **Procedures** (SID, STAR, ARRPROC, DEPPROC) : Format alphanumérique avec tirets/points

### Structures composées complexes

Les structures composées suivantes sont également validées avec leur structure interne :

- **ROUTE** : Validation de la structure de route (longueur max 2000 caractères)
- **RTEPTS** (Route Points) : Validation des points de route avec sous-champs (PT, PTID, LAT, LON, FL, ETO, ATOT, SPEED, etc.)
  - Au moins un identifiant de point requis (PT, PTID, ou LAT/LON)
  - LAT et LON doivent être présents ensemble
  - Validation des sous-champs selon leurs types
- **VEC** (Vector) : Validation des éléments de vecteur (TRACKANGLE, GROUNDSPEED, ALT)
  - Au moins un élément requis
  - Validation des valeurs selon leurs types
- **ADDR** (Address) : Validation des adresses avec sous-champs (ADDR, FAC)
  - ADDR: max 8 caractères alphanumériques
  - FAC: max 20 caractères
- **REFDATA** (Reference Data) : Validation des données de référence (IFPLID, ORIGIN, FAC, NETWORKTYPE)
  - Au moins un élément requis
  - Validation des valeurs selon leurs types

La validation est automatiquement effectuée lors de l'appel à `message.validate()`.

**Total : 67+ types de champs validés sémantiquement + 5 structures composées complexes**

### Exemple de validation

```rust
use aftn::AdexpParser;

let input = "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
-EOBD 151220
-EOBT 1200
";

let message = AdexpParser::parse_message(input)?;
match message.validate() {
    Ok(()) => println!("Message valide"),
    Err(e) => eprintln!("Erreur de validation: {}", e),
}
```

## Format des messages ADEXP

Un message ADEXP suit cette structure :

```
-ADEXP
-TITLE [TYPE]
-[SECTION]
-FIELD_NAME [VALUE]
...
-END
```

Exemple :
```
-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
```

## Développement progressif

L'implémentation suit une approche progressive :

### AFTN
1. ✅ Structure de base du projet
2. ✅ Grammaire PEST de base
3. ✅ Parsing des catégories de messages
4. ✅ Tests unitaires fréquents
5. ✅ Fuzzing agressif

### ADEXP
1. ✅ Structure de base du parser ADEXP
2. ✅ Grammaire PEST pour ADEXP
3. ✅ Parsing des différents types de messages
4. ✅ Tests unitaires fréquents
5. ✅ Fuzzing agressif

## Contribution

Les tests sont exécutés fréquemment pour éviter que des erreurs ne s'introduisent progressivement. Chaque nouvelle fonctionnalité doit être accompagnée de tests appropriés.

## Licence

[À définir]

