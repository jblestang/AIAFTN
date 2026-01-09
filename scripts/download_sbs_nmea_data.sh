#!/bin/bash
# Script pour télécharger/générer des données SBS et NMEA pour les tests

set -e

SAMPLES_DIR="tests/samples"
SBS_DIR="${SAMPLES_DIR}/sbs"
NMEA_DIR="${SAMPLES_DIR}/nmea"

mkdir -p "$SBS_DIR"
mkdir -p "$NMEA_DIR"

echo "=== Téléchargement/Génération de données SBS et NMEA ==="

# 1. Générer des données SBS synthétiques (format BaseStation)
echo "Génération de données SBS synthétiques..."

# Générer 15000 messages SBS variés
python3 << 'PYTHON_SCRIPT'
import random
import datetime

def generate_sbs_message(msg_type, hex_ident, callsign, lat, lon, alt, speed, track):
    """Génère un message SBS au format BaseStation"""
    now = datetime.datetime.now()
    date_str = now.strftime("%Y/%m/%d")
    time_str = now.strftime("%H:%M:%S.%f")[:-3]
    
    # Format: MSG,type,transmission_type,session_id,aircraft_id,hex_ident,flight_id,date_gen,time_gen,date_log,time_log,callsign,altitude,speed,track,lat,lon,vr,squawk,alert,emergency,spi,is_on_ground
    if msg_type == 1:  # Identification
        return f"MSG,1,145,{random.randint(10000,99999)},{hex_ident},{random.randint(10000,99999)},{date_str},{time_str},{date_str},{time_str},{callsign},,,,,,,,,,,,0"
    elif msg_type == 2:  # Surface Position
        return f"MSG,2,145,{random.randint(10000,99999)},{hex_ident},{random.randint(10000,99999)},{date_str},{time_str},{date_str},{time_str},,,0.0,0.0,{lat:.6f},{lon:.6f},,,,,,1"
    elif msg_type == 3:  # Airborne Position
        return f"MSG,3,145,{random.randint(10000,99999)},{hex_ident},{random.randint(10000,99999)},{date_str},{time_str},{date_str},{time_str},,{int(alt)},{speed:.1f},{track:.1f},{lat:.6f},{lon:.6f},,,,,,0"
    elif msg_type == 4:  # Airborne Velocity
        vr = random.randint(-5000, 5000)
        return f"MSG,4,145,{random.randint(10000,99999)},{hex_ident},{random.randint(10000,99999)},{date_str},{time_str},{date_str},{time_str},,,{speed:.1f},{track:.1f},,,{vr},,,,,,0"
    elif msg_type == 6:  # Surveillance ID
        squawk = f"{random.randint(0,7)}{random.randint(0,7)}{random.randint(0,7)}{random.randint(0,7)}"
        return f"MSG,6,145,{random.randint(10000,99999)},{hex_ident},{random.randint(10000,99999)},{date_str},{time_str},{date_str},{time_str},,,,,,,,,,{squawk},,,0"
    else:
        return f"MSG,{msg_type},145,{random.randint(10000,99999)},{hex_ident},{random.randint(10000,99999)},{date_str},{time_str},{date_str},{time_str},,,,,,,,,,,,,,0"

# Générer des hex_ident ICAO valides (6 caractères hex)
def random_hex_ident():
    return ''.join(random.choices('0123456789ABCDEF', k=6))

# Générer des callsigns valides
callsigns = [
    "BAW1425", "AFR1234", "UAE567", "DLH890", "KLM123", "EZY456", "RYR789",
    "SWA123", "UAL456", "AAL789", "JAL123", "QTR456", "SIA789", "THA123"
]

with open("tests/samples/sbs/sbs_large_dataset.txt", "w") as f:
    for i in range(15000):
        hex_ident = random_hex_ident()
        callsign = random.choice(callsigns) if random.random() > 0.3 else ""
        msg_type = random.choice([1, 2, 3, 4, 6])
        
        if msg_type in [3, 4]:
            lat = random.uniform(-90, 90)
            lon = random.uniform(-180, 180)
            alt = random.uniform(0, 50000)
            speed = random.uniform(0, 1000)
            track = random.uniform(0, 359.9)
        elif msg_type == 2:
            lat = random.uniform(-90, 90)
            lon = random.uniform(-180, 180)
            alt = 0
            speed = 0
            track = 0
        else:
            lat = 0
            lon = 0
            alt = 0
            speed = 0
            track = 0
        
        msg = generate_sbs_message(msg_type, hex_ident, callsign, lat, lon, alt, speed, track)
        f.write(msg + "\n")

print("✓ Généré 15000 messages SBS dans tests/samples/sbs/sbs_large_dataset.txt")
PYTHON_SCRIPT

# 2. Générer des données NMEA synthétiques
echo "Génération de données NMEA synthétiques..."

python3 << 'PYTHON_SCRIPT'
import random
import datetime

def calculate_nmea_checksum(data):
    """Calcule le checksum NMEA (XOR de tous les octets)"""
    checksum = 0
    for byte in data.encode('ascii'):
        checksum ^= byte
    return f"{checksum:02X}"

def generate_gpgga():
    """Génère un message GPGGA (Global Positioning System Fix Data)"""
    time = datetime.datetime.now().strftime("%H%M%S.%f")[:-4]
    lat_deg = random.randint(0, 90)
    lat_min = random.uniform(0, 59.999)
    lat_dir = random.choice(['N', 'S'])
    lon_deg = random.randint(0, 180)
    lon_min = random.uniform(0, 59.999)
    lon_dir = random.choice(['E', 'W'])
    quality = random.choice([0, 1, 2, 3, 4, 5, 6, 7, 8])
    num_sats = random.randint(0, 12)
    hdop = random.uniform(0.5, 3.0)
    altitude = random.uniform(-100, 10000)
    geoid_sep = random.uniform(-50, 50)
    
    msg = f"GPGGA,{time},{lat_deg:02d}{lat_min:05.2f},{lat_dir},{lon_deg:03d}{lon_min:05.2f},{lon_dir},{quality},{num_sats:02d},{hdop:.1f},{altitude:.1f},M,{geoid_sep:.1f},M,,"
    checksum = calculate_nmea_checksum(msg)
    return f"${msg}*{checksum}"

def generate_gprmc():
    """Génère un message GPRMC (Recommended Minimum Specific GPS/Transit Data)"""
    time = datetime.datetime.now().strftime("%H%M%S.%f")[:-4]
    date = datetime.datetime.now().strftime("%d%m%y")
    lat_deg = random.randint(0, 90)
    lat_min = random.uniform(0, 59.999)
    lat_dir = random.choice(['N', 'S'])
    lon_deg = random.randint(0, 180)
    lon_min = random.uniform(0, 59.999)
    lon_dir = random.choice(['E', 'W'])
    status = random.choice(['A', 'V'])
    speed = random.uniform(0, 50)
    track = random.uniform(0, 359.9)
    mag_var = random.uniform(0, 180)
    var_dir = random.choice(['E', 'W'])
    
    msg = f"GPRMC,{time},{status},{lat_deg:02d}{lat_min:05.2f},{lat_dir},{lon_deg:03d}{lon_min:05.2f},{lon_dir},{speed:.1f},{track:.1f},{date},{mag_var:.1f},{var_dir}"
    checksum = calculate_nmea_checksum(msg)
    return f"${msg}*{checksum}"

def generate_gpgsa():
    """Génère un message GPGSA (GPS DOP and active satellites)"""
    mode = random.choice(['A', 'M'])
    fix_type = random.choice(['1', '2', '3'])
    prns = []
    for _ in range(12):
        if random.random() > 0.2:
            prns.append(f"{random.randint(1,32):02d}")
        else:
            prns.append("")
    pdop = random.uniform(0.5, 5.0)
    hdop = random.uniform(0.5, 3.0)
    vdop = random.uniform(0.5, 3.0)
    
    msg = f"GPGSA,{mode},{fix_type},{','.join(prns)},{pdop:.1f},{hdop:.1f},{vdop:.1f}"
    checksum = calculate_nmea_checksum(msg)
    return f"${msg}*{checksum}"

def generate_gpvtg():
    """Génère un message GPVTG (Track made good and Ground speed)"""
    track_true = random.uniform(0, 359.9)
    track_mag = random.uniform(0, 359.9)
    speed_knots = random.uniform(0, 50)
    speed_kmh = speed_knots * 1.852
    mode = random.choice(['A', 'D', 'E', 'M', 'N', 'S'])
    
    msg = f"GPVTG,{track_true:.1f},T,{track_mag:.1f},M,{speed_knots:.1f},N,{speed_kmh:.1f},K,{mode}"
    checksum = calculate_nmea_checksum(msg)
    return f"${msg}*{checksum}"

def generate_ais_message():
    """Génère un message AIS AIVDM (simplifié)"""
    # Message AIS Type 1 (Position Report)
    # Format simplifié pour la génération
    msg_id = "1"
    repeat = "0"
    mmsi = f"{random.randint(100000000, 999999999)}"
    nav_status = random.randint(0, 15)
    rot = random.randint(-127, 127)
    sog = random.randint(0, 1023)
    pos_acc = random.choice([0, 1])
    lon = random.randint(-18000000, 18000000)
    lat = random.randint(-9000000, 9000000)
    cog = random.randint(0, 3599)
    true_heading = random.randint(0, 511)
    timestamp = random.randint(0, 59)
    
    # Encodage simplifié (en réalité c'est du 6-bit ASCII)
    payload = f"{msg_id}{repeat}{mmsi}{nav_status:04b}{rot:08b}{sog:010b}{pos_acc}{lon:018b}{lat:017b}{cog:012b}{true_heading:09b}{timestamp:06b}"
    # Pour simplifier, on génère un payload encodé en base64-like
    payload_encoded = "A" * 20  # Placeholder simplifié
    
    msg = f"AIVDM,1,1,,A,{payload_encoded},0"
    checksum = calculate_nmea_checksum(msg)
    return f"!{msg}*{checksum}"

# Générer 12000 messages NMEA variés
with open("tests/samples/nmea/nmea_large_dataset.txt", "w") as f:
    for i in range(12000):
        msg_type = random.choice(['gpgga', 'gprmc', 'gpgsa', 'gpvtg', 'ais'])
        
        if msg_type == 'gpgga':
            msg = generate_gpgga()
        elif msg_type == 'gprmc':
            msg = generate_gprmc()
        elif msg_type == 'gpgsa':
            msg = generate_gpgsa()
        elif msg_type == 'gpvtg':
            msg = generate_gpvtg()
        else:  # ais
            msg = generate_ais_message()
        
        f.write(msg + "\n")

print("✓ Généré 12000 messages NMEA dans tests/samples/nmea/nmea_large_dataset.txt")
PYTHON_SCRIPT

# 3. Télécharger des données réelles depuis OpenSky Network (si disponible)
echo "Tentative de téléchargement de données SBS depuis OpenSky Network..."

# Note: OpenSky Network nécessite une API key pour les grandes quantités de données
# On va créer un script Python pour essayer de télécharger des données publiques

python3 << 'PYTHON_SCRIPT'
# Essayer de télécharger des données depuis OpenSky Network
# Note: L'API publique a des limites, mais on peut essayer
try:
    import requests
    import json
    import random
    from datetime import datetime
    # Exemple: obtenir les états des avions en temps réel (limité à 1000)
    response = requests.get("https://opensky-network.org/api/states/all", timeout=10)
    if response.status_code == 200:
        data = response.json()
        if 'states' in data and len(data['states']) > 0:
            # Convertir en format SBS
            with open("tests/samples/sbs/sbs_opensky_sample.txt", "w") as f:
                count = 0
                for state in data['states'][:1000]:  # Limiter à 1000 pour l'API publique
                    if state and len(state) >= 17:
                        # Format SBS simplifié depuis les données OpenSky
                        callsign = state[1].strip() if state[1] else ""
                        hex_ident = state[0] if state[0] else ""
                        lat = state[6] if state[6] else 0
                        lon = state[5] if state[5] else 0
                        alt = state[7] if state[7] else 0
                        speed = state[9] if state[9] else 0
                        track = state[10] if state[10] else 0
                        
                        if hex_ident:
                            from datetime import datetime
                            now = datetime.now()
                            date_str = now.strftime("%Y/%m/%d")
                            time_str = now.strftime("%H:%M:%S.%f")[:-3]
                            msg = f"MSG,3,145,{random.randint(10000,99999)},{hex_ident},{random.randint(10000,99999)},{date_str},{time_str},{date_str},{time_str},{callsign},{int(alt)},{speed:.1f},{track:.1f},{lat:.6f},{lon:.6f},,,,,,0"
                            f.write(msg + "\n")
                            count += 1
                
                print(f"✓ Téléchargé {count} messages SBS depuis OpenSky Network")
        else:
            print("⚠ OpenSky Network API: Pas de données disponibles (limites API)")
    else:
        print(f"⚠ OpenSky Network API: Erreur {response.status_code}")
except ImportError:
    print("⚠ Module 'requests' non disponible - skip du téléchargement OpenSky")
    print("  (Les données synthétiques ont été générées avec succès)")
except Exception as e:
    print(f"⚠ Impossible de télécharger depuis OpenSky Network: {e}")
    print("  (C'est normal, l'API publique a des limites)")
PYTHON_SCRIPT

echo ""
echo "=== Résumé ==="
echo "Données générées:"
echo "  - SBS: tests/samples/sbs/sbs_large_dataset.txt (15000 messages)"
echo "  - NMEA: tests/samples/nmea/nmea_large_dataset.txt (12000 messages)"
echo ""
echo "Total: > 27000 messages pour les tests"

