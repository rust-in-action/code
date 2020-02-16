fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
  *i + *j // <1>
}

fn main() {
  let res = add_with_lifetimes(&10, &20); // <2> <3>
  println!("{}", res);
}