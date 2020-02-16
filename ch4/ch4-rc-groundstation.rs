use std::rc::Rc;

#[derive(Debug)]
struct GroundStation {}

fn main() {
  let base: Rc<GroundStation> = Rc::new(GroundStation {});
  
  println!("{:?}", base);
}