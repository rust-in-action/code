use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
struct MacAddress([u8; 6]);

impl Display for MacAddress {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let octet = self.0;
    write!(
      f,
      "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
      octet[0], octet[1], octet[2], octet[3], octet[4], octet[5]
    )
  }
}

impl MacAddress {
  fn is_local(&self) -> bool {
    (self.0[0] & 0b_0000_0010) == 0
  }

  fn is_universal(&self) -> bool {
    !self.is_local()
  }

  fn is_unicast(&self) -> bool {
    (self.0[0] & 0b_0000_0001) == 0
  }

  fn is_multicast(&self) -> bool {
    !self.is_unicast()
  }
}

fn main() -> Result<(), std::io::Error> {
  let mac = MacAddress([0x6, 0, 0, 0, 0, 0]);
  println!("mac: {}", mac);
  println!("mac: {:?}", mac);
  println!("universal?: {:?}", mac.is_universal());

  Ok(())
}
