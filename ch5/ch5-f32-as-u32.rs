fn main() {
  let a: f32 = 42.42;
  let frankentype: u32 = unsafe { std::mem::transmute(a) };   // <1>

  println!("{}", frankentype);                                // <2>
  println!("{:032b}", frankentype);                           // <3>

  let b: f32 = unsafe { std::mem::transmute(frankentype) };   // <4>
  println!("{}", b);                                          // <4>