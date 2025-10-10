use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn main() {
    let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "../README.md"].iter().collect();
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);

    for line_ in reader.lines() {
        // <1>
        let line = line_.unwrap(); // <2>
        println!("{} ({} bytes long)", line, line.len());
    }
}
