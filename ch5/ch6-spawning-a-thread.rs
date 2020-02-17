#![feature(thread_id)]

use std::time;
use std::thread;

fn child_main() {
   let thread_id = thread::current().id() as u64;
   let delay_ms = 100 - (10 * thread_id);
   let delay = time::Duration::from_millis(delay_ms);
   std::thread::sleep(delay);

   println!("hello from {:?}", thread_id);
}

fn main() {
    let mut child_threads = vec![];

    for _ in ..5 {
        let child_thread = thread::spawn(child_main);
        child_threads.push(child_thread);
    }

    for child_thread in child_threads {
        child_thread.join();
    }
    print!("done");
}
