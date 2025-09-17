#![no_main]
use libfuzzer_sys::fuzz_target;
use oi4_dnp_encoding::{decode, encode};

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = core::str::from_utf8(data) {
        // Try decode; it may fail, that's fine.
        if let Ok(decoded) = decode(s) {
            // Re-encode and ensure stable roundtrip through decode again.
            let re = encode(&decoded);
            if let Ok(decoded2) = decode(&re) {
                debug_assert_eq!(decoded, decoded2);
            }
        }
    }
});

