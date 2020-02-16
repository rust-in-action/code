use std::{thread, time};

fn main() {
  for n in 1..1001 {
    let mut handlers: Vec<thread::JoinHandle<()>> = Vec::with_capacity(n);

    let start = time::Instant::now();
    for _m in 0..n {
      let handle = thread::spawn(|| {
        let pause = time::Duration::from_millis(20);
        thread::sleep(pause);
      });
      handlers.push(handle);
    }

    while let Some(handle) = handlers.pop() {
      handle.join();
    }

    let finish = time::Instant::now();
    println!("{}\t{:02?}", n, finish.duration_since(start));
  }
}
