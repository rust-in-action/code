use std::io::prelude::*;                              // <1>

const BYTES_PER_LINE: usize = 16;
const INPUT: &'static [u8] = br#"
fn main() {
    println!("Hello, world!)
}"#;

fn main() -> std::io::Result<()> {
    let mut buffer: Vec<u8> = vec!();                 // <3>
    INPUT.read_to_end(&mut buffer)?;                  // <4>

    let mut position_in_input = 0;                    // <3>
    for line in buffer.chunks(BYTES_PER_LINE) {
        print!("[0x{:08x}] ", position_in_input);                    
        for byte in line {
            print!("{:02x} ", byte);
        }
        println!();
        position_in_input += BYTES_PER_LINE;
    }

    Ok(())
}


// use std::io::prelude::*;  // <1>

// const BYTES_PER_LINE: usize = 16; // <2>
// const INPUT: &'static [u8] = br#"
// fn main() {
//     println!("Hello, world!)
// }"#;

// fn main() -> std::io::Result<()> {
//     let mut f = INPUT;                                // <5>
//     let mut buffer: Vec<u8> = vec!();             // <7>

//     f.read_to_end(&mut buffer)?;

//     let mut pos = 0;                                  // <6>
//     for line in buffer.chunks(BYTES_PER_LINE) {
//         print!("[0x{:08x}] ", pos);
//         for byte in line {
//             print!("{:02x} ", byte);
//         }
//         println!();
//         pos += BYTES_PER_LINE;
//     }

//     Ok(())
// }


// fn main() {
//     let mut f = INPUT;                                // <5>
//     let mut pos = 0;                                  // <6>
//     let mut buffer = [0; BYTES_PER_LINE];             // <7>

//     loop {
//         let res = f.read_exact(&mut buffer);          // <8>
//         if res.is_err() {                             // <9>
//             println!("{:#?}", res);
//             break;                                    // <9>
//         }                                             // <9>

//         // if pos >= 120*f.len() { // needed because read_exact() will never fail in this case
//         //     break;
//         // }

//         print!("[0x{:08x}] ", pos);
//         for byte in &buffer {
//             print!("{:02x} ", byte)
//         }
//         println!();

//         pos += BYTES_PER_LINE;
//     }
// }
