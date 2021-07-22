use regex::Regex;    // <1>

fn main() {
  let re = Regex::new("picture").unwrap();    // <2>

  let quote = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

  for line in quote.lines() {
    let contains_substring = re.find(line);
    match contains_substring {    // <3>

        Some(_) => println!("{}", line),    // <4>
        None => (),    // <5>
    }
  }
}
