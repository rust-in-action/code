use std::ops::{Add};

fn add<T: Add<Output = T>>(i: T, j: T) -> T {
  i + j
}

fn main() {
  let (a, b) = (1.2, 3.4);
  let (x, y) = (10, 20);

  let c = add(a,b);
  let z = add(x,y);

  println!("{} + {} = {}", a, b, c);
  println!("{} + {} = {}", x, y, z);
}
