/// equation.rs
/// 
/// From Chapter 1 - Rust in Action

fn three() -> i32 {
  1 + 2
}

fn main() {
  let a = 7;
  let b = three();
  let c = a + b;
  println!("{} + {} = {}", a, b, c);
}
