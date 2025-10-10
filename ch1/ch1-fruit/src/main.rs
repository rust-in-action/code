fn main() {
    let fruit = vec!['🥝', '🍌', '🍇'];

    let buffer_overflow = fruit[0]; // <1>

    assert_eq!(buffer_overflow, '🥝') // <2>
}
