fn main() {
  let mut letters = vec![            // <1>
      "a", "b", "c"
  ];

  for letter in letters {
      println!("{}", letter);
      letters.push(letter.clone());  // <2>
  }
}
