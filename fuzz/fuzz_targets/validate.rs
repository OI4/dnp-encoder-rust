#![no_main]
use libfuzzer_sys::fuzz_target;
use oi4_dnp_encoding::{validate_dnp, Rules};

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = core::str::from_utf8(data) {
        let _ = validate_dnp(s, &Rules::default());
        // Also attempt stricter rule (not the strict feature, just runtime rule)
        let strict_like = Rules { enforce_reserved_masking: true, allow_lowercase_hex: false };
        let _ = validate_dnp(s, &strict_like);
    }
});

