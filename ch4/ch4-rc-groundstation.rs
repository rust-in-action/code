use std::rc::Rc;                          // <1>

#[derive(Debug)]
struct GroundStation {}

fn main() {
  let base = Rc::new(GroundStation {});   // <2>

  println!("{:?}", base);                 // <3>
}