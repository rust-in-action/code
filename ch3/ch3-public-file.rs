#[derive(Debug)]
pub struct File { // <1>
  name: String,   // <2>
  data: Vec<u8>,  // <2>
}

impl File {
  pub fn new(name: &str) -> File {
    File {
      name: String::from(name),
      data: Vec::new(),
    }
  }
  
  pub fn len(&self) -> usize {
    self.data.len()
  }
  
  pub fn name(&self) -> String {
    self.name.clone() // <3>
  }
}


fn main() {
  let f1 = File::new("f1.txt");
  
  let f1_name = f1.name(); // <4>
  let f1_length = f1.len();
  
  println!("{:?}", f1);
  println!("{} is {} bytes long", f1_name, f1_length);
}