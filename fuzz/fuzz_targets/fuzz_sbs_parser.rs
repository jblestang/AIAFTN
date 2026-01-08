#![no_main]
use libfuzzer_sys::fuzz_target;
use aftn::SbsParser;

fuzz_target!(|data: &[u8]| {
    // Convertir les bytes en string, en ignorant les erreurs UTF-8
    if let Ok(input) = std::str::from_utf8(data) {
        // Essayer de parser - on ignore les erreurs car on teste la robustesse
        let _ = SbsParser::parse_message(input);
    }
});

