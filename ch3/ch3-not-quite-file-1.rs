#![allow(unused_variables)] // <1> Relax compiler warnings while working through ideas

type File = String; // <2> Create a type alias. The compiler won't distinguish between String & File, but your source code will.

fn open(f: &mut File) -> bool {
    true // <3> let's assume for the moment that this always succeeds
}

fn close(f: &mut File) -> bool {
    true // <3>
}

#[allow(dead_code)] // <4> Relaxes a compiler warning about an unused function
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! { // <5> Using `!` as a return type indicates to the Rust compiler that this function never returns
    unimplemented!() // <6> A macro that crashes the program if it is encountered
}

fn main() {
    let mut f1 = File::from("f1.txt"); // <7> With the type declaration at line 3, `File` "inherits" all of String's methods 
    open(&mut f1);
    //read(f1 , vec![]); // <8> There's little point in calling this method
    close(&mut f1);
}