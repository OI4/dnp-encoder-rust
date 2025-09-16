//! Internal helpers for hexadecimal digit handling.
//! Kept `pub(crate)` to avoid exposing implementation details.

#[inline]
pub(crate) const fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(10 + (b - b'a')),
        b'A'..=b'F' => Some(10 + (b - b'A')),
        _ => None,
    }
}

#[inline]
pub(crate) const fn is_hex(b: u8) -> bool {
    matches!(b, b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')
}

#[inline]
pub(crate) const fn has_lowercase_hex(h1: u8, h2: u8) -> bool {
    matches!(h1, b'a'..=b'f') || matches!(h2, b'a'..=b'f')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_val_basic() {
        assert_eq!(hex_val(b'0'), Some(0));
        assert_eq!(hex_val(b'9'), Some(9));
        assert_eq!(hex_val(b'A'), Some(10));
        assert_eq!(hex_val(b'F'), Some(15));
        assert_eq!(hex_val(b'a'), Some(10));
        assert_eq!(hex_val(b'f'), Some(15));
        assert_eq!(hex_val(b'g'), None);
        assert_eq!(hex_val(b'/'), None);
    }

    #[test]
    fn test_is_hex() {
        for b in b"0123456789ABCDEFabcdef" { assert!(is_hex(*b)); }
        for b in b"GXYZ!" { assert!(!is_hex(*b)); }
    }

    #[test]
    fn test_has_lowercase_hex() {
        assert!(has_lowercase_hex(b'a', b'0'));
        assert!(has_lowercase_hex(b'0', b'f'));
        assert!(has_lowercase_hex(b'a', b'f'));
        assert!(!has_lowercase_hex(b'A', b'0'));
        assert!(!has_lowercase_hex(b'0', b'F'));
    }
}

