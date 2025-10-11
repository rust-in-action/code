mod ch5_bit_patterns;
mod ch5_endianness;
mod ch5_f32_as_u32;
mod ch5_impossible_add;
mod ch5_int_vs_int;
mod ch5_to_oblivion;
mod ch5_u8_to_mock_rand;
mod ch5_visualizing_f32;

pub fn main() {
    println!("-- ch5_bit_patterns --");
    ch5_bit_patterns::main();

    println!("-- ch5_endianness --");
    ch5_endianness::main();

    println!("-- ch5_f32_as_u32 --");
    ch5_f32_as_u32::main();

    println!("-- ch5_impossible_add --");
    ch5_impossible_add::main();

    println!("-- ch5_int_vs_int --");
    ch5_int_vs_int::main();

    println!("-- ch5_to_oblivion --");
    ch5_to_oblivion::main();

    println!("-- ch5_u8_to_mock_rand --");
    ch5_u8_to_mock_rand::main();

    println!("-- ch5_visualizing_f32 --");
    ch5_visualizing_f32::main();
}
