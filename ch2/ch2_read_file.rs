use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn main() {
    let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "../README.md"].iter().collect();
    let f = File::open(path).unwrap(); // <1>
    let mut reader = BufReader::new(f);
    let mut line = String::new(); // <2>

    loop {
        let len = reader.read_line(&mut line).unwrap(); // <3>
        if len == 0 {
            break;
        }

        println!("{} ({} bytes long)", line, len);
        line.truncate(0); // <4>
    }
}
