struct Celsius(f32);

fn main() {
  let nice_and_warm = Celsius(22.3);
  println!("It's {:?}℃ today.", nice_and_warm);
}
