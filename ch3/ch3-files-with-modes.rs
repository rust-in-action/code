#[derive(Debug)]
pub enum FileOpenMode {
  Read,
  Write,
  Append,
  Truncate,
}

#[derive(Debug)]
pub enum FileHandle {
  Handle(usize),
  None,
}

#[derive(Debug)]
pub enum FileState {
  PendingCreation,
  Created(FileOpenMode),
  Opened(FileOpenMode),
  Error(String),
  Closed,
  Deleted,
}

#[derive(Debug)]
pub struct File {
  name: String,
  data: Vec<u8>,
  state: FileState,
  handle: FileHandle,
}

impl File {
  pub fn new(name: &str) -> File {
    File {
      name: String::from(name),
      data: Vec::new(),
      state: FileState::PendingCreation, // <1>
      handle: FileHandle::None, // <1>
    }
  }
  
  pub fn from_options(name: &str, state: FileState, handle: FileHandle) -> File {
    File {
      name: String::from(name),
      data: Vec::new(),
      state: state,
      handle: handle,
    }
  }
}

fn main() {
  let f1 = File::new("f1.txt");
  let f2 = File::from_options("f2.txt", 
                   FileState::Opened(FileOpenMode::Read), 
                   FileHandle::Handle(123)
               );
  let f3 = File::from_options("f3.txt", 
                   FileState::Opened(FileOpenMode::Write), 
                   FileHandle::None
               );
  
  let mut files = [f1, f2, f3];
  
  for f in &files {
    println!("{:?}", f);
  }
  
  // uh oh, disk failure
  for ref mut f in &mut files {
    f.state = FileState::Error(String::from("disk read failure"));
    println!("{:?}", f);
  }
}