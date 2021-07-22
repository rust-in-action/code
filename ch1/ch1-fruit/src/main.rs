fn main() {
  let fruit = vec!['🥝', '🍌', '🍇'];

  let buffer_overflow = fruit[4];    // <1>

  assert_eq!(buffer_overflow, '🍉')  // <2>
}
