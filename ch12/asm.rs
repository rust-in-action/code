#![feature(asm)]

use std::asm;

fn main() {
    // let int_id: u8 = 42;
    unsafe {
        asm!("int 42");
    }
}

