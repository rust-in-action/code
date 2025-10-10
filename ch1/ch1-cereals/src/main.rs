#[allow(dead_code)]
#[derive(Debug)] // <1>
enum Cereal {
    Barley,
    Millet,
    Rice,
    Rye,
    Spelt,
    Wheat,
}

fn main() {
    let grains: Vec<Cereal> = vec![Cereal::Rye]; // <3>
    drop(grains); // <5>
}
