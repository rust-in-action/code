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

impl File {
  fn new(name: &str) -> File {
    File {
      name: String::from(name),
      data: Vec::new(),
      state: FileState::Closed,
    }
  }

  fn read(
    self: &File,
    save_to: &mut Vec<u8>,
  ) -> Result<usize, String> {
    if self.state != FileState::Open {
      return Err(String::from("File must be open for reading"));
    }
    let mut tmp = self.data.clone();
    let read_length = tmp.len();
    save_to.reserve(read_length);
    save_to.append(&mut tmp);
    Ok(read_length)
  }
}

fn open(mut f: File) -> Result<File, String> {
  f.state = FileState::Open;
  Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
  f.state = FileState::Closed;
  Ok(f)
}

fn main() {
  let mut f5 = File::new("5.txt");

  let mut buffer: Vec<u8> = vec![];

  if f5.read(&mut buffer).is_err() {
    println!("Error checking is working");
  }

  f5 = open(f5).unwrap();
  let f5_length = f5.read(&mut buffer).unwrap();
  f5 = close(f5).unwrap();

  let text = String::from_utf8_lossy(&buffer);

  println!("{:?}", f5);
  println!("{} is {} bytes long", &f5.name, f5_length);
  println!("{}", text);
}
