fn main() {
  let a      = 10; // <1>
  let b: i32 = 20; // <2>

  let c = add(a,  b);
  println!("a + b = {}", c);
}

fn add(i: i32, j: i32) -> i32 {
  i + j
}
