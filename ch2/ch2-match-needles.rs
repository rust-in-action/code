fn main() {
  let needle = 42;                 // <1>
  let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

  for item in &haystack {
    let result = match item {      // <2>
      42 | 132 => "hit!",          // <3>
      _ => "miss",                 // <4>
    };

    if result == "hit!" {
      println!("{}: {}", item, result);
    }
  }
}