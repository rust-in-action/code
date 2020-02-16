// fn mock_rand(n: u8) -> f32 {
//     (n as f32) / 255.
// }

fn mock_rand(n: u8) -> f32 {
    let base: u32 = 0b0_01111110_00000000000000000000000; // <1> Underscores mark the sign/mantissa/exponent boundaries
    let large_n =  (n as u32) << 15; // <2> Align the input byte `n` to 32 bits, then increase its value  by shifting its bits 15 places to the left 
    let f32_bits = base | large_n; // <3> Take a bitwise OR, merging the base with the input byte
    let m = f32::from_bits(f32_bits);// <4> Interpret `f32_bits` (which is type `u32`) as an `f32`
    2.0 * ( m - 0.5 ) // <5> Normalize the output range
}

fn main() {
    println!("max of input range: {:08b} -> {}", 0xff, mock_rand(0xff));
    println!("mid of input range: {:08b} -> {}", 0x77, mock_rand(0x77));
    println!("min of input range: {:08b} -> {}", 0x00, mock_rand(0x00));    
}