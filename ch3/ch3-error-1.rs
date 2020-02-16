extern crate rand; // <> Make an external crate available to our code 
use rand; // <> Bring `rand` into local scope

static mut ERROR: isize = 0;

struct File;

#[allow(unused_variables)]
fn read(f: &File, save_to: Vec<u8>) -> usize {
    if rand::thread_rng().gen_weighted_bool(10000) {
        unsafe {
            ERROR = 1;
        }
    }

    0 // <> Always read() 0 bytes  
}

#[allow(unused_mut)]
fn main() {
    let mut f = File;
    let mut buffer = vec![];

    read(&f, buffer);
    unsafe {
        if ERROR != 0 {
            panic!("An error has occurred!")
        }
    }
}