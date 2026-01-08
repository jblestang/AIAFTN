#!/bin/bash
# Script pour générer un corpus de base pour le fuzzing

# Corpus AFTN
CORPUS_DIR_AFTN="fuzz/corpus/fuzz_parser"
mkdir -p "$CORPUS_DIR_AFTN"

# Messages AFTN valides
echo "GG LFPGYYYX LFPOYYYX 151230 NOTAM A1234/24 LFPG RWY 09/27 CLOSED" > "$CORPUS_DIR_AFTN/valid_notam.txt"
echo "DD LFPGYYYX LFPOYYYX 201530 METAR LFPG 201530Z 28015KT 9999 FEW030 12/08 Q1013" > "$CORPUS_DIR_AFTN/valid_metar.txt"
echo "FF LFPGYYYX LFPOYYYX 151200 TAF LFPG 151200Z 1512/1612 28015KT 9999 FEW030" > "$CORPUS_DIR_AFTN/valid_taf.txt"
echo "SS LFPGYYYX LFPOYYYX 151200 FPL ABC123 V LFPG 151200 LFPB 1800" > "$CORPUS_DIR_AFTN/valid_fpl.txt"

# Messages AFTN avec séquence
echo "GG LFPGYYYX LFPOYYYX 151230 TEST MESSAGE /SEQ 001" > "$CORPUS_DIR_AFTN/with_sequence.txt"

# Messages AFTN avec plusieurs destinations
echo "DD LFPGYYYX LFPOYYYX LFPBYYYX LFPNYYYX 201530 METAR LFPG 201530Z" > "$CORPUS_DIR_AFTN/multiple_dest.txt"

# Messages AFTN potentiellement invalides (pour tester la robustesse)
echo "XX LFPGYYYX LFPOYYYX 151230 TEST" > "$CORPUS_DIR_AFTN/invalid_priority.txt"
echo "GG SHORT LFPOYYYX 151230 TEST" > "$CORPUS_DIR_AFTN/short_address.txt"
echo "GG LFPGYYYX LFPOYYYX 321230 TEST" > "$CORPUS_DIR_AFTN/invalid_date.txt"

# Corpus ADEXP
CORPUS_DIR_ADEXP="fuzz/corpus/fuzz_adexp_parser"
mkdir -p "$CORPUS_DIR_ADEXP"

# Messages ADEXP valides
echo "-ADEXP
-TITLE FPL
-ARCID ABC123
-ADEP LFPG
-ADES LFPB
" > "$CORPUS_DIR_ADEXP/valid_fpl.txt"

echo "-ADEXP
-TITLE CHG
-ARCID ABC123
-ADEP LFPG
" > "$CORPUS_DIR_ADEXP/valid_chg.txt"

echo "-ADEXP
-TITLE CNL
-ARCID ABC123
" > "$CORPUS_DIR_ADEXP/valid_cnl.txt"

echo "-ADEXP
-TITLE DLA
-ARCID ABC123
-ADEP LFPG
" > "$CORPUS_DIR_ADEXP/valid_dla.txt"

# Messages ADEXP avec sections
echo "-ADEXP
-TITLE FPL
-ARCID ABC123
-ROUTE
-ADEP LFPG
-ADES LFPB
" > "$CORPUS_DIR_ADEXP/with_sections.txt"

# Messages ADEXP avec marqueur de fin
echo "-ADEXP
-TITLE FPL
-ARCID ABC123
-END
" > "$CORPUS_DIR_ADEXP/with_end_marker.txt"

# Messages ADEXP potentiellement invalides
echo "-ADEXP
" > "$CORPUS_DIR_ADEXP/empty.txt"

echo "-ADEXP
-ARCID ABC123
" > "$CORPUS_DIR_ADEXP/missing_title.txt"

echo "-ADEXP
-TITLE
-ARCID ABC123
" > "$CORPUS_DIR_ADEXP/empty_title.txt"

echo "Corpus AFTN généré dans $CORPUS_DIR_AFTN"
echo "Corpus ADEXP généré dans $CORPUS_DIR_ADEXP"

