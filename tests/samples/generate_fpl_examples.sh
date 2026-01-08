#!/bin/bash
# Script pour générer des exemples FPL réalistes basés sur les spécifications ICAO

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FPL_DIR="${SCRIPT_DIR}/aftn_fpl/fpl_generated"
mkdir -p "${FPL_DIR}"

echo "=== Génération d'exemples FPL réalistes ==="

# Fonction pour générer un FPL réaliste
generate_fpl() {
    local num=$1
    local priorities=("GG" "DD" "FF" "SS")
    local origins=("LFPGYYYX" "LFPOYYYX" "LFPBYYYX" "EDDFYYYX" "EGLLYYYX" "KJFKYYYX" "KLAXYYYX")
    local dests=("LFPOYYYX" "LFPBYYYX" "EDDFYYYX" "EGLLYYYX" "KJFKYYYX" "KLAXYYYX" "KMIAYYYX")
    local aircraft_types=("A320" "B738" "A330" "B777" "A350" "B787")
    local flight_rules=("I" "V" "Y" "Z")
    local routes=("DCT" "N0450F350" "N0450F370" "N0450F390" "N0450F410")
    
    local priority=${priorities[$RANDOM % ${#priorities[@]}]}
    local origin=${origins[$RANDOM % ${#origins[@]}]}
    local dest=${dests[$RANDOM % ${#dests[@]}]}
    local callsign=$(printf "ABC%03d" $((RANDOM % 999 + 1)))
    local flight_rule=${flight_rules[$RANDOM % ${#flight_rules[@]}]}
    local aircraft_type=${aircraft_types[$RANDOM % ${#aircraft_types[@]}]}
    
    local day=$(printf "%02d" $((RANDOM % 28 + 1)))
    local hour=$(printf "%02d" $((RANDOM % 24)))
    local minute=$(printf "%02d" $((RANDOM % 60)))
    
    local speed=$(printf "%04d" $((RANDOM % 500 + 400)))
    local flight_level=$(printf "%03d" $((RANDOM % 400 + 100)))
    
    local route_part=${routes[$RANDOM % ${#routes[@]}]}
    
    # Format FPL basique
    echo "${priority} ${origin} ${dest} ${day}${hour}${minute} FPL ${callsign} ${flight_rule} ${aircraft_type} ${day}${hour}${minute} ${origin} ${speed} ${flight_level} ${route_part} ${dest} ${hour}$(printf "%02d" $((RANDOM % 60)))"
}

# Générer 1000 exemples FPL
echo "Génération de 1000 exemples FPL..."
for i in {1..1000}; do
    generate_fpl $i >> "${FPL_DIR}/fpl_examples_1000.txt"
    if [ $((i % 100)) -eq 0 ]; then
        echo "  Généré $i FPL..."
    fi
done

# Générer des FPL avec routes complexes
echo "Génération de FPL avec routes complexes..."
for i in {1..100}; do
    priority="GG"
    origin="LFPGYYYX"
    dest="KJFKYYYX"
    callsign=$(printf "AF%03d" $i)
    day=$(printf "%02d" $((RANDOM % 28 + 1)))
    hour=$(printf "%02d" $((RANDOM % 24)))
    minute=$(printf "%02d" $((RANDOM % 60)))
    
    echo "${priority} ${origin} ${dest} ${day}${hour}${minute} FPL ${callsign} I A330 ${day}${hour}${minute} ${origin} N0450F350 DCT GOW N0450F370 DCT 50N050W N0450F390 DCT ${dest} ${hour}$(printf "%02d" $((RANDOM % 60)))" >> "${FPL_DIR}/fpl_complex_routes.txt"
done

# Générer des FPL avec alternates
echo "Génération de FPL avec aérodromes alternatifs..."
for i in {1..100}; do
    priority="GG"
    origin="LFPGYYYX"
    dest="KJFKYYYX"
    alt1="KBOSYYYX"
    alt2="KIADYYYX"
    callsign=$(printf "DL%03d" $i)
    day=$(printf "%02d" $((RANDOM % 28 + 1)))
    hour=$(printf "%02d" $((RANDOM % 24)))
    minute=$(printf "%02d" $((RANDOM % 60)))
    
    echo "${priority} ${origin} ${dest} ${alt1} ${alt2} ${day}${hour}${minute} FPL ${callsign} I B777 ${day}${hour}${minute} ${origin} N0450F350 DCT ${dest} ${hour}$(printf "%02d" $((RANDOM % 60)))" >> "${FPL_DIR}/fpl_with_alternates.txt"
done

echo ""
echo "=== Résumé ==="
echo "FPL générés dans: ${FPL_DIR}"
if [ -f "${FPL_DIR}/fpl_examples_1000.txt" ]; then
    echo "  - Exemples de base: $(wc -l < "${FPL_DIR}/fpl_examples_1000.txt" | tr -d ' ') lignes"
fi
if [ -f "${FPL_DIR}/fpl_complex_routes.txt" ]; then
    echo "  - Routes complexes: $(wc -l < "${FPL_DIR}/fpl_complex_routes.txt" | tr -d ' ') lignes"
fi
if [ -f "${FPL_DIR}/fpl_with_alternates.txt" ]; then
    echo "  - Avec alternates: $(wc -l < "${FPL_DIR}/fpl_with_alternates.txt" | tr -d ' ') lignes"
fi

echo ""
echo "Génération terminée !"

