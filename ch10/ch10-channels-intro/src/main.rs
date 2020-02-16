#[macro_use] // <1> The `select!` macro used on line 12 originates here.
extern crate crossbeam; // <2> The `crossbeam` crate provides many concurrency primitive. Notably for us, it includes excellent implementations of channels for us to make use of.

use std::thread;
use crossbeam::channel::{unbounded}; // <3> `crossbeam::channels` has two types of channels, bounded and unbounded. Bounded channels have a fixed capacity.

fn main() {
    let (tx, rx) = unbounded(); // <4> Creating a channel involves calling a function that returns `Sender<T>` and `Receiver<T>`. In this example, the compiler detects that we are using a number and creates `Sender<i32>` and `Receiver<i32>`.
    thread::spawn(move || {
        tx.send(42);
    });
    select!{ // <5> The `select!` macro takes its name from other messaging systems, such as the POSIX sockets API. It allows the main thread to block and wait for a message.
       recv(rx) -> msg => println!("{:?}", msg), // <6> `recv(rx)` is syntax defined by the macro.
    }
}
