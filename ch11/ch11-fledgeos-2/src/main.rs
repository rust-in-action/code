#![no_std]
#![no_main]
#![feature(core_intrinsics)]
#![feature(lang_items)]

use core::intrinsics;
use core::panic::PanicInfo;
use x86_64::instructions::{hlt};

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
  unsafe {
    intrinsics::abort();
  }
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() { }

#[no_mangle]
pub extern "C" fn _start() -> ! {
  let framebuffer = 0xb8000 as *mut u8;

  unsafe {
    framebuffer
      .offset(1)
      .write_volatile(0x30);
  }

  loop {
    hlt();
  }
}
