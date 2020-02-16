use std::collections::BTreeMap;

fn main() {
    let mut voc = BTreeMap::new();

    voc.insert(3_697_915, "Amsterdam");
    voc.insert(1_300_405, "Middelburg");
    voc.insert(  540_000, "Enkhuizen");
    voc.insert(  469_400, "Delft");
    voc.insert(  266_868, "Hoorn");
    voc.insert(  173_000, "Rotterdam");

    for (guilders,kamer) in &voc {
        println!("chamber {} invested {}", kamer, guilders); // will print in sorted order
    }

    print!("smaller chambers: ");
    for (_guilders,kamer) in voc.range(0..500_000) {
        print!("{} ", kamer); // will print in order
    }
    println!("");

}
