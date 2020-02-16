#![allow(unused_variables)]

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

  fn new_with_data(name: &str, data: &Vec<u8>) -> File { // <1> This method has snuck in to deal with cases where we want to simulate cases where a file has pre-existing data
    let mut f = File::new(name);
    f.data = data.clone();
    f
  }

  fn read(self: &File, save_to: &mut Vec<u8>) -> usize { // <2> The `f` argument has been replaced with `self` 
    let mut tmp = self.data.clone();
    let read_length = tmp.len();
    save_to.reserve(read_length);
    save_to.append(&mut tmp);
    read_length
  }
}

fn open(f: &mut File) -> bool { // <3> These two can remain as-in until we've looked at error handling 
  true 
}

fn close(f: &mut File) -> bool { // <3>
  true
}

fn main() {
  let f3_data: Vec<u8> = vec![114, 117, 115, 116, 33]; // <4> An explicit type needs to be provided, as `vec!` can't infer the necessary type through the function boundary
  let mut f3 = File::new_with_data("2.txt", &f3_data); 

  let mut buffer: Vec<u8> = vec![];

  open(&mut f3);
  let f3_length = f3.read(&mut buffer); // <5> Here is the the change in the calling code 
  close(&mut f3);
  
  let text = String::from_utf8_lossy(&buffer);

  println!("{:?}", f3);
  println!("{} is {} bytes long", &f3.name, f3_length);
  println!("{}", text);
}