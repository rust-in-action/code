#[derive(PartialEq)] // <1>
struct Hostname(String); // <2>

fn main() {
    let ordinary_string = String::from("localhost"); 
    let host = Hostname ( ordinary_string.clone() );
    if host == ordinary_string { // <3>
      println!("huh?");
    };
}