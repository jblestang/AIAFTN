.PHONY: test test-all fuzz fuzz-setup corpus clean

# Exécuter tous les tests
test-all:
	cargo test

# Exécuter uniquement les tests unitaires
test:
	cargo test --lib

# Exécuter les tests d'intégration
test-integration:
	cargo test --test integration_tests
	cargo test --test category_tests
	cargo test --test robustness_tests

# Générer le corpus de fuzzing
corpus:
	./scripts/generate_fuzz_corpus.sh

# Configurer le fuzzing (nécessite cargo-fuzz)
fuzz-setup:
	cargo install cargo-fuzz || true
	cd fuzz && cargo fuzz list

# Exécuter le fuzzing AFTN
fuzz: corpus
	cd fuzz && cargo fuzz run fuzz_parser

# Exécuter le fuzzing ADEXP
fuzz-adexp: corpus
	cd fuzz && cargo fuzz run fuzz_adexp_parser

# Exécuter le fuzzing pour les deux
fuzz-all: corpus
	cd fuzz && cargo fuzz run fuzz_parser &
	cd fuzz && cargo fuzz run fuzz_adexp_parser &
	wait

# Nettoyer les artefacts de build
clean:
	cargo clean
	rm -rf fuzz/corpus fuzz/artifacts fuzz/target

