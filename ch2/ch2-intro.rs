fn main() {
  let start_at = 3;
  let stop_at = 10;
  let mut items = vec![];

  for (i,x) in (start_at..stop_at).enumerate() {
    let y = i as i64 * x;
    items.push(y);
  }

  println!("{:?}", items);

//let multiples_of_10 = items.iter().filter(|&y| y % 10 == 0).map(|&y| y.clone()).collect::<Vec<_>>();
//let multiples_of_10: Vec<_> = items.iter().filter(|&y| y % 10 == 0).map(|&y| y.clone()).collect();
  let multiples_of_10: Vec<_> = items.into_iter().filter(|y| y % 10 == 0).collect();
  println!("{:?}", multiples_of_10);

}
