fn main() {
  let a: f32 = 42.42;
  let frankentype: u32 = unsafe {
    std::mem::transmute(a)                  // <1>
  };

  println!("{}", frankentype);              // <2>
  println!("{:032b}", frankentype);         // <3>

  let b: f32 = unsafe {
    std::mem::transmute(frankentype)
  };
  println!("{}", b);
  assert_eq!(a, b);                        // <4>
}
