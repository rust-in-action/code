extern crate num;

use num::{Integer, Unsigned};

fn fibonacci<T: Integer + Unsigned>(n: T) -> T {
  match n {
    0 => 0,
    1 => 1,
    _ => n + fibonacci(n-1),
  }
}

fn main() {
    let n = 10;
    println!("{}", fibonacci(n as u16));
    println!("{}", fibonacci(n as u64));
}
