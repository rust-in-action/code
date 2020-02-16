use std::fs::File;
use std::net::Ipv6Addr;

fn main() -> Result<(), std::io::Error> {
    let _f = File::open("invisible.txt")?;     // <1> `File::open()` returns `Result<(), std::io::Error>`
    let _localhost = "::1".parse::<Ipv6Addr>()?; // <2> `"".parse::<Ipv6Addr>()` returns `Result<Ipv6Addr, std::net::AddrParseError>`

    Ok(())
}