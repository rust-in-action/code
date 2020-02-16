static GLOBAL: i32 = 1000; // <1> Create a global static, which is a global variable in Rust programs

fn noop() -> *const i32 {
    let noop_local = 12345; // <2> Create a local variable within noop(). While an optimizer might clear this away, hopefully we r
    &noop_local as *const i32 // <3> Return the address of `noop_local` as a raw pointer
}

fn main() {
    let local_str = "a";            // <4>
    let local_int = 123;            // <4>
    let boxed_str = Box::new('b');  // <4>
    let boxed_int = Box::new(789);  // <4>
    let fn_int = noop();            // <4> Create various values of several types, including values on the heap.

    println!("GLOBAL:    {:p}", &GLOBAL as *const i32);    // <5>
    println!("local_str: {:p}", local_str as *const str);  // <5>
    println!("local_int: {:p}", &local_int as *const i32); // <5>
    println!("boxed_int: {:p}", Box::into_raw(boxed_int)); // <5>
    println!("boxed_str: {:p}", Box::into_raw(boxed_str)); // <5>
    println!("fn_int:    {:p}", fn_int);                   // <5> Print out the values' addresses
}
