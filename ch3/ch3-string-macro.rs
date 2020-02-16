macro_rules! string {
    ($x:expr) => (
        String::from($x);
    )
}

fn main() {
    let s = string!("hello");
    println!("{}", s);
}