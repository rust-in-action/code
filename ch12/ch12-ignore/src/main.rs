use libc::{signal,raise};
use libc::{SIG_DFL, SIG_IGN, SIGTERM};

fn main() {
  unsafe {                              // <1>
    signal(SIGTERM, SIG_IGN);           // <2>
    raise(SIGTERM);                     // <3>
  }

  println!("ok");

  unsafe {
    signal(SIGTERM, SIG_DFL);           // <4>
    raise(SIGTERM);                     // <5>
  }

  println!("not ok");                   // <6>
}
