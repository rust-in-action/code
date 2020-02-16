struct Counter {
  value: u64, // <1>
}

impl Counter {
  fn new() -> Self { // <2> <3>
    Counter { value: 0 } // <4> <5>
  }

  fn incr(&mut self) { // <6>
    self.value += 1;
  }
}

fn main() {
   let mut counter = Counter::new();

   counter.incr();
   counter.incr();

   println!("{}", counter.value);
}
