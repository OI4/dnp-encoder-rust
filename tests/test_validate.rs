#![cfg(feature = "alloc")]
use oi4_dnp_encoding::{validate_dnp, Rules};

#[test]
fn validate_basic_ok() { validate_dnp("ABC123-._~", &Rules::default()).unwrap(); }

#[test]
fn validate_escape_ok() { validate_dnp(",2C", &Rules::default()).unwrap(); }

#[test]
fn validate_lone_comma() { assert!(validate_dnp(",", &Rules::default()).is_err()); }

#[test]
fn validate_short_escape() { assert!(validate_dnp(",2", &Rules::default()).is_err()); }

#[test]
fn validate_invalid_hex_digit() { assert!(validate_dnp(",2G", &Rules::default()).is_err()); }

#[test]
fn validate_unescaped_reserved_when_enforce() {
    let r = Rules { enforce_reserved_masking: true, allow_lowercase_hex: true };
    assert!(validate_dnp(" ", &r).is_err());
    assert!(validate_dnp(",", &r).is_err());
}

#[cfg(not(feature = "strict"))]
#[test]
fn validate_lowercase_allowed_runtime_rule() {
    let r = Rules { enforce_reserved_masking: false, allow_lowercase_hex: true };
    validate_dnp(",2c", &r).unwrap();
}

#[test]
fn validate_lowercase_rejected_rule_false() {
    let r = Rules { enforce_reserved_masking: false, allow_lowercase_hex: false };
    assert!(validate_dnp(",2c", &r).is_err());
}

#[cfg(feature = "strict")]
#[test]
fn validate_lowercase_rejected_strict_even_if_rule_allows() {
    let r = Rules { enforce_reserved_masking: false, allow_lowercase_hex: true };
    assert!(validate_dnp(",2c", &r).is_err());
}

#[test]
fn validate_unicode_passthrough() {
    let r = Rules { enforce_reserved_masking: true, allow_lowercase_hex: false };
    validate_dnp("äöΩ🙂", &r).unwrap();
}

