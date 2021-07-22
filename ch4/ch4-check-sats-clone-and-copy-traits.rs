#[derive(Debug,Clone,Copy)]   // <1>
struct CubeSat {
  id: u64,
}

#[derive(Debug,Clone,Copy)]   // <1>
enum StatusMessage {
  Ok,
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
  StatusMessage::Ok
}

fn main () {
  let sat_a = CubeSat { id: 0 };

  let a_status = check_status(sat_a.clone());   // <2>
  println!("a: {:?}", a_status.clone());        // <2>

  let a_status = check_status(sat_a);           // <3>
  println!("a: {:?}", a_status);                // <3>
}
