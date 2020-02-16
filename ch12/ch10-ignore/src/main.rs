extern crate libc; // 0.2.42

use libc::{signal,raise};
use libc::{SIG_DFL, SIG_IGN, SIGTERM};

fn main() {
    unsafe {
        signal(SIGTERM, SIG_IGN);
        raise(SIGTERM);
    }
    println!("ok");
    
    unsafe {
        signal(SIGTERM, SIG_DFL);
        raise(SIGTERM);
    }
    println!("not ok");
}