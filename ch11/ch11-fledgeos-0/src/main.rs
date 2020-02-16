#![feature(core_intrinsics)]
#![no_std]
#![no_main]

use core::intrinsics;
use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
  unsafe {
    intrinsics::abort();
  }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
  let mut framebuffer = 0xb8000 as *mut u8;
  unsafe {
      framebuffer.offset( 1).write_volatile(0x30);
  }
  loop {}
}
