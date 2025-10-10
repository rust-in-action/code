fn add_with_lifetimes(i: &i32, j: &i32) -> i32 {
    *i + *j // <1>
}

pub fn main() {
    let a = 10;
    let b = 20;
    let res = add_with_lifetimes(&a, &b); // <2>

    println!("{}", res);
}
