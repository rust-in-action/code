#![cfg(not(windows))]                            // <1>

use std::time::{Duration};
use std::thread::{sleep};
use libc::{SIGTERM, SIGUSR1};

static mut SHUT_DOWN: bool = false;

fn main() {
  register_signal_handlers();                    // <2>

  let delay = Duration::from_secs(1);

  for i in 1_usize.. {
    println!("{}", i);
    unsafe {                                     // <3>
      if SHUT_DOWN {
        println!("*");
        return;
      }
    }

    sleep(delay);

    let signal = if i > 2 {
      SIGTERM
    } else {
      SIGUSR1
    };

    unsafe {                                       // <4>
      libc::raise(signal);
    }
  }
  unreachable!();
}

fn register_signal_handlers() {
  unsafe {                                         // <4>
    libc::signal(SIGTERM, handle_sigterm as usize);
    libc::signal(SIGUSR1, handle_sigusr1 as usize);
  }
}

#[allow(dead_code)]                                // <5>
fn handle_sigterm(_signal: i32) {
  register_signal_handlers();                      // <6>

  println!("SIGTERM");

  unsafe {                                         // <7>
    SHUT_DOWN = true;
  }
}

#[allow(dead_code)]                                // <5>
fn handle_sigusr1(_signal: i32) {
  register_signal_handlers();                      // <6>

  println!("SIGUSR1");
}
