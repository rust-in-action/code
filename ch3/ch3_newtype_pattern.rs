struct Hostname(String); // <1>

fn connect(host: Hostname) {
    // <2>
    println!("connected to {}", host.0); // <3>
}

pub fn main() {
    let ordinary_string = String::from("localhost");
    let host = Hostname(ordinary_string.clone());

    connect(host);
}
