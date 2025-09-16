use crate::encode::is_unreserved;
use crate::error::{Error, ErrorKind};

/// Validation rule set (runtime adjustable apart from compile-time `strict` feature).
#[derive(Debug, Clone, Copy, Default)]
pub struct Rules {
    /// If true, any ASCII char outside unreserved set must appear only as encoded triplet ",XX".
    pub enforce_reserved_masking: bool,
    /// Accept lowercase hex inside escapes when strict feature not active. (Default true)
    pub allow_lowercase_hex: bool,
}

impl Rules {
    pub const fn strict_like() -> Self {
        Self {
            enforce_reserved_masking: true,
            allow_lowercase_hex: false,
        }
    }
}

/// Validate an encoded DNP string against masking rules.
/// Does not attempt semantic validation beyond escape formatting & reserved usage.
pub fn validate_dnp(input: &str, rules: &Rules) -> Result<(), Error> {
    let bytes = input.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b',' {
            // Expect two hex digits
            if i + 2 >= bytes.len() {
                return Err(Error::new(ErrorKind::LoneComma, Some(i)));
            }
            let h1 = bytes[i + 1];
            let h2 = bytes[i + 2];
            // strict feature -> forbid lowercase
            #[cfg(feature = "strict")]
            {
                if has_lowercase_hex(h1, h2) {
                    return Err(Error::new(ErrorKind::LowercaseHexInStrict, Some(i)));
                }
            }

            if !rules.allow_lowercase_hex && has_lowercase_hex(h1, h2) {
                return Err(Error::new(ErrorKind::LowercaseHexInStrict, Some(i)));
            }

            if !is_hex(h1) {
                return Err(Error::new(
                    ErrorKind::InvalidHexDigit(h1 as char),
                    Some(i + 1),
                ));
            }
            if !is_hex(h2) {
                return Err(Error::new(
                    ErrorKind::InvalidHexDigit(h2 as char),
                    Some(i + 2),
                ));
            }
            i += 3;
            continue;
        }
        if b < 0x80 {
            // ASCII plain char
            if rules.enforce_reserved_masking && !is_unreserved(b) {
                return Err(Error::new(
                    ErrorKind::UnescapedReservedAscii(b as char),
                    Some(i),
                ));
            }
            i += 1;
        } else {
            // UTF-8 multibyte char boundary
            // Skip full char
            let rest = &input[i..];
            let ch = rest.chars().next().unwrap();
            i += ch.len_utf8();
        }
    }
    Ok(())
}

#[inline]
const fn is_hex(b: u8) -> bool {
    //matches!(b, b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')
    b.is_ascii_hexdigit()
}

#[inline]
fn has_lowercase_hex(h1: u8, h2: u8) -> bool {
    (b'a'..=b'f').contains(&h1) || (b'a'..=b'f').contains(&h2)
}
