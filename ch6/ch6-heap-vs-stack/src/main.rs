use std::mem::drop; // <1> Bring manual `drop()` into local scope

fn main() {
    let a = Box::new(1);
    let b = Box::new(1);
    let c = Box::new(1);

    let result1 = *a + *b + *c; // <2> Use the variables so that they're not optimized away by the compiler. The unary `pass:[*]` operator is called the dereference operator. It returns the value within the box.

    drop(a); // <3> The memory holding `a` is now available

    let d = Box::new(1);
    let result2 = *b + *c + *d;

    println!("{} {}", result1, result2);
}
