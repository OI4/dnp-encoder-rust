#![cfg(all(feature = "std", feature = "alloc"))]
use std::io::Write;
use std::process::{Command, Stdio};

fn bin() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_BIN_EXE_oi4-dnp-encoding-cli"))
}

#[test]
fn cli_usage_no_args() {
    let out = Command::new(bin())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&out.stderr).contains("Usage:"));
}

#[test]
fn cli_encode_arg() {
    let out = Command::new(bin())
        .arg("encode")
        .arg("Hello World!")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();
    assert!(out.status.success());
    assert_eq!(
        String::from_utf8_lossy(&out.stdout).trim(),
        "Hello,20World,21"
    );
}

#[test]
fn cli_encode_stdin() {
    let mut child = Command::new(bin())
        .arg("encode")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(b"Space -> \n")
        .unwrap();
    let out = child.wait_with_output().unwrap();
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains(",20"));
}

#[test]
fn cli_decode_arg() {
    let out = Command::new(bin())
        .arg("decode")
        .arg("Hello,20World,21")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();
    assert!(out.status.success());
    assert_eq!(String::from_utf8_lossy(&out.stdout).trim(), "Hello World!");
}

#[test]
fn cli_decode_error() {
    let out = Command::new(bin())
        .arg("decode")
        .arg(",G0")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(5));
    assert!(String::from_utf8_lossy(&out.stderr).contains("decode error:"));
}

#[test]
fn cli_unknown_command() {
    let out = Command::new(bin())
        .arg("bogus")
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(2));
}
