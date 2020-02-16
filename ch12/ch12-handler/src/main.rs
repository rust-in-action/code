#![cfg(not(windows))]

extern crate libc;

use std::mem;
use std::time;
use std::thread::{sleep};
use libc::{SIGTERM, SIGALRM, SIGHUP, SIGQUIT};

static mut SHUT_DOWN: bool = false;

#[inline]
fn register_signal_handler() {
    let fn_ptr: fn(i32) -> () = handle_signals;                     // <1> hardcoding for simplicity

    unsafe {                                                       
        let fn_ptr_as_usize: usize = mem::transmute(fn_ptr);        // <2>
        libc::signal(SIGTERM, fn_ptr_as_usize);                     // <3>
    }
}

#[allow(dead_code)]                                                // <4>
fn handle_signals(sig: i32) {
    register_signal_handler();

    unsafe {                                                          // <4>
        SHUT_DOWN = match sig {                                    // <5>
            SIGALRM  => false,                                     // <5>
            SIGHUP   => false,                                     // <5>
            SIGTERM  => true,                                      // <5>
            SIGQUIT  => true,                                      // <5>
            _ => false,                                            // <5>
        };
    }
}

fn main() {
    register_signal_handler();
    let delay = time::Duration::from_secs(1);

    for i in 1.. {                                               // <6>
        unsafe {                                                    // <6>
            if SHUT_DOWN {
                println!();  // only print a newline character
                return;
            }
        }

        sleep(delay);
        print!(".");

        if i > 3 {
            unsafe {
                libc::raise(SIGTERM);
            }
        }
    }
}