use regex::Regex; // <1>

pub fn main() {
    let re = Regex::new("picture").unwrap(); // <2>

    let quote = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

    for line in quote.lines() {
        if let Some(_) = re.find(line) {
            println!("{}", line);
        }
    }
}
