extern crate reqwest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let url = "http://www.rustinaction.com/";
  let mut response = reqwest::get(url)?;

  let content = response.text()?;
  print!("{}", content);

  Ok(())
}
