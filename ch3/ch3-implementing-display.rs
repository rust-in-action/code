#![allow(dead_code)] // <1> Silence warnings related to FileState::Open not being used

use std::fmt;  // <2> Bring the `std::fmt` crate into local scope, allowing us to make use of `fmt::Result`
use std::fmt::{Display};  // <3> Bring `Display` into local scope, avoiding the need for us to prefix it as `fmt::Display` in our code

#[derive(Debug,PartialEq)]
enum FileState {
  Open,
  Closed,
}

#[derive(Debug)]
struct File {
  name: String,
  data: Vec<u8>,
  state: FileState,
}

impl Display for FileState {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
     match *self {
         FileState::Open => write!(f, "OPEN"),     // <4> Sneakily, we can make use of `write!` to do the grunt work for us. Strings already implement `Display` themselves, so there's very little left for us to do.
         FileState::Closed => write!(f, "CLOSED"), // <4>
     }
   }
}

impl Display for File {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "<{} ({})>", self.name, self.state)  // <5> We can rely on the FileState Display implementation in our own code 
   }
}

impl File {
  fn new(name: &str) -> File {
    File {
        name: String::from(name), 
        data: Vec::new(), 
        state: FileState::Closed
    }
  }
}

fn main() {
  let f5 = File::new("f5.txt");
  //...
  println!("{:?}", f5); // <1>
  println!("{}", f5); // <1>
}