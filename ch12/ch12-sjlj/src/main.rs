#![feature(link_llvm_intrinsics)]
#![allow(non_camel_case_types)]
#![cfg(not(windows))]                           // <1>

use libc::{
  SIGALRM, SIGHUP, SIGQUIT, SIGTERM, SIGUSR1,
};
use std::mem;

const JMP_BUF_WIDTH: usize =
  mem::size_of::<usize>() * 8;
type jmp_buf = [i8; JMP_BUF_WIDTH];

static mut SHUT_DOWN: bool = false;            // <2>
static mut RETURN_HERE: jmp_buf = [0; JMP_BUF_WIDTH];
const MOCK_SIGNAL_AT: usize = 3;               // <3>

extern "C" {
  #[link_name = "llvm.eh.sjlj.setjmp"]
  pub fn setjmp(_: *mut i8) -> i32;

  #[link_name = "llvm.eh.sjlj.longjmp"]
  pub fn longjmp(_: *mut i8);
}

#[inline]                                      // <4>
fn ptr_to_jmp_buf() -> *mut i8 {
  unsafe { &RETURN_HERE as *const i8 as *mut i8 }
}

#[inline]                                      // <4>
fn return_early() {
  let franken_pointer = ptr_to_jmp_buf();
  unsafe { longjmp(franken_pointer) };         // <5>
}

fn register_signal_handler() {
  unsafe {
    libc::signal(SIGUSR1, handle_signals as usize); // <6>
  }
}

#[allow(dead_code)]
fn handle_signals(sig: i32) {
  register_signal_handler();

  let should_shut_down = match sig {
    SIGHUP => false,
    SIGALRM => false,
    SIGTERM => true,
    SIGQUIT => true,
    SIGUSR1 => true,
    _ => false,
  };

  unsafe {
    SHUT_DOWN = should_shut_down;
  }

  return_early();
}

fn print_depth(depth: usize) {
  for _ in 0..depth {
    print!("#");
  }
  println!();
}

fn dive(depth: usize, max_depth: usize) {
  unsafe {
    if SHUT_DOWN {
      println!("!");
      return;
    }
  }
  print_depth(depth);

  if depth >= max_depth {
    return;
  } else if depth == MOCK_SIGNAL_AT {
    unsafe {
      libc::raise(SIGUSR1);
    }
  } else {
    dive(depth + 1, max_depth);
  }
  print_depth(depth);
}

fn main() {
  const JUMP_SET: i32 = 0;

  register_signal_handler();

  let return_point = ptr_to_jmp_buf();
  let rc = unsafe { setjmp(return_point) };
  if rc == JUMP_SET {
    dive(0, 10);
  } else {
    println!("early return!");
  }

  println!("finishing!")
}
