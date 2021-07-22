#[derive(Debug)]   // <1>
struct File {
  name: String,
  data: Vec<u8>,   // <2>
}

fn main() {
  let f1 = File {
    name: String::from("f1.txt"),   // <3>
    data: Vec::new(),               // <4>
  };

  let f1_name = &f1.name;           // <5>
  let f1_length = &f1.data.len();   // <5>

  println!("{:?}", f1);
  println!("{} is {} bytes long", f1_name, f1_length);
}
