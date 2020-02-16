#[macro_use]
extern crate serde_json;

fn main() {
    let capitals = json!({
      "Cook Islands": "Avarua",
      "Fiji": "Suva",
      "Kiribati": "South Tarawa",
      "Niue": "Alofi",
      "Tonga": "Nuku'alofa",
      "Tuvalu": "Funafuti"
    });

    println!("Capital of Tonga is: {}", capitals["Tonga"])
}