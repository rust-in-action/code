fn factorial_usize(n: usize) -> usize {
  match n {
    0 => 0,
    1 => 1,
    _ => n + factorial_usize(n-1),
  }
}

fn factorial_u64(n: u64) -> u64 {
  match n {
    0 => 0,
    1 => 1,
    _ => n + factorial_u64(n-1),
  }
}

fn factorial_u32(n: u32) -> u32 {
  match n {
    0 => 0,
    1 => 1,
    _ => n + factorial_u32(n-1),
  }
}

fn factorial_u16(n: u16) -> u16 {
  match n {
    0 => 0,
    1 => 1,
    _ => n + factorial_u16(n-1),
  }
}

fn factorial_u8(n: u8) -> u8 {
  match n {
    0 => 0,
    1 => 1,
    _ => n + factorial_u8(n-1),
  }
}


fn main() {
    let n = 10;

    println!("{}", factorial_usize(n as usize));
    println!("{}", factorial_u64(n as u64));
    println!("{}", factorial_u32(n as u32));
    println!("{}", factorial_u16(n as u16));
    println!("{}", factorial_u8(n as u8));
}
