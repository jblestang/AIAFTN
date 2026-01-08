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
- **CHGDEP** : Changed Departure (réservé par la France)
- **Generic** : Messages génériques non catégorisés

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

