#![cfg(all(feature = "alloc", feature = "std"))]
use oi4_dnp_encoding::{Error, ErrorKind};

#[test]
fn error_variants_display_and_debug() {
    let cases = [
        (
            Error::new(ErrorKind::InvalidHexDigit('X'), Some(0)),
            "invalid hex digit 'X'",
            "InvalidHexDigit",
        ),
        (
            Error::new(ErrorKind::LoneComma, None),
            "lone comma",
            "LoneComma",
        ),
        (
            Error::new(ErrorKind::LowercaseHexInStrict, Some(5)),
            "lowercase hex",
            "LowercaseHexInStrict",
        ),
        (
            Error::new(ErrorKind::UnescapedReservedAscii('#'), Some(9)),
            "unescaped reserved ASCII '#'",
            "UnescapedReservedAscii",
        ),
    ];
    for (err, disp_snip, dbg_snip) in cases {
        let d = err.to_string();
        assert!(d.contains(disp_snip));
        let dbg = format!("{:?}", err);
        assert!(dbg.contains(dbg_snip));
        // accessors
        assert!(err.kind().to_string().len() > 0);
    }
}
