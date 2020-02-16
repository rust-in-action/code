use rand;
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Debug)]
struct Dwarf {}

#[derive(Debug)]
struct Elf {}

#[derive(Debug)]
struct Human {}

#[derive(Debug)]
enum Thing {
  Sword,
  Trinket,
}

trait Enchanter: std::fmt::Debug {
  fn competency(&self) -> f64;

  fn enchant(&self, thing: &mut Thing) {
    print!("{:?} mutters incoherently. ", self);
    if rand::thread_rng().gen_bool(self.competency()) {
      println!("The {:?} glows brightly.", thing);
      return;
    }
    println!("The {:?} fizzes, then turns into a worthless trinket.", thing);
    *thing = Thing::Trinket {};
  }
}

impl Enchanter for Dwarf {
  fn competency(&self) -> f64 {
    0.5
  }
}
impl Enchanter for Elf {
  fn competency(&self) -> f64 {
    0.95
  }
}
impl Enchanter for Human {
  fn competency(&self) -> f64 {
    0.8
  }
}

fn main() {
  let mut it = Thing::Sword;

  let d = Dwarf {};
  let e = Elf {};
  let h = Human {};

  let party: Vec<&dyn Enchanter> = vec![&d, &h, &e];
  let spellcaster = party.choose(&mut rand::thread_rng()).unwrap();
  spellcaster.enchant(&mut it)
}
