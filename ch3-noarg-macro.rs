macro_rules! perma_string { // <1>
    () => { // <2>
        String::from("hello")
    }
}

fn main() {
    let s = perma_string!();
    println!("{}", s);
}