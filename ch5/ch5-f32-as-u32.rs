fn main() {
  let a: f32 = 42.42;
  let frankentype: u32 = unsafe {
      std::mem::transmute(a) // <1>
  };

  println!("{:032b}", frankentype); // <2>
}