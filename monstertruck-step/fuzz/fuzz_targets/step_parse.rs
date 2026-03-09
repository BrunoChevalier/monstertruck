#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Only attempt to parse valid UTF-8 strings.
    let input_str = match std::str::from_utf8(data) {
        Ok(s) => s,
        Err(_) => return,
    };

    // Feed arbitrary strings to the STEP parser.
    // This should never panic regardless of input.
    if let Ok(exchange) = ruststep::parser::parse(input_str) {
        // Exercise more code paths by iterating over data sections.
        for data_section in &exchange.data {
            // Access entity count to exercise deserialization paths.
            let _ = data_section.entities.len();
        }
    }
});
