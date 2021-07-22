fn main() {            //<1>
  let a = 10;          //<2>
  let b: i32 = 20;     //<3>
  let c = 30i32;       //<4>
  let d = 30_i32;      //<5>
  let e = add(add(a, b), add(c, d));

  println!("( a + b ) + ( c + d ) = {}", e);
}

fn add(i: i32, j: i32) -> i32 {    //<6>
  i + j                            //<7>
}