// derived from intermezzos & phill-opp-os

#![allow(unused_variables)]
#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

use x86_64;

use core::intrinsics;
use core::panic::PanicInfo;

static START: usize = 0xb8000;
static WIDTH: usize = 120;
static HEIGHT: usize = 25;

static mut CURSOR: usize = 0;

struct Position {
    row: usize,
    column: usize,
}

impl Position {
    fn from_usize(position: usize) -> Self {
        Position {
            row: position / WIDTH,
            column: (position % WIDTH) / 2,
        }
    }

    fn to_usize(&self) -> usize {
        (self.row * WIDTH) + (self.column * 2)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Color {
    Black =	0x0,    White = 0xF,
    Blue = 0x1,     BrightBlue = 0x9,
    Green =	0x2,    BrightGreen = 0xA,
    Cyan = 0x3,     BrightCyan = 0xB,
    Red = 0x4,      BrightRed = 0xC,
    Magenta = 0x5,  BrightMagenta = 0xD,
    Brown =	0x6,    Yellow = 0xE,
    Gray = 0x7,     DarkGray = 0x8
}

macro_rules! screen {
    () => (
        unsafe { core::slice::from_raw_parts_mut(0xb8000 as *mut u8, 4000) }
    );
}

struct Cell {
    position: Position,
    foreground: Color,
    background: Color,
    character: u8,
    blink: bool,
}

impl Cell {
    fn as_color(&self) -> u8 {
        let mut color = 0;

        color |= (self.background as u8) << 4;
        color |= self.foreground as u8;

        if self.blink {
            color |= 0b1000_0000;
        }

        color
    }

    fn write(&self) {
        let xy = self.position.to_usize();

        let s = screen!();
        s[xy] = self.character;
        s[xy+1] = self.as_color();
    }
}

fn print_dot() {
    print_byte(b'.');
}

fn print_message(msg: &[u8]) {
    for byte in msg {
        print_byte(*byte);
        unsafe {
            CURSOR += 2;
        }
    }
}

fn print_byte(character: u8) {
    // TODO: check printable
    let xy = Position::from_usize(unsafe { CURSOR });
    let cell = Cell {
        character: character,
        position:xy,
        blink: false,
        foreground: Color::White,
        background: Color::Cyan,
    };
    cell.write();
}

#[lang = "eh_personality"]
extern fn eh_personality() {
}

#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
  unsafe {
    intrinsics::abort();
  }
}


#[no_mangle]
pub extern "C" fn _start() -> ! {

    unsafe {
        CURSOR = 1;
    }
    print_message(b"Rust in Action");
    unsafe {
        CURSOR = WIDTH + 40; // (WIDTH*2);
    }
    print_message(b"www.rustinaction.com");

    unsafe {
        CURSOR = WIDTH * 2;
    }

    loop {
        x86_64::instructions::hlt();
    }
}
