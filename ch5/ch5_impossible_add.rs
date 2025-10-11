#[allow(arithmetic_overflow)]

pub fn main() {
    let (a, b) = (200_u8, 200);
    let c = a.checked_add(b);
    println!("200 + 200 = {:?}", c);
}
