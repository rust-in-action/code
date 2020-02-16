fn main() {
    if cfg!(debug_assertions) {
        println!("hello, debug build")
    }
}