struct IPv4 (u8, u8, u8, u8);

impl From<[u8;4]> for IPv4 {
  fn from(bytes: [u8;4]) -> IPv4 {
    IPv4 (bytes[0], bytes[1], bytes[2], bytes[3])
  }
}

fn main() {
  let numbers: [u8;4] = [127, 0, 0, 1];
  let host = IPv4::from(numbers);

  println!("localhost: {}.{}.{}.{}", host.0, host.1, host.2, host.3);
}
