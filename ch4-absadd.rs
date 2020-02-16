fn main() {
  let x = 21;
  let y = -2;
  let z = add_abs(x, y);
}

fn absolute(a: i32) -> i32 {
  if a < 0 {
    return -a;
  }
  a
}

fn add_abs(a: i32, b: i32) -> i32 {
  let c = absolute(a) + absolute(b);
  c
}
