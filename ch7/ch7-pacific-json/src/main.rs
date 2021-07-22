#[macro_use]                          // <1>
extern crate serde_json;              // <1>

fn main() {
  let capitals = json!({              // <2>
    "Cook Islands": "Avarua",
    "Fiji": "Suva",
    "Kiribati": "South Tarawa",
    "Niue": "Alofi",
    "Tonga": "Nuku'alofa",
    "Tuvalu": "Funafuti"
  });

  println!("Capital of Tonga is: {}", capitals["Tonga"])
}
