use std::io;

fn main() {
  let mut buffer = Vec<f64>;

  loop {
    let mut input = String::new();
    let reading: f64;

    match io::stdin().read_line(&mut input) {
        Ok(_n_bytes) => {
            reading = input.parse();
        },
        Err(err) => {
            println!("error: {:#?}", err);
            continue
        },
    }


  }
}
