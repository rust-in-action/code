#[derive(Debug,PartialEq)]
pub enum FileState {
  Open,
  Closed,
}

#[derive(Debug)]
struct File {
  pub name: String,
  data: Vec<u8>,
  pub state: FileState,
}

impl File {
  pub fn new(name: &str) -> File {
    File {
        name: String::from(name), 
        data: Vec::new(), 
        state: FileState::Closed
    }
  }
}

fn main() {
  let f7 = File::new("f7.txt");
  //...
  println!("{:?}", f7);
}