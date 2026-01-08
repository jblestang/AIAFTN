#!/bin/bash
# Script pour télécharger des bases de données AFTN/FPL depuis diverses sources en ligne

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
AFTN_DIR="${SCRIPT_DIR}/aftn_fpl"
METAR_DIR="${AFTN_DIR}/metar"
TAF_DIR="${AFTN_DIR}/taf"
FPL_DIR="${AFTN_DIR}/fpl"
GITHUB_DIR="${AFTN_DIR}/github"

mkdir -p "${METAR_DIR}"
mkdir -p "${TAF_DIR}"
mkdir -p "${FPL_DIR}"
mkdir -p "${GITHUB_DIR}"

echo "=== Recherche et téléchargement de bases de données AFTN/FPL ==="
echo ""

# 1. Aviation Weather (NOAA) - METAR et TAF
echo "1. Téléchargement depuis Aviation Weather (NOAA)..."
echo "   - METAR..."

# Télécharger les METAR récents
curl -s "https://www.aviationweather.gov/adds/dataserver_current/current/metar.cache.csv" \
    -o "${METAR_DIR}/noaa_metar.csv" 2>/dev/null || {
    echo "     ⚠ Impossible de télécharger METAR depuis NOAA"
}

if [ -f "${METAR_DIR}/noaa_metar.csv" ]; then
    METAR_COUNT=$(wc -l < "${METAR_DIR}/noaa_metar.csv" | tr -d ' ')
    echo "     ✓ ${METAR_COUNT} lignes téléchargées"
    
    # Extraire les messages METAR bruts (colonne raw_text est généralement la dernière)
    tail -n +2 "${METAR_DIR}/noaa_metar.csv" | awk -F',' '{print $NF}' | grep -E "^(METAR|SPECI)" > "${METAR_DIR}/noaa_metar_raw.txt" 2>/dev/null || true
    
    if [ -f "${METAR_DIR}/noaa_metar_raw.txt" ]; then
        RAW_COUNT=$(wc -l < "${METAR_DIR}/noaa_metar_raw.txt" | tr -d ' ')
        echo "     ✓ ${RAW_COUNT} messages METAR bruts extraits"
    fi
fi

echo "   - TAF..."
curl -s "https://www.aviationweather.gov/adds/dataserver_current/current/taf.cache.csv" \
    -o "${TAF_DIR}/noaa_taf.csv" 2>/dev/null || {
    echo "     ⚠ Impossible de télécharger TAF depuis NOAA"
}

if [ -f "${TAF_DIR}/noaa_taf.csv" ]; then
    TAF_COUNT=$(wc -l < "${TAF_DIR}/noaa_taf.csv" | tr -d ' ')
    echo "     ✓ ${TAF_COUNT} lignes téléchargées"
    
    # Extraire les messages TAF bruts (colonne raw_text est généralement la dernière)
    tail -n +2 "${TAF_DIR}/noaa_taf.csv" | awk -F',' '{print $NF}' | grep -E "^TAF" > "${TAF_DIR}/noaa_taf_raw.txt" 2>/dev/null || true
    
    if [ -f "${TAF_DIR}/noaa_taf_raw.txt" ]; then
        RAW_COUNT=$(wc -l < "${TAF_DIR}/noaa_taf_raw.txt" | tr -d ' ')
        echo "     ✓ ${RAW_COUNT} messages TAF bruts extraits"
    fi
fi

# 2. Dépôts GitHub avec des exemples AFTN/FPL
echo ""
echo "2. Recherche dans les dépôts GitHub..."

GITHUB_REPOS=(
    "pventon/ICAO-ATS-and-OLDI-Message-Parser"
    "aviationweather/aviationweather.github.io"
    "mikenye/aviationweather"
)

for repo in "${GITHUB_REPOS[@]}"; do
    echo "   - Recherche dans ${repo}..."
    
    # Chercher des fichiers contenant AFTN, FPL, ou des exemples
    FILES=(
        "test/test_data/aftn_messages.txt"
        "test/data/aftn.txt"
        "examples/aftn.txt"
        "samples/aftn.txt"
        "data/aftn.txt"
        "test/test_data/fpl_messages.txt"
        "examples/fpl.txt"
        "tests/test_data/aftn_messages.txt"
        "tests/data/aftn.txt"
    )
    
    for file_path in "${FILES[@]}"; do
        # Essayer main, master, et la branche par défaut
        for branch in "main" "master"; do
            URL="https://raw.githubusercontent.com/${repo}/${branch}/${file_path}"
            OUTPUT_FILE="${GITHUB_DIR}/$(echo ${repo} | tr '/' '_')_$(basename ${file_path})"
            
            if curl -sLf "${URL}" -o "${OUTPUT_FILE}" 2>/dev/null; then
                if [ -s "${OUTPUT_FILE}" ] && ! grep -q "404: Not Found" "${OUTPUT_FILE}" 2>/dev/null; then
                    LINE_COUNT=$(wc -l < "${OUTPUT_FILE}" | tr -d ' ')
                    echo "     ✓ Téléchargé: ${file_path} (${LINE_COUNT} lignes)"
                    break 2
                fi
            fi
        done
    done
    
    sleep 1  # Respecter les limites de taux
done

# 3. OpenSky Network - Données de vol (peut contenir des FPL)
echo ""
echo "3. Recherche dans OpenSky Network..."
echo "   Note: OpenSky fournit des données de vol mais pas directement des FPL AFTN"
echo "   Les données sont disponibles via leur API REST"

