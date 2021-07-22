use std::mem::transmute;

fn main() {
  let big_endian: [u8; 4]    = [0xAA, 0xBB, 0xCC, 0xDD];
  let little_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];

  let a: i32 = unsafe { transmute(big_endian)    }; <1>
  let b: i32 = unsafe { transmute(little_endian) }; <1>

  println!("{} vs {}", a, b);
}
