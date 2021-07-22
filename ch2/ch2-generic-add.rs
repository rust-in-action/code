use std::ops::{Add};                             // <1>
use std::time::{Duration};                       // <2>

fn add<T: Add<Output = T>>(i: T, j: T) -> T {    // <3>
  i + j
}

fn main() {
  let floats = add(1.2, 3.4);      // <4>
  let ints = add(10, 20);          // <5>
  let durations = add(             // <6>
    Duration::new(5, 0),           // <6>
    Duration::new(10, 0)           // <6>
  );

  println!("{}", floats);
  println!("{}", ints);
  println!("{:?}", durations);    // <7>

}