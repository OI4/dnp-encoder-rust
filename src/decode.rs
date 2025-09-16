#[cfg(feature = "strict")]
use crate::encode::is_unreserved;
#[cfg(feature = "alloc")]
use crate::error::{Error, ErrorKind};
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "strict")]
use crate::hex::{hex_val, has_lowercase_hex};
#[cfg(not(feature = "strict"))]
use crate::hex::hex_val;

#[cfg(feature = "alloc")]
pub fn decode(input: &str) -> Result<String, Error> {
    #[cfg(not(feature = "strict"))]
    {
        if !input.as_bytes().contains(&b',') {
            // fast path only in non-strict mode
            return Ok(String::from(input));
        }
    }
    let mut out = String::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] == b',' {
            // escape expected
            if i + 2 >= bytes.len() {
                return Err(Error::new(ErrorKind::LoneComma, Some(i)));
            }
            let h1 = bytes[i + 1];
            let h2 = bytes[i + 2];
            #[cfg(feature = "strict")]
            {
                if has_lowercase_hex(h1, h2) {
                    return Err(Error::new(ErrorKind::LowercaseHexInStrict, Some(i)));
                }
            }
            let v1 = hex_val(h1)
                .ok_or_else(|| Error::new(ErrorKind::InvalidHexDigit(h1 as char), Some(i + 1)))?;
            let v2 = hex_val(h2)
                .ok_or_else(|| Error::new(ErrorKind::InvalidHexDigit(h2 as char), Some(i + 2)))?;
            let val = (v1 << 4) | v2;
            out.push(val as char);
            i += 3;
        } else {
            // Copy next UTF-8 char verbatim (could be multibyte)
            let s = &input[i..];
            let ch = s.chars().next().unwrap();
            #[cfg(feature = "strict")]
            {
                if ch.is_ascii() {
                    let bch = ch as u8;
                    if !is_unreserved(bch) {
                        return Err(Error::new(ErrorKind::UnescapedReservedAscii(ch), Some(i)));
                    }
                }
            }
            out.push(ch);
            i += ch.len_utf8();
        }
    }
    Ok(out)
}
