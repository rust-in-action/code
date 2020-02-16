fn main() {
    let a: i32 = 40;
    let b: Box<i32> = Box::new(60);

    println!("{} + {} = {}", a, b, a + *b); 
}
