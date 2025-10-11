pub fn main() {
    let a: f32 = 42.42;
    let frankentype: u32 = f32::to_bits(a);

    println!("{}", frankentype); // <2>
    println!("{:032b}", frankentype); // <3>

    let b: f32 = f32::from_bits(frankentype);
    println!("{}", b);
    assert_eq!(a, b); // <4>
}
