#![allow(unused_variables)]       // <1>

#[derive(Debug)]
struct File;                      // <2>

trait Read {                      // <3>
    fn read(
      self: &Self,
      save_to: &mut Vec<u8>,
    ) -> Result<usize, String>;   // <4>
}

impl Read for File {
    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        Ok(0)                    // <5>
    }
}

fn main() {
    let f = File{};
    let mut buffer = vec!();
    let n_bytes = f.read(&mut buffer).unwrap();
    println!("{} byte(s) read from {:?}", n_bytes, f);
}
