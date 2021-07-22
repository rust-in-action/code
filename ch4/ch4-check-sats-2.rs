#[derive(Debug)]   // <1>
struct CubeSat {
  id: u64,
}

#[derive(Debug)]
enum StatusMessage {
  Ok,
}

fn check_status(
  sat_id: CubeSat
) -> StatusMessage {   // <2>
  StatusMessage::Ok
}

fn main() {
  let sat_a = CubeSat { id: 0 };   // <3>
  let sat_b = CubeSat { id: 1 };   // <3>
  let sat_c = CubeSat { id: 2 };   // <3>

  let a_status = check_status(sat_a);
  let b_status = check_status(sat_b);
  let c_status = check_status(sat_c);
  println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

  // "waiting" ...
  let a_status = check_status(sat_a);
  let b_status = check_status(sat_b);
  let c_status = check_status(sat_c);
  println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);
}
