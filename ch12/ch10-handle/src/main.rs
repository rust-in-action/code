extern crate libc; // 0.2.42

use std::time::{Duration};
use std::thread::{sleep};
use libc::{signal,raise};
use libc::{SIG_DFL, SIG_IGN, SIGTERM, SIGHUP};

type handler_ptr = extern "C" fn(i32) -> ();

static mut ABORT: bool = false;

#[inline(never)]
extern "C" fn handle_signals(sig: i32) { // rustc will warn that that this code is never run
    let handle_signals_ptr: handler_ptr = handle_signals;
    unsafe { // set the signal handler again, to prevent it resetting to SIG_DFL
        signal(SIGTERM, std::mem::transmute(handle_signals_ptr));
    }
    
    let should_abort = match sig {
        SIGTERM => true,
        SIGHUP => false,
        _ => false,
    };
    
    unsafe { // make a quick update here, and defer the real work to somewhere else
        ABORT = should_abort;
    }
} 

fn main() {
    let delay = Duration::from_secs(1);

    // "main loop"
    for i in 1..=60 {
        unsafe { // at every step, check to see if a signal has been sent
            if ABORT {
                break;
            }
        }

        sleep(delay);
        println!(". {}", i);
        
        if i > 2 {
            unsafe { raise(SIGTERM); }
        }
    }
}