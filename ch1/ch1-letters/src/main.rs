fn main() {
    let letters = vec![
        // <1>
        "a", "b", "c",
    ];

    for letter in &letters {
        println!("{}", letter);
    }
}
