use std::collections::{HashMap, HashSet};

fn main() {

  let input_text = "does this work
  i dont know
  how rust works";

  let mut character_counts = HashMap::new();

  let mut n_lines = 0u32;

  for l in input_text.lines() {
    n_lines = n_lines + 1;

    let mut chars_for_line = HashSet::new();

    for c in l.chars() {
      if chars_for_line.contains(&c) {
        continue
      }
      let c_count = character_counts.entry(c).or_insert(0u32);
      *c_count += 1;
      chars_for_line.insert(c);
    }
  }

  for (c, c_count) in &character_counts {
    if *c_count == n_lines {
      println!("{}", c);
    }
  }
}
