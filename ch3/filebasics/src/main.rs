//! Simulating files one step at a time.

/// Represents a "file", 
/// which probably lives on a file system.
#[derive(Debug)]
pub struct File {
  name: String,
  data: Vec<u8>,
}

impl File {
  /// New files are assumed to be empty, but a name is required.
  pub fn new(name: &str) -> File {
    File {
      name: String::from(name),
      data: Vec::new(),
    }
  }

  /// Returns the file's length in bytes.
  pub fn len(&self) -> usize {
    self.data.len()
  }

  /// Returns the file's name.
  pub fn name(&self) -> String {
    self.name.clone()
  }
}

fn main() {
  let f1 = File::new("f1.txt");

  let f1_name = f1.name();
  let f1_length = f1.len();

  println!("{:?}", f1);
  println!("{} is {} bytes long", f1_name, f1_length);
}