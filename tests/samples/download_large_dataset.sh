#!/bin/bash
# Script pour télécharger un large échantillon de messages AFTN depuis diverses sources

SAMPLES_DIR="tests/samples/large_dataset"
mkdir -p "$SAMPLES_DIR"

echo "Recherche et téléchargement d'exemples de messages AFTN..."

# Essayer de télécharger depuis des dépôts GitHub publics
echo "Tentative de téléchargement depuis GitHub..."

# Chercher des dépôts avec des exemples AFTN
GITHUB_REPOS=(
    "pventon/ICAO-ATS-and-OLDI-Message-Parser"
    "aviation-data/aftn-messages"
    "openaviation/aftn-samples"
)

for repo in "${GITHUB_REPOS[@]}"; do
    echo "Tentative: $repo"
    # Essayer de trouver des fichiers de test ou d'exemples
    curl -sL "https://api.github.com/repos/$repo/contents" 2>/dev/null | \
        grep -i "\.txt\|\.aftn\|test\|sample" | head -5 || true
done

# Télécharger depuis des sources publiques d'aviation
echo "Tentative de téléchargement depuis sources publiques..."

# Essayer de télécharger des METAR depuis des sources publiques
METAR_SOURCES=(
    "https://www.aviationweather.gov/adds/dataserver_current/current/metar.cache.csv"
    "https://tgftp.nws.noaa.gov/data/observations/metar/stations/"
)

# Créer un script pour générer des variantes de messages
cat > "$SAMPLES_DIR/generate_variants.sh" << 'GENEOF'
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
GENEOF

chmod +x "$SAMPLES_DIR/generate_variants.sh"

echo "Script de génération créé dans $SAMPLES_DIR/generate_variants.sh"
echo "Exécutez-le pour générer des milliers d'exemples de messages"

