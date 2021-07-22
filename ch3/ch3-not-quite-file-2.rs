#![allow(unused_variables)]       // <1>

#[derive(Debug)]                  // <2>
struct File {
  name: String,
  data: Vec<u8>,
}

fn open(f: &mut File) -> bool {   // <3>
  true
}

fn close(f: &mut File) -> bool {   // <3>
  true
}

fn read(
  f: &File,
  save_to: &mut Vec<u8>,
) -> usize { // <4>
  let mut tmp = f.data.clone();   // <5>
  let read_length = tmp.len();

  save_to.reserve(read_length);   // <6>
  save_to.append(&mut tmp);       // <7>
  read_length
}

fn main() {
  let mut f2 = File {
    name: String::from("2.txt"),
    data: vec![114, 117, 115, 116, 33],
  };

  let mut buffer: Vec<u8> = vec![];

  open(&mut f2);                            // <8>
  let f2_length = read(&f2, &mut buffer);   // <8>
  close(&mut f2);                           // <8>

  let text = String::from_utf8_lossy(&buffer);   // <9>

  println!("{:?}", f2);
  println!("{} is {} bytes long", &f2.name, f2_length);
  println!("{}", text)   // <10>
}
