#[derive(Debug)]
struct Value<V>(usize, usize, V);

struct SparseMatrix<V> {
    values: Vec<Value<V>>
}

// impl SparseMatrix {
//     fn 
// }



fn main () {
  let val = Value(0, 1, 12.078);
  
  println!("{:?}", val);
}


