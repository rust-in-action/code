use std::thread;                          // <1>

fn main() {
    let mut data = 100;

    thread::spawn(|| { data = 500; });    // <2>
    thread::spawn(|| { data = 1000; });   // <2>

    println!("{}", data);
}