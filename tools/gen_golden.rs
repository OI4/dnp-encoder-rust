// gen_golden.rs
// Reads lines from stdin and prints: <input>\t<encoded>
// Usage: cargo run --bin gen_golden < inputs.txt > golden.tsv

use std::io::{self, BufRead};

fn main() {
    #[cfg(not(feature = "alloc"))]
    {
        eprintln!("alloc feature required");
        std::process::exit(1);
    }
    #[cfg(feature = "alloc")]
    {
        use oi4_dnp_encoding::encode;
        for line in io::stdin().lock().lines() {
            let l = line.unwrap();
            let enc = encode(&l);
            println!("{}\t{}", l, enc);
        }
    }
}

