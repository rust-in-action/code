// derived from intermezzos & phill-opp-os

//#![allow(unused_variables)]
//#![feature(lang_items)]
#![feature(core_intrinsics)]
//#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

//use x86_64;

use core::intrinsics;
use core::panic::PanicInfo;

//#[lang = "eh_personality"]
//extern fn eh_personality() {
//}

#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
  unsafe {
    intrinsics::abort();
  }
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut vga = 0xb8001 as *mut u8;
    unsafe { *vga = 0x30; }
    loop {
//        x86_64::instructions::hlt();
    }
}
