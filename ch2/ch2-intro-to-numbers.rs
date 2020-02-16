fn main() {
  let twenty = 20; // <1>
  let twenty_one: i32 = twenty + 1; // <2>
  let floats_okay = 21.0; // <3>
  let one_million = 1_000_000; // <4>

  println!("{}; {}; {}; {}", twenty, twenty_one, floats_okay, one_million);
}