#[allow(arithmetic_overflow)]    <1>

fn main() {
  let (a, b) = (200, 200);
  let c: u8 = a + b;             <2>
  println!("200 + 200 = {}", c);
}
