#!/bin/bash
# Script pour télécharger des messages AFTN depuis diverses sources publiques

SAMPLES_DIR="tests/samples/large_dataset"
mkdir -p "$SAMPLES_DIR"

echo "Téléchargement de messages AFTN depuis sources publiques..."

# 1. Aviation Weather (NOAA) - METAR
echo "Téléchargement de METAR depuis Aviation Weather..."
curl -s "https://www.aviationweather.gov/adds/dataserver_current/current/metar.cache.csv" 2>/dev/null | \
    head -1000 | tail -n +2 | while IFS=',' read -r station time lat lon temp dewp wdir wspd vis alt rmk; do
    if [ ! -z "$station" ] && [ ! -z "$time" ]; then
        day=$(echo "$time" | cut -d'T' -f1 | cut -d'-' -f3)
        hour=$(echo "$time" | cut -d'T' -f2 | cut -d':' -f1)
        minute=$(echo "$time" | cut -d'T' -f2 | cut -d':' -f2)
        echo "GG LFPGYYYX LFPOYYYX ${day}${hour}${minute} METAR ${station} ${day}${hour}${minute}Z ${wdir}${wspd}KT 9999 FEW030 ${temp}/${dewp} Q${alt}" >> "$SAMPLES_DIR/metar_from_noaa.txt" 2>/dev/null || true
    fi
done

# 2. Télécharger depuis des dépôts GitHub avec des exemples
echo "Recherche d'exemples sur GitHub..."

# Essayer de trouver des fichiers d'exemples dans des dépôts publics
GITHUB_FILES=(
    "https://raw.githubusercontent.com/pventon/ICAO-ATS-and-OLDI-Message-Parser/main/test/test_data/aftn_messages.txt"
    "https://raw.githubusercontent.com/search?q=AFTN+message+example+extension:txt"
)

for url in "${GITHUB_FILES[@]}"; do
    echo "Tentative: $url"
    curl -sL "$url" 2>/dev/null | head -500 >> "$SAMPLES_DIR/github_samples.txt" 2>/dev/null || echo "Échec: $url"
done

# 3. Générer des variantes réalistes
echo "Génération de variantes réalistes..."

generate_aftn_message() {
    local msg_type=$1
    local num=$2
    
    local priorities=("GG" "DD" "FF" "SS" "KK" "LL")
    local origins=("LFPGYYYX" "LFPOYYYX" "LFPBYYYX" "LFPNYYYX" "LFPYYYX" "EDDFYYYX" "EGLLYYYX")
    local dests=("LFPOYYYX" "LFPBYYYX" "LFPNYYYX" "LFPGYYYX" "EDDFYYYX" "EGLLYYYX")
    
    local priority=${priorities[$RANDOM % ${#priorities[@]}]}
    local origin=${origins[$RANDOM % ${#origins[@]}]}
    local num_dests=$((RANDOM % 3 + 1))
    local dests_str=""
    for ((i=0; i<num_dests; i++)); do
        dests_str+="${dests[$RANDOM % ${#dests[@]}]} "
    done
    
    local day=$(printf "%02d" $((RANDOM % 28 + 1)))
    local hour=$(printf "%02d" $((RANDOM % 24)))
    local minute=$(printf "%02d" $((RANDOM % 60)))
    
    case $msg_type in
        NOTAM)
            echo "${priority} ${origin} ${dests_str}${day}${hour}${minute} NOTAM A${num}/24 LFPG RWY 09/27 CLOSED"
            ;;
        METAR)
            local wind_dir=$(printf "%03d" $((RANDOM % 360)))
            local wind_speed=$(printf "%02d" $((RANDOM % 50 + 10)))
            local temp=$((RANDOM % 30))
            local dew=$((RANDOM % 20))
            local qnh=$((RANDOM % 100 + 1000))
            echo "${priority} ${origin} ${dests_str}${day}${hour}${minute} METAR LFPG ${day}${hour}${minute}Z ${wind_dir}${wind_speed}KT 9999 FEW030 ${temp}/${dew} Q${qnh}"
            ;;
        TAF)
            local end_day=$(printf "%02d" $((RANDOM % 28 + 1)))
            local end_hour=$(printf "%02d" $((RANDOM % 24)))
            echo "${priority} ${origin} ${dests_str}${day}${hour}00 TAF LFPG ${day}${hour}00Z ${day}${hour}/${end_day}${end_hour} 28015KT 9999 FEW030"
            ;;
        FPL)
            local callsign="ABC$((RANDOM % 999))"
            local flight_type=("V" "I" "Y" "Z")
            local ft=${flight_type[$RANDOM % ${#flight_type[@]}]}
            echo "${priority} ${origin} ${dests_str}${day}${hour}00 FPL ${callsign} ${ft} LFPG ${day}${hour}00 LFPB 1800"
            ;;
    esac
}

# Générer 10000 messages de chaque type
echo "Génération de 10000 NOTAM..."
for i in {1..10000}; do
    generate_aftn_message NOTAM $i >> "$SAMPLES_DIR/notam_large_$((i/1000)).txt"
    if [ $((i % 1000)) -eq 0 ]; then
        echo "  Généré $i NOTAM..."
    fi
done

echo "Génération de 10000 METAR..."
for i in {1..10000}; do
    generate_aftn_message METAR $i >> "$SAMPLES_DIR/metar_large_$((i/1000)).txt"
    if [ $((i % 1000)) -eq 0 ]; then
        echo "  Généré $i METAR..."
    fi
done

echo "Génération de 5000 TAF..."
for i in {1..5000}; do
    generate_aftn_message TAF $i >> "$SAMPLES_DIR/taf_large_$((i/1000)).txt"
    if [ $((i % 1000)) -eq 0 ]; then
        echo "  Généré $i TAF..."
    fi
done

echo "Génération de 5000 FPL..."
for i in {1..5000}; do
    generate_aftn_message FPL $i >> "$SAMPLES_DIR/fpl_large_$((i/1000)).txt"
    if [ $((i % 1000)) -eq 0 ]; then
        echo "  Généré $i FPL..."
    fi
done

echo "Téléchargement terminé. Fichiers dans $SAMPLES_DIR"
ls -lh "$SAMPLES_DIR" | head -20

