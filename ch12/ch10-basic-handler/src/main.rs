#![cfg(not(windows))]                                               // <1> Indicates to `cargo`/`rustc` that this code won't run on MS Windows.

extern crate libc;                                                  // <2> Imports the libc crate, enabling access to the kernel's functionality, such as signals.

use std::time;                                                      // <3> Brings the `Duration` type into scope, enabling us to represent spans of time.
use std::thread::{sleep};                                           // <4> Enables our program to pause its execution.
use libc::{SIGTERM, SIGUSR1};                                       // <5> Brings the two signal constants into local scope.

static mut SHUT_DOWN: bool = false;                                 // <6> Initialize a Boolean mutable static to false.

fn main() {
    register_signal_handlers();

    let delay = time::Duration::from_secs(1);

    for i in 1_usize.. { // <7>
        println!("{}", i);
        unsafe {
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
        unsafe {
            libc::raise(signal);
        }
    }
    unreachable!();
}

fn register_signal_handlers() {
    unsafe {                                                        // <7> Calling functions within `libc` requires an `unsafe` block, as it's outside of Rust's control.
        libc::signal(SIGTERM, handle_sigterm as usize);             // <8> `libc::signal` takes a signal name (technically, an integer) and an address of a function (a _function pointer_, albeit untyped) as arguments and associates the signal with the function.
        libc::signal(SIGUSR1, handle_sigusr1 as usize);             // <9> Why `usize`? `libc:signal()` requires an integer as its second argument. As _function pointers_. `handle_sigterm` and `handle_sigusr1` have the type `fn(i32) -> ()`.
    }
}

#[allow(dead_code)]                                                 // <10>  Without this attribute, `rustc` warns that this function is never run.
fn handle_sigterm(_signal: i32) {
    register_signal_handlers();

    println!("SIGTERM");

    unsafe {
        SHUT_DOWN = true;
    }
}

#[allow(dead_code)]                                                // <10>
fn handle_sigusr1(_signal: i32) {
    register_signal_handlers();

    println!("SIGUSR1");
}
