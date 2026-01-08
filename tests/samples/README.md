# Exemples de messages AFTN réels

Ce répertoire contient des exemples de messages AFTN réels utilisés pour valider le parser.

## Sources

Les exemples sont basés sur :
- Format standard ICAO pour les messages AFTN
- Spécification AFTN 3.4
- Exemples de la FAA (Federal Aviation Administration)
- Manuel EUR AMHS (ICAO)

## Types de messages

- **NOTAM** : Notice to Airmen
- **METAR** : Meteorological Aerodrome Report
- **TAF** : Terminal Aerodrome Forecast
- **SIGMET** : Significant Meteorological Information
- **AIRMET** : Airmen's Meteorological Information
- **ATIS** : Automatic Terminal Information Service
- **VOLMET** : Meteorological Information for Aircraft in Flight
- **FPL** : Flight Plan
- **POS** : Position Report

## Format

Chaque fichier contient un message AFTN complet au format :
```
[PRIORITÉ] [ORIGINE] [DESTINATIONS...] [JJHHMM] [CORPS] [/SEQ NUMÉRO]
```

## Utilisation

Ces exemples sont utilisés par les tests dans `tests/real_world_tests.rs` pour valider le parser.

