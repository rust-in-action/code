fn main() {
    let mut n_nonzero = 0;

    for i in 0..10000 {
        let ptr = i as *const u8;                // <1>
        let byte_at_addr = unsafe { *ptr };      // <2>

        if byte_at_addr != 0 {
            n_nonzero += 1;
        }
    }

    println!("non-zero bytes in memory: {}", n_nonzero);
}
