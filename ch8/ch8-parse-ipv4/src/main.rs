use std::net::SocketAddr;

fn main() {
    let addr: SocketAddr  = "10.00.0.42:1234".parse().unwrap();
    println!("{:#?}", addr);
}