#![feature(rustc_private)]

use std::env;
use std::libc::{SIGTERM, SIGALRM, SIGHUP, SIGQUIT};

const USAGE: &'static str = "Usage: sig <signal> <pid>";

fn main() {
    let mut args = env::args();
    let signal =  args.nth(1).expect(USAGE);
    let pid=  args.nth(2).expect(USAGE);


    println!("{} <-{}!", pid, signal);
}