# 4. FAA - Exemples de messages (si disponibles)
echo ""
echo "4. Recherche d'exemples FAA..."
FAA_URLS=(
    "https://www.faa.gov/about/office_org/headquarters_offices/ato/service_units/air_traffic_services/flight_plan_filing/guidance/reference_guide/message_ack_rej/message_examples"
)

for url in "${FAA_URLS[@]}"; do
    echo "   - Tentative: ${url}"
    curl -sL "${url}" -o "${FPL_DIR}/faa_examples.html" 2>/dev/null || true
    if [ -f "${FPL_DIR}/faa_examples.html" ] && [ -s "${FPL_DIR}/faa_examples.html" ]; then
        echo "     ✓ Page téléchargée (peut contenir des exemples)"
    fi
done

# 5. Aviation Edge - API publique (si disponible)
echo ""
echo "5. Recherche d'autres sources publiques..."

# 6. Créer un fichier consolidé pour les tests
echo ""
echo "6. Consolidation des données pour les tests..."

CONSOLIDATED_METAR="${AFTN_DIR}/consolidated_metar.txt"
CONSOLIDATED_TAF="${AFTN_DIR}/consolidated_taf.txt"
CONSOLIDATED_FPL="${AFTN_DIR}/consolidated_fpl.txt"
CONSOLIDATED_AFTN="${AFTN_DIR}/consolidated_aftn.txt"

# Consolider METAR
if [ -f "${METAR_DIR}/noaa_metar_raw.txt" ]; then
    head -1000 "${METAR_DIR}/noaa_metar_raw.txt" > "${CONSOLIDATED_METAR}" 2>/dev/null || true
    echo "   ✓ ${CONSOLIDATED_METAR} créé"
fi

# Consolider TAF
if [ -f "${TAF_DIR}/noaa_taf_raw.txt" ]; then
    head -1000 "${TAF_DIR}/noaa_taf_raw.txt" > "${CONSOLIDATED_TAF}" 2>/dev/null || true
    echo "   ✓ ${CONSOLIDATED_TAF} créé"
fi

# Consolider FPL depuis GitHub
find "${GITHUB_DIR}" -name "*.txt" -type f -exec cat {} \; > "${CONSOLIDATED_FPL}" 2>/dev/null || true
if [ -f "${CONSOLIDATED_FPL}" ] && [ -s "${CONSOLIDATED_FPL}" ]; then
    FPL_COUNT=$(wc -l < "${CONSOLIDATED_FPL}" | tr -d ' ')
    echo "   ✓ ${CONSOLIDATED_FPL} créé (${FPL_COUNT} lignes)"
fi

# Consolider tous les messages AFTN
cat "${CONSOLIDATED_METAR}" "${CONSOLIDATED_TAF}" "${CONSOLIDATED_FPL}" > "${CONSOLIDATED_AFTN}" 2>/dev/null || true
if [ -f "${CONSOLIDATED_AFTN}" ] && [ -s "${CONSOLIDATED_AFTN}" ]; then
    AFTN_COUNT=$(wc -l < "${CONSOLIDATED_AFTN}" | tr -d ' ')
    echo "   ✓ ${CONSOLIDATED_AFTN} créé (${AFTN_COUNT} lignes)"
fi

echo ""
echo "=== Résumé ==="
echo "Données téléchargées dans: ${AFTN_DIR}"
echo ""
echo "METAR:"
if [ -f "${METAR_DIR}/noaa_metar.csv" ]; then
    echo "  - CSV: $(wc -l < "${METAR_DIR}/noaa_metar.csv" | tr -d ' ') lignes"
fi
if [ -f "${METAR_DIR}/noaa_metar_raw.txt" ]; then
    echo "  - Raw: $(wc -l < "${METAR_DIR}/noaa_metar_raw.txt" | tr -d ' ') messages"
fi

echo ""
echo "TAF:"
if [ -f "${TAF_DIR}/noaa_taf.csv" ]; then
    echo "  - CSV: $(wc -l < "${TAF_DIR}/noaa_taf.csv" | tr -d ' ') lignes"
fi
if [ -f "${TAF_DIR}/noaa_taf_raw.txt" ]; then
    echo "  - Raw: $(wc -l < "${TAF_DIR}/noaa_taf_raw.txt" | tr -d ' ') messages"
fi

echo ""
echo "FPL/AFTN depuis GitHub:"
find "${GITHUB_DIR}" -name "*.txt" -type f | wc -l | xargs echo "  - Fichiers:"

echo ""
echo "Fichiers consolidés:"
if [ -f "${CONSOLIDATED_METAR}" ]; then
    echo "  - METAR: $(wc -l < "${CONSOLIDATED_METAR}" | tr -d ' ') lignes"
fi
if [ -f "${CONSOLIDATED_TAF}" ]; then
    echo "  - TAF: $(wc -l < "${CONSOLIDATED_TAF}" | tr -d ' ') lignes"
fi
if [ -f "${CONSOLIDATED_FPL}" ]; then
    echo "  - FPL: $(wc -l < "${CONSOLIDATED_FPL}" | tr -d ' ') lignes"
fi
if [ -f "${CONSOLIDATED_AFTN}" ]; then
    echo "  - AFTN (total): $(wc -l < "${CONSOLIDATED_AFTN}" | tr -d ' ') lignes"
fi

echo ""
echo "Téléchargement terminé !"

