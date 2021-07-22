#[derive(Debug)]    // <1>
enum Cereal {       // <2>
    Barley, Millet, Rice,
    Rye, Spelt, Wheat,
}

fn main() {
    let mut grains: Vec<Cereal> = vec![];   // <3>
    grains.push(Cereal::Rye);               // <4>
    drop(grains);                           // <5>

    println!("{:?}", grains);               // <6>
}
