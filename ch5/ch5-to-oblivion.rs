fn main() {
  let mut i: u16 = 0;
  print!("{}..", i);

  loop {
      i += 1000;
      print!("{}..", i);
      if i % 10000 == 0 {
          print!{"\n"}
      }
  }
}
