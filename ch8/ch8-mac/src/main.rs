extern crate rand;

use rand::RngCore;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
struct MacAddress([u8; 6]);                        // <1>

impl Display for MacAddress {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let octet = &self.0;
    write!(
      f,
      "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}", // <2>
      octet[0], octet[1], octet[2],                // <2>
      octet[3], octet[4], octet[5]                 // <2>
    )
  }
}

// MAC address format summary:
// https://en.wikipedia.org/wiki/MAC_address#Address_details
//
// 1st Octet:
// -- bit 0 (LSB): 0 = unicast             / 1 = multicast
// -- bit 1:       0 = globally unique OUI / 1 = locally assigned
//
impl MacAddress {
  fn new() -> MacAddress {
    let mut octets: [u8; 6] = [0; 6];
    rand::thread_rng().fill_bytes(&mut octets);
    octets[0] |= 0b_0000_0010;                     // <3>
    octets[0] &= 0b_1111_1110;                     // <3>
    MacAddress { 0: octets }
  }

  fn is_local(&self) -> bool {
    (self.0[0] & 0b_0000_0010) == 0b_0000_0010
  }

  fn is_unicast(&self) -> bool {
    (self.0[0] & 0b_0000_0001) == 0b_0000_0000
  }
}

fn main() {
  let mac = MacAddress::new();
  assert!(mac.is_local());
  assert!(mac.is_unicast());
  println!("mac: {}", mac);
}
