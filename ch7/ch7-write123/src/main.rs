use std::io::Cursor;
use byteorder::{LittleEndian};
use byteorder::{ReadBytesExt, WriteBytesExt};

fn write_numbers_to_file() -> (u32, i8, f64) {
    let mut w = vec![]; // slices implement Read and Write, and can thus act as mock files

    let one: u32   = 1;
    let two: i8    = 2;
    let three: f64 = 3.0;

    w.write_u32::<LittleEndian>(one).unwrap();
    println!("{:?}", &w);

    w.write_i8(two).unwrap(); // single byte methods don't take an endianness parameter
    println!("{:?}", &w);

    w.write_f64::<LittleEndian>(three).unwrap();
    println!("{:?}", &w);

    (one, two, three)
}

fn read_numbers_from_file() -> (u32, i8, f64) {
    let mut r = Cursor::new(vec![1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 8, 64]);
    let one_ = r.read_u32::<LittleEndian>().unwrap();
    let two_ = r.read_i8().unwrap();
    let three_ = r.read_f64::<LittleEndian>().unwrap();

    (one_, two_, three_)
}

fn main() {
    write_numbers_to_file();
    read_numbers_from_file();
}
