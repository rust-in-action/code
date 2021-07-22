struct Hostname(String);                 // <1>

fn connect(host: Hostname) {             // <2>
  println!("connected to {}", host.0);   // <3>
}

fn main() {
    let ordinary_string = String::from("localhost");
    let host = Hostname ( ordinary_string.clone() );

    connect(ordinary_string);
}
