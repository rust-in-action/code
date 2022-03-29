fn main() {
  let needle = 42;
  let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862]; // <1>
    
  for reference in haystack.iter() { // <2>
    let item = *reference; // <3>
    if item == needle {
      println!("{}", item);
    }

    // if reference == &needle { // <4>
    //   println!("{}", reference);
    // }
  }
}
