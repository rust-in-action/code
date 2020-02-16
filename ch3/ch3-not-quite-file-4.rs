#[derive(Debug)] // <1>
struct File {
  name: String,
  data: Vec<u8>, // <2>
  state: &'static str,
}

type FileMode = &'static str;

const OPEN: FileMode = "open";
const CLOSED: FileMode = "closed";

fn open(f: &mut File) -> bool {
    f.state = OPEN;
    true // <3> let's assume for the moment that this always succeeds
}

fn close(f: &mut File) -> bool {
    f.state = CLOSED;
    true // <3>
}

fn read(f: &File) -> (usize, Vec<u8>) {
    (f.data.len(), f.data.clone())
}

fn main() {
  let mut f2 = File { 
    name: String::from("2.txt"), // <3>
    data: vec![],
    state: CLOSED,
  };

  let f2_name = &f2.name.clone(); // <5>

  open(&mut f2);
  let (f2_length, _) = read(&f2);
  close(&mut f2);
  
  println!("{:?}", f2);
  println!("{} is {} bytes long", f2_name, f2_length);
}