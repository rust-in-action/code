#![allow(unused_variables)]              // <1>

type File = String;                      // <2>

fn open(f: &mut File) -> bool {
    true   // <3>
}

fn close(f: &mut File) -> bool {
    true                                 // <3>
}

#[allow(dead_code)]                      // <4>
fn read(f: &mut File,
        save_to: &mut Vec<u8>) -> ! {    // <5>
    unimplemented!()                     // <6>
}

fn main() {
    let mut f1 = File::from("f1.txt");  // <7>
    open(&mut f1);
    //read(f1, vec![]);                 // <8>
    close(&mut f1);
}