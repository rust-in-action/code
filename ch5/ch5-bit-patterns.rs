fn main() {
  let zero: u16 = 0b0000_0000_0000_0000;
  let one:  u16 = 0b0000_0000_0000_0001;
  let two:  u16 = 0b0000_0000_0000_0010;
  // ...
  let sixtyfivethousand_533: u16 = 0b1111_1111_1111_1101;
  let sixtyfivethousand_534: u16 = 0b1111_1111_1111_1110;
  let sixtyfivethousand_535: u16 = 0b1111_1111_1111_1111;

  print!("{}, {}, {}, ..., ", zero, one, two);
  println!("{}, {}, {}", sixty5_533, sixty5_534, sixty5_535);
}
