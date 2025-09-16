#![cfg(feature="alloc")]
use oi4_dnp_encoding::{encode, encode_into, encoded_len, decode};
use std::process::Stdio;

struct FailingWriter { fail_after: usize, writes: usize }
impl core::fmt::Write for FailingWriter { fn write_str(&mut self, s: &str) -> core::fmt::Result { let _ = s; if self.writes==self.fail_after { return Err(core::fmt::Error); } self.writes+=1; Ok(()) } }

#[test]
fn encode_basic_and_reserved() {
    assert_eq!(encode("ABC"), "ABC");
    assert_eq!(encode("Hello World!"), "Hello,20World,21");
    assert_eq!(encode(",#:/?"), ",2C,23,3A,2F,3F");
}

#[test]
fn encode_unicode_passthrough() {
    let s = "äöüΩ🙂"; assert_eq!(encode(s), s);
}

#[test]
fn encoded_len_matches_output() {
    let s = "Hello World!#"; let out = encode(s); assert_eq!(out.len(), encoded_len(s));
}

#[test]
fn encode_into_matches_encode() {
    let s = "Hello World!#Test"; let expected = encode(s); let mut buf = String::with_capacity(expected.len()); encode_into(s, &mut buf).unwrap(); assert_eq!(buf, expected);
}

#[test]
fn encode_into_error_path() {
    let mut w = FailingWriter { fail_after: 0, writes: 0 }; let _ = encode_into("Hello", &mut w).err().expect("fmt::Error expected");
}

#[test]
fn full_ascii_rule_check() {
    // Build ASCII except CR/LF
    let mut src = Vec::new(); for b in 0x01u8..=0x7Fu8 { if b==b'\n'||b==b'\r' {continue;} src.push(b);} let s = String::from_utf8(src).unwrap(); let enc = encode(&s);
    // commas form triplets
    let bytes = enc.as_bytes(); let mut i=0; while i<bytes.len(){ if bytes[i]==b',' { assert!(i+2<bytes.len()); assert!(bytes[i+1].is_ascii_hexdigit()); assert!(bytes[i+2].is_ascii_hexdigit()); i+=3;} else {i+=1;} }
}

#[test]
fn encoded_len_multibyte_unicode() {
    let s = "ÄΩ漢🙂"; // all should pass through
    assert_eq!(encoded_len(s), s.len());
    assert_eq!(encode(s), s);
}

#[test]
fn golden_basic_examples() {
    let cases = [
        (" ", ",20"), (",", ",2C"), ("#", ",23"), ("/", ",2F"), (":", ",3A"), ("?", ",3F"), ("@", ",40"), ("[", ",5B"), ("]", ",5D"), ("\\", ",5C"), ("{", ",7B"), ("}", ",7D")
    ];
    for (plain, exp) in cases { assert_eq!(encode(plain), exp); }
}

#[test]
fn differential_go_optional() {
    // Only runs if GO_DNP_REPO and go tool available
    let repo = std::env::var("GO_DNP_REPO").ok();
    if repo.is_none() { return; }
    if std::process::Command::new("go").arg("version").stdout(Stdio::null()).stderr(Stdio::null()).status().is_err() { return; }
    let manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let script = std::path::Path::new(&manifest).join("tools").join("compare_go.sh");
    if !script.exists() { return; }
    let inputs = ["Hello World!", "äöüΩ", "#,:?@[]", "Mix123-_~End"].join("\n");
    let mut child = std::process::Command::new("bash")
        .arg(script)
        .current_dir(&manifest)
        .env("GO_DNP_REPO", repo.unwrap())
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn().expect("spawn compare");
    if let Some(mut stdin) = child.stdin.take() { use std::io::Write; stdin.write_all(inputs.as_bytes()).unwrap(); }
    let out = child.wait_with_output().unwrap();
    if !out.status.success() { panic!("Go differential mismatch: {}", String::from_utf8_lossy(&out.stderr)); }
}

#[test]
fn encode_long_string_roundtrip() {
    let pattern = "Hello World!#";
    let src = pattern.repeat(1024);
    let enc = encode(&src);
    let dec = decode(&enc).unwrap();
    assert_eq!(dec, src);
}

#[test]
fn encode_mixed_unicode_and_reserved() {
    let s = "äöüΩ,漢字#🙂";
    let enc = encode(s);
    assert!(enc.contains(",2C")); // comma
    assert!(enc.contains(",23")); // '#'
    assert_eq!(decode(&enc).unwrap(), s);
}
