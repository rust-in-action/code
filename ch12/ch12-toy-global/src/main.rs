use rand;

static mut SHUT_DOWN: bool = false;

fn main() {
  loop {
    unsafe {                            // <1>
      SHUT_DOWN = rand::random();       // <2>
    }
    print!(".");

    if unsafe { SHUT_DOWN } {
      break
    };
  }
  println!()
}
