extern crate image;

use std::fs::File;
use std::io::prelude::*;
use std::env;
use image::*;

const USAGE: &'static str = "usage: fview INPUT_FILE OUTPUT_FILE";
const WIDTH: u32 = 512;

fn main() {
    let arg1 = env::args().nth(1);
    let input_filename = arg1.expect(USAGE);

    let arg2 = env::args().nth(2);
    let output_filename = arg2.expect(USAGE);

    let mut fin = File::open(&input_filename).expect("Unable to open file");
    let meta = fin.metadata().expect("Unable to read INPUT_FILE's metadata");
    let len = meta.len() as u32;
    let height = len / WIDTH;

    let mut bytes: Vec<u8> = Vec::with_capacity(len as usize);
    fin.read_to_end(&mut bytes).expect("unable to read data");

    let mut img = RgbImage::new(WIDTH, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let i = x + (y * WIDTH);
        let color: Color = match bytes[i as usize] {
            0x00                                => Rgb{data: [0x00, 0x00, 0x00]}, // NULL bytes
            9 ..= 13 | 32                       => Rgb{data: [0xff, 0xff, 0xff]}, // white space
            1 ..= 8 | 13 ..= 31                 => Rgb{data: [0x0, 0xff, 0x07]}, // control characters
            43 ..= 47 | 58 ..= 64 |  91 ..= 96  => Rgb{data: [0x32, 0x12, 0x61]}, // punctuation
            48 ..= 57                           => Rgb{data: [0xec, 0x1f, 0x17]}, // 0-9
            65 ..= 90                           => Rgb{data: [0x67, 0x22, 0x22]}, // A-Z
            97 ..= 122                          => Rgb{data: [0xf8, 0xee, 0x22]}, // a-z
            255                                 => Rgb{data: [0x65, 0x10, 0x17]},
            _                                   => Rgb{data: [0x66, 0xe7, 0xf2]}, // non-ASCII bytes
        };
        *pixel = color;
    }
    let ref mut f = File::create(output_filename).expect("Unable create OUTPUT_FILE");
    image::ImageRgb8(img).save(f, image::PNG).expect("Unable to save image to OUTPUT_FILE");
}
