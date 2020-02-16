#[allow(unused_mut)]

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
  let f = File::open("readme.md").unwrap(); // <1>
  let reader = BufReader::new(f);

  for line_ in reader.lines() {
    let line = line_.unwrap();
    println!("{} ({} bytes long)", line, line.len());
  }
}
