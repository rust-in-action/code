extern crate reqwest;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut response = reqwest::get("http://www.rustinaction.com/")?;

    let content = response.text()?;

    for line in content.lines() {
        println!("{}", line.strip());
    }

    Ok(())
}
