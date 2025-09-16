#![cfg(feature = "std")]
use std::io::{self, Read};
use std::process::ExitCode;

fn read_all_stdin() -> io::Result<String> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;
    if buf.ends_with('\n') { buf.pop(); if buf.ends_with('\r') { buf.pop(); } }
    Ok(buf)
}

fn usage() {
    eprintln!("Usage: oi4-dnp-encoding-cli <encode|decode> [TEXT]\nIf TEXT omitted, reads from stdin.");
}

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let cmd = match args.next() { Some(c) => c, None => { usage(); return ExitCode::from(2); } };
    let data = match args.next() { Some(rest) => rest, None => match read_all_stdin() { Ok(s) => s, Err(e) => { eprintln!("stdin error: {e}"); return ExitCode::from(3);} } };
    match cmd.as_str() {
        "encode" => {
            #[cfg(feature = "alloc")]
            {
                let out = oi4_dnp_encoding::encode(&data);
                println!("{out}");
                ExitCode::SUCCESS
            }
            #[cfg(not(feature = "alloc"))]
            { eprintln!("alloc feature required for encode CLI"); ExitCode::from(4) }
        }
        "decode" => {
            #[cfg(feature = "alloc")]
            {
                match oi4_dnp_encoding::decode(&data) {
                    Ok(out) => { println!("{out}"); ExitCode::SUCCESS }
                    Err(e) => { eprintln!("decode error: {e}"); ExitCode::from(5) }
                }
            }
            #[cfg(not(feature = "alloc"))]
            { eprintln!("alloc feature required for decode CLI"); ExitCode::from(4) }
        }
        _ => { usage(); ExitCode::from(2) }
    }
}
