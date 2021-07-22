use std::collections::HashMap;

fn main() {
  let mut capitals = HashMap::new();             // <1>

  capitals.insert("Cook Islands", "Avarua");
  capitals.insert("Fiji", "Suva");
  capitals.insert("Kiribati", "South Tarawa");
  capitals.insert("Niue", "Alofi");
  capitals.insert("Tonga", "Nuku'alofa");
  capitals.insert("Tuvalu", "Funafuti");

  let tongan_capital = capitals["Tonga"];        // <2>

  println!("Capital of Tonga is: {}", tongan_capital);
}
