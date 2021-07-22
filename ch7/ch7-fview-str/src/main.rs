use std::io::prelude::*;                           <1>

const BYTES_PER_LINE: usize = 16;
const INPUT: &'static [u8] = br#"                  <2>
fn main() {
    println!("Hello, world!");
}"#;

fn main() -> std::io::Result<()> {
    let mut buffer: Vec<u8> = vec!();              <3>
    INPUT.read_to_end(&mut buffer)?;               <4>

    let mut position_in_input = 0;
    for line in buffer.chunks(BYTES_PER_LINE) {
        print!("[0x{:08x}] ", position_in_input);  <5>
        for byte in line {
            print!("{:02x} ", byte);
        }
        println!();                                <6>
        position_in_input += BYTES_PER_LINE;
    }

    Ok(())
}
