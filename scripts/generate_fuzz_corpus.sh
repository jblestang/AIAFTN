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

# Corpus NMEA
CORPUS_DIR_NMEA="fuzz/corpus/fuzz_nmea_parser"
mkdir -p "$CORPUS_DIR_NMEA"

# Messages NMEA valides
echo '$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47' > "$CORPUS_DIR_NMEA/valid_gga.txt"
echo '$GPRMC,123519,A,4807.038,N,01131.000,E,022.4,084.4,230394,003.1,W*6A' > "$CORPUS_DIR_NMEA/valid_rmc.txt"
echo '!AIVDM,1,1,,A,13HOI:0P0000VOHLCnHQKwvL05Ip,0*XX' > "$CORPUS_DIR_NMEA/valid_ais.txt"

# Messages NMEA invalides (pour tester la robustesse)
echo 'GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47' > "$CORPUS_DIR_NMEA/invalid_start.txt"
echo '$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,' > "$CORPUS_DIR_NMEA/missing_checksum.txt"
echo '$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*00' > "$CORPUS_DIR_NMEA/invalid_checksum.txt"

# Corpus SBS
CORPUS_DIR_SBS="fuzz/corpus/fuzz_sbs_parser"
mkdir -p "$CORPUS_DIR_SBS"

# Messages SBS valides
echo "MSG,1,145,29315,4CA2E6,27215,2015/02/05,14:53:22.734,2015/02/05,14:53:22.734,BAW1425,,,,,,,,,,,0" > "$CORPUS_DIR_SBS/valid_identification.txt"
echo "MSG,3,145,29315,4CA2E6,27215,2015/02/05,14:53:22.734,2015/02/05,14:53:22.734,,37025,1035.0,295.6,51.4703,-0.4543,,,,,0" > "$CORPUS_DIR_SBS/valid_position.txt"
echo "MSG,4,145,29315,4CA2E6,27215,2015/02/05,14:53:22.734,2015/02/05,14:53:22.734,,,1035.0,295.6,,,-3200,,,,,0" > "$CORPUS_DIR_SBS/valid_velocity.txt"

# Messages SBS invalides (pour tester la robustesse)
echo "INVALID,1,145,29315,4CA2E6,27215,2015/02/05,14:53:22.734,2015/02/05,14:53:22.734,BAW1425,,,,,,,,,,,0" > "$CORPUS_DIR_SBS/invalid_start.txt"
echo "MSG,,145,29315,4CA2E6,27215,2015/02/05,14:53:22.734,2015/02/05,14:53:22.734,BAW1425,,,,,,,,,,,0" > "$CORPUS_DIR_SBS/missing_type.txt"
echo "MSG,99,145,29315,4CA2E6,27215,2015/02/05,14:53:22.734,2015/02/05,14:53:22.734,BAW1425,,,,,,,,,,,0" > "$CORPUS_DIR_SBS/invalid_type.txt"

echo "Corpus AFTN généré dans $CORPUS_DIR_AFTN"
echo "Corpus ADEXP généré dans $CORPUS_DIR_ADEXP"
echo "Corpus NMEA généré dans $CORPUS_DIR_NMEA"
echo "Corpus SBS généré dans $CORPUS_DIR_SBS"

