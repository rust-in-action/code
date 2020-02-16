fn main() {
    let mut n_nonzero = 0;  

    for i in 0..10000 { 
        let ptr = i as *const u8; // <1> Convert `i` to a `pass:[*const T]`, a "`raw pointer`" of type `u8`. Raw pointers allow programmers to inspect raw memory addresses. We treat every address as a unit, ignoring the fact that most values span multiple bytes.
        let byte_at_addr = unsafe { *ptr }; // <2> _Dereference_ the pointer. That is, read the value at address `i`. Another way of saying this is read the value being pointed to.

        if byte_at_addr != 0 {
            n_nonzero += 1;
        }
    }

    println!("non-zero bytes in memory: {}", n_nonzero);
}