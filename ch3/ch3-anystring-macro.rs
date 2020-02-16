macro_rules! string {
    ($x:expr) => ( // <1>
        String::from(stringify!($x)); // <2>
    )
}

fn main() {
    let s = string!(hello there);
    println!("{}", s);
}