use std::mem;

fn main() {
  let big_endian: [u8; 4] = [
    0xAA, // 1101_1101
    0xBB, // 1100_1100
    0xCC, // 1011_1011
    0xDD, // 1010_1010
  ];

  let little_endian: [u8; 4] = [
    0xDD, // 1010_1010
    0xCC, // 1011_1011
    0xBB, // 1100_1100
    0xAA, // 1101_1101
  ];

  let (a,b): (i32, i32) = unsafe {
    (mem::transmute(big_endian), mem::transmute(little_endian))
  };

  println!("{} vs {}", a, b);
}
