use std::fs::File;
use std::net::Ipv6Addr;

fn main() -> Result<(), std::io::Error> {
  let _f = File::open("invisible.txt")?;

  let _localhost = "::1"
    .parse::<Ipv6Addr>()?;
     
  Ok(())
}
