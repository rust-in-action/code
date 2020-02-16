use std::fs::File;
use std::error::Error;
use std::net::Ipv6Addr;

fn main() -> Result<(), Box<Error>> {
    let _f = File::open("invisible.txt")?;
    let _localhost = "::1".parse::<Ipv6Addr>()?;

    Ok(())
}