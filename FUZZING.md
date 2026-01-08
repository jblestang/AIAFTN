# Guide de Fuzzing pour AFTN Parser

Ce document décrit comment utiliser le fuzzing pour tester la robustesse du parseur AFTN.

## Installation

### Prérequis

1. Installer `cargo-fuzz` :
```bash
cargo install cargo-fuzz
```

2. Installer les outils de fuzzing (AFL++ recommandé) :
```bash
# Sur macOS avec Homebrew
brew install afl-fuzz

# Sur Linux (Ubuntu/Debian)
sudo apt-get install afl++
```

## Configuration

Le fuzzing est configuré dans le dossier `fuzz/` avec :
- `fuzz/Cargo.toml` : Configuration du projet de fuzzing
- `fuzz/fuzz_targets/fuzz_parser.rs` : Cible de fuzzing principale

## Utilisation

### 1. Générer le corpus initial

```bash
make corpus
```

ou

```bash
./scripts/generate_fuzz_corpus.sh
```

Cela crée un ensemble de messages AFTN valides et invalides dans `fuzz/corpus/fuzz_parser/`.

### 2. Exécuter le fuzzing

#### Avec cargo-fuzz (recommandé)

```bash
cd fuzz
cargo fuzz run fuzz_parser
```

#### Avec AFL++ (plus agressif)

```bash
cd fuzz
cargo fuzz build fuzz_parser
afl-fuzz -i corpus/fuzz_parser -o artifacts fuzz/target/x86_64-unknown-linux-gnu/release/fuzz_parser
```

### 3. Analyser les résultats

Les crashes et timeouts sont sauvegardés dans `fuzz/artifacts/`. Analysez-les pour identifier les problèmes potentiels.

## Stratégie de fuzzing agressive

### Corpus diversifié

Le corpus initial inclut :
- Messages valides de toutes les catégories
- Messages avec séquences
- Messages avec plusieurs destinations
- Messages potentiellement invalides (pour tester la robustesse)

### Ajout de cas de test personnalisés

Ajoutez vos propres cas de test dans `fuzz/corpus/fuzz_parser/` :
- Messages réels capturés
- Messages malformés connus
- Cas limites identifiés

### Fuzzing continu

Pour un fuzzing continu en arrière-plan :

```bash
# Utiliser tmux ou screen
tmux new -s fuzzing
cd fuzz
cargo fuzz run fuzz_parser
# Détacher avec Ctrl+B puis D
```

## Interprétation des résultats

### Crashes

Si le fuzzer trouve un crash :
1. Reproduire le crash avec le fichier généré
2. Analyser la stack trace
3. Corriger le bug
4. Ajouter un test de régression
5. Relancer le fuzzing

### Timeouts

Les timeouts peuvent indiquer :
- Boucles infinies dans le parser
- Regex trop complexes
- Problèmes de performance

### Coverage

Suivez la couverture de code pour identifier les zones non testées :
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

## Bonnes pratiques

1. **Fuzzer régulièrement** : Intégrez le fuzzing dans votre CI/CD
2. **Diversifier le corpus** : Ajoutez régulièrement de nouveaux cas
3. **Analyser les résultats** : Ne laissez pas les crashes s'accumuler
4. **Documenter les bugs** : Gardez une trace des problèmes trouvés
5. **Tester les corrections** : Vérifiez que les corrections fonctionnent

## Intégration CI/CD

Exemple pour GitHub Actions :

```yaml
name: Fuzzing
on: [push, pull_request]
jobs:
  fuzz:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
      - run: cargo install cargo-fuzz
      - run: make corpus
      - run: cd fuzz && timeout 300 cargo fuzz run fuzz_parser || true
```

## Ressources

- [cargo-fuzz documentation](https://github.com/rust-fuzz/cargo-fuzz)
- [AFL++ documentation](https://github.com/AFLplusplus/AFLplusplus)
- [Rust Fuzz Book](https://rust-fuzz.github.io/book/)

