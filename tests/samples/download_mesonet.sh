#!/bin/bash
# Script pour télécharger les données METAR et TAF depuis mesonet.agron.iastate.edu

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MESONET_DIR="${SCRIPT_DIR}/mesonet"
METAR_DIR="${MESONET_DIR}/metar"
TAF_DIR="${MESONET_DIR}/taf"

mkdir -p "${METAR_DIR}"
mkdir -p "${TAF_DIR}"

# Obtenir la date d'aujourd'hui et hier
TODAY=$(date +%Y%m%d)
YESTERDAY=$(date -v-1d +%Y%m%d 2>/dev/null || date -d "1 day ago" +%Y%m%d)
YEAR=$(date +%Y)
MONTH=$(date +%m)
DAY=$(date +%d)
YEAR2=$(date -v-1d +%Y 2>/dev/null || date -d "1 day ago" +%Y)
MONTH2=$(date -v-1d +%m 2>/dev/null || date -d "1 day ago" +%m)
DAY2=$(date -v-1d +%d 2>/dev/null || date -d "1 day ago" +%d)

echo "=== Téléchargement des données METAR et TAF depuis mesonet.agron.iastate.edu ==="
echo "Période: ${YEAR2}-${MONTH2}-${DAY2} à ${YEAR}-${MONTH}-${DAY}"

# Télécharger des données METAR pour plusieurs stations majeures
STATIONS=("KDSM" "KORD" "KJFK" "KLAX" "KATL" "KDEN" "KSEA" "KMIA")

echo ""
echo "Téléchargement des données METAR pour ${#STATIONS[@]} stations..."

METAR_COUNT=0
for STATION in "${STATIONS[@]}"; do
    echo "  Téléchargement METAR pour ${STATION}..."
    METAR_URL="https://mesonet.agron.iastate.edu/cgi-bin/request/asos.py?station=${STATION}&data=metar&year1=${YEAR2}&month1=${MONTH2}&day1=${DAY2}&year2=${YEAR}&month2=${MONTH}&day2=${DAY}&tz=Etc%2FUTC&format=raw&latlon=no&elev=no&missing=empty&trace=empty&direct=no&report_type=1&report_type=2"
    
    curl -s "${METAR_URL}" >> "${METAR_DIR}/metar_raw.txt" 2>/dev/null || true
    sleep 1
done

# Extraire les messages METAR valides
echo "Extraction des messages METAR pour le parsing..."
if [ -f "${METAR_DIR}/metar_raw.txt" ]; then
    # Extraire les lignes qui contiennent METAR ou SPECI ou commencent par un code d'aéroport
    grep -E "^(METAR|SPECI)|^[A-Z]{4} [0-9]{6}Z" "${METAR_DIR}/metar_raw.txt" > "${METAR_DIR}/metar_messages.txt" 2>/dev/null || \
    grep -i "METAR\|SPECI" "${METAR_DIR}/metar_raw.txt" | head -1000 > "${METAR_DIR}/metar_messages.txt" 2>/dev/null || \
    head -1000 "${METAR_DIR}/metar_raw.txt" > "${METAR_DIR}/metar_messages.txt"
    
    METAR_MSG_COUNT=$(wc -l < "${METAR_DIR}/metar_messages.txt" 2>/dev/null | tr -d ' ' || echo "0")
    echo "  ✓ ${METAR_MSG_COUNT} messages METAR extraits"
fi

# Télécharger des données TAF pour plusieurs stations
echo ""
echo "Téléchargement des données TAF pour ${#STATIONS[@]} stations..."

TAF_COUNT=0
for STATION in "${STATIONS[@]}"; do
    echo "  Téléchargement TAF pour ${STATION}..."
    TAF_URL="https://mesonet.agron.iastate.edu/cgi-bin/request/taf.py?station=${STATION}&year1=${YEAR2}&month1=${MONTH2}&day1=${DAY2}&year2=${YEAR}&month2=${MONTH}&day2=${DAY}&tz=Etc%2FUTC&format=raw&latlon=no&elev=no&missing=empty&trace=empty&direct=no&report_type=1&report_type=2"
    
    curl -s "${TAF_URL}" >> "${TAF_DIR}/taf_raw.txt" 2>/dev/null || true
    sleep 1
done

# Extraire les messages TAF valides
echo "Extraction des messages TAF pour le parsing..."
if [ -f "${TAF_DIR}/taf_raw.txt" ]; then
    # Extraire les lignes qui commencent par TAF
    grep -E "^TAF|^[A-Z]{4} [0-9]{6}Z" "${TAF_DIR}/taf_raw.txt" > "${TAF_DIR}/taf_messages.txt" 2>/dev/null || \
    grep -i "TAF" "${TAF_DIR}/taf_raw.txt" | head -1000 > "${TAF_DIR}/taf_messages.txt" 2>/dev/null || \
    head -1000 "${TAF_DIR}/taf_raw.txt" > "${TAF_DIR}/taf_messages.txt"
    
    TAF_MSG_COUNT=$(wc -l < "${TAF_DIR}/taf_messages.txt" 2>/dev/null | tr -d ' ' || echo "0")
    echo "  ✓ ${TAF_MSG_COUNT} messages TAF extraits"
fi

echo ""
echo "=== Résumé ==="
echo "Données METAR: ${METAR_DIR}"
if [ -f "${METAR_DIR}/metar_messages.txt" ]; then
    echo "  - Messages METAR: $(wc -l < "${METAR_DIR}/metar_messages.txt" | tr -d ' ') lignes"
fi
echo "Données TAF: ${TAF_DIR}"
if [ -f "${TAF_DIR}/taf_messages.txt" ]; then
    echo "  - Messages TAF: $(wc -l < "${TAF_DIR}/taf_messages.txt" | tr -d ' ') lignes"
fi
echo ""
echo "Téléchargement terminé !"

