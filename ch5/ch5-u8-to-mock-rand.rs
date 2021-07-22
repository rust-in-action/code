fn mock_rand(n: u8) -> f32 {
    let base: u32 = 0b0_01111110_00000000000000000000000;
    let large_n =  (n as u32) << 15;
    let f32_bits = base | large_n;
    let m = f32::from_bits(f32_bits);
    2.0 * ( m - 0.5 )
}

fn main() {
    println!("max of input range: {:08b} -> {:?}", 0xff, mock_rand(0xff));
    println!("mid of input range: {:08b} -> {:?}", 0x7f, mock_rand(0x7f));
    println!("min of input range: {:08b} -> {:?}", 0x00, mock_rand(0x00));
}
