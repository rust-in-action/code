fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
  *i + *j                                   // <1>
}

fn main() {
  let a = 10;
  let b = 20;
  let res = add_with_lifetimes(&a, &b);     // <2>

  println!("{}", res);
}