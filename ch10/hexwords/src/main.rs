use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() -> std::io::Result<()> {
    let f = File::open("/usr/share/dict/words")?;
    let reader = BufReader::new(f);

    'lines: for line in reader.lines() {
        let word = line.unwrap();
        for byte in word.bytes() {
            match byte {
                b'A' | b'B' | b'C' | b'D' | b'E' | b'F' |
                b'a' | b'b' | b'c' | b'd' | b'e' | b'f' => continue,
                _ => continue 'lines,
            }
        }

        if word.len() > 2 {
            println!("{}", word);
        }
    };

    Ok(())
}
