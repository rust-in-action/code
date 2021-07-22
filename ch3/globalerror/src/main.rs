use rand::{random};            // <1>

static mut ERROR: isize = 0;   // <2>

struct File;                   // <3>

#[allow(unused_variables)]
fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    if random() && random() && random() {   // <4>
        unsafe {
            ERROR = 1;                      // <5>
        }
    }

    0                          // <6>
}

#[allow(unused_mut)]           // <7>
fn main() {
    let mut f = File;
    let mut buffer = vec![];

    read(&f, &mut buffer);
    unsafe {   // <8>
        if ERROR != 0 {
            panic!("An error has occurred!")
        }
    }
}