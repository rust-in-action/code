//#![allow(unused_variables)]

extern crate rand;
use rand;

fn one_in_(n: usize) -> bool {
  rand::thread_rng().gen_weighted_bool(n)
}

#[derive(Debug)]
struct File {
  name: String,
  data: Vec<u8>,
}

impl File {
  fn new(name: &str) -> File {
    File {                     
      name: String::from(name),  
      data: Vec::new(),  
    }
  }

  fn new_with_data(name: &str, data: &Vec<u8>) -> File { // <1>
    let mut f = File::new(name);
    f.data = data.clone();
    f
  }

  fn read(self: &File, save_to: &mut Vec<u8>) -> usize { // <2>
    let mut tmp = self.data.clone();
    let read_length = tmp.len();
    save_to.reserve(read_length);
    save_to.append(&mut tmp);
    read_length
  }
}

fn open(f: &mut File) -> Result<File, String> {
  if one_in_(10_000) {
    return Err(String::from("Permission denied"))
  }
  Some(f)
}

fn close(f: &mut File) -> Result<File, String> {
  if one_in_(100_000) {
    return Err(String::from("Interrupted by signal!"))
  }
  Some(f)
}

fn main() {
  let f3_data: Vec<u8> = vec![114, 117, 115, 116, 33]; // <4>
  let mut f3 = File::new_with_data("2.txt", &f3_data); 

  let mut buffer: Vec<u8> = vec![];

  open(&mut f3);
  let f3_length = f3.read(&mut buffer); // <5>
  close(&mut f3);
  
  let text = String::from_utf8_lossy(&buffer);

  println!("{:?}", f3);
  println!("{} is {} bytes long", &f3.name, f3_length);
  println!("{}", text);
}