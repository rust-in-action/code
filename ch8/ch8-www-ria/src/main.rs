use std::io::{self, BufReader};
use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut buffer = String::with_capacity(1024);
    let mut stream = TcpStream::connect("www.rustinaction.com:80")?;
    stream.write(b"GET / HTTP/1.0\r\nHost: www.rustinaction.com\r\nAccept: */*\r\n\r\n")?;
    
    let mut webpage = BufReader::new(stream);
    webpage.
    loop {
        let n_bytes = webpage.read_line(&mut buffer);
        match n_bytes {
            Ok(n) if n == 0 => break,
            Ok(_) => {
                print!("{}", buffer); 
                buffer.clear();
            },
            //Err(ref err) if err.kind() == io::ErrorKind::UnexpectedEof => break,
            Err(err) => return Err(err),
        }
    }
    // for line in webpage.lines() {
    //     println!("{}", line.unwrap());
    // }
    println!();
    Ok(())
}
