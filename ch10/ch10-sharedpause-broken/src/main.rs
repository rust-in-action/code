use std::{thread,time};

fn main() {
  let pause = time::Duration::from_millis(20);
  let handle1 = thread::spawn(|| {
    thread::sleep(pause);
  });
  let handle2 = thread::spawn(|| {
    thread::sleep(pause);
  });

  handle1.join();
  handle2.join();
}
