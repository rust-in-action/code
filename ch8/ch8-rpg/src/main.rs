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
    let probability_of_success = self.competency();
    let spell_is_successful = rand::thread_rng()
      .gen_bool(probability_of_success);      // <1>

    print!("{:?} mutters incoherently. ", self);
    if spell_is_successful {
      println!("The {:?} glows brightly.", thing);
    } else {
      println!("The {:?} fizzes, \
             then turns into a worthless trinket.", thing);
      *thing = Thing::Trinket {};
    }
  }
}

impl Enchanter for Dwarf {
  fn competency(&self) -> f64 {
    0.5                                       // <2>
  }
}
impl Enchanter for Elf {
  fn competency(&self) -> f64 {
    0.95                                      // <3>
  }
}
impl Enchanter for Human {
  fn competency(&self) -> f64 {
    0.8                                       // <4>
  }
}

fn main() {
  let mut it = Thing::Sword;

  let d = Dwarf {};
  let e = Elf {};
  let h = Human {};

  let party: Vec<&dyn Enchanter> = vec![&d, &h, &e];// <5>
  let spellcaster = party.choose(&mut rand::thread_rng()).unwrap();

  spellcaster.enchant(&mut it);
}
