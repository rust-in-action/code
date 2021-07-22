#![no_std]                       // <1>
#![no_main]                      // <1>
#![feature(core_intrinsics)]     // <2>

use core::intrinsics;            // <2>
use core::panic::PanicInfo;      // <3>

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
  intrinsics::abort();           // <4>
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
  let framebuffer = 0xb8000 as *mut u8;

  unsafe {
    framebuffer
      .offset(1)                 // <5>
      .write_volatile(0x30);     // <6>
  }

  loop {}
}
