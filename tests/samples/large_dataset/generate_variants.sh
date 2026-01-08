#!/bin/bash
# Génère des variantes de messages AFTN pour tester le parser

generate_notam() {
    local num=$1
    local priorities=("GG" "DD" "FF" "SS")
    local origins=("LFPGYYYX" "LFPOYYYX" "LFPBYYYX" "LFPNYYYX")
    local dests=("LFPOYYYX" "LFPBYYYX" "LFPNYYYX" "LFPGYYYX")
    local day=$(printf "%02d" $((RANDOM % 28 + 1)))
    local hour=$(printf "%02d" $((RANDOM % 24)))
    local minute=$(printf "%02d" $((RANDOM % 60)))
    
    local priority=${priorities[$RANDOM % ${#priorities[@]}]}
    local origin=${origins[$RANDOM % ${#origins[@]}]}
    local dest=${dests[$RANDOM % ${#dests[@]}]}
    
    echo "${priority} ${origin} ${dest} ${day}${hour}${minute} NOTAM A${num}/24 LFPG RWY 09/27 CLOSED DUE TO MAINTENANCE"
}

generate_metar() {
    local num=$1
    local day=$(printf "%02d" $((RANDOM % 28 + 1)))
    local hour=$(printf "%02d" $((RANDOM % 24)))
    local minute=$(printf "%02d" $((RANDOM % 60)))
    local wind_dir=$((RANDOM % 360))
    local wind_speed=$((RANDOM % 50 + 10))
    local temp=$((RANDOM % 30))
    local dew=$((RANDOM % 20))
    local qnh=$((RANDOM % 100 + 1000))
    
    echo "GG LFPGYYYX LFPOYYYX ${day}${hour}${minute} METAR LFPG ${day}${hour}${minute}Z ${wind_dir}${wind_speed}KT 9999 FEW030 ${temp}/${dew} Q${qnh}"
}

# Générer 1000 exemples de chaque type
echo "Génération de 1000 NOTAM..."
for i in {1..1000}; do
    generate_notam $i >> "$SAMPLES_DIR/notam_batch_$((i/100)).txt"
done

echo "Génération de 1000 METAR..."
for i in {1..1000}; do
    generate_metar $i >> "$SAMPLES_DIR/metar_batch_$((i/100)).txt"
done

echo "Génération terminée"
