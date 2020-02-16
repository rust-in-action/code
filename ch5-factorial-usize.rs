fn factorial(n: usize) -> usize {
  match n {
    0 => 0,
    1 => 1,
    _ => n + factorial(n-1),
  }
}

fn main() {
  let n = 10;
  println!("{}", factorial(n));
}
