use std::collections::HashMap;

fn main() {
    let mut capitals = HashMap::new();

    capitals.insert("Cook Islands", "Avarua");
    capitals.insert("Fiji", "Suva");
    capitals.insert("Kiribati", "South Tarawa");
    capitals.insert("Niue", "Alofi");
    capitals.insert("Tonga", "Nuku'alofa");
    capitals.insert("Tuvalu", "Funafuti");

    println!("Capital of Tonga is: {}", capitals["Tonga"]);
}
