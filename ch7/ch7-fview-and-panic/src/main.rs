use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::env;

const WIDTH: usize = 32;

fn main() {
    let arg1 = env::args().nth(1);
    let fname = arg1.expect("usage: fview FILENAME");

    let mut f = File::open(fname).expect(format!("Unable to open {}", fname));
    //let f = BufReader(f);

    let mut pos = 0;
    let mut buffer = [0; WIDTH];
    
    loop {
        let res = f.read_exact(&mut buffer).unwrap();
        print!("[0x{:08x}] ", pos);
        for byte in &buffer {
            match *byte {
                0x00 => print!(".  "),
                0xff => print!("## "),
                _    => print!("{:02x} ", byte),
            }
        }
        println!("");
        pos += WIDTH;
    }

}
