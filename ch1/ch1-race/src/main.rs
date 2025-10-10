use std::thread; // <1>

#[allow(unused_assignments, unused_variables)]
fn main() {
    let mut data = 100;

    thread::spawn(move || {
        data = 500;
    }); // <2>
    thread::spawn(move || {
        data = 1000;
    }); // <2>

    println!("{}", data);
}
