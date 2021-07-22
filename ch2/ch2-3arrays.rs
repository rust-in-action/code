fn main() {
  let one             = [1, 2, 3];
  let two: [u8; 3]    = [1, 2, 3];
  let blank1          = [0; 3];
  let blank2: [u8; 3] = [0; 3];

  let arrays = [one, two, blank1, blank2];

  for a in &arrays {
    print!("{:?}: ", a);
    for n in a.iter() {
      print!("\t{} + 10 = {}", n, n+10);
    }

    let mut sum = 0;
    for i in 0..a.len() {
      sum += a[i];
    }
    println!("\t(Σ{:?} = {})", a, sum);
  }
}