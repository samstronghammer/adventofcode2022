use crate::util::advent;
use std::collections::HashMap;

fn find_marker(data: String, count: usize) -> usize {
  let mut counts: HashMap<char, u8> = HashMap::new();
  for i in 0..data.len() {
    let new_char = data.chars().nth(i).unwrap();
    if counts.contains_key(&new_char) {
      counts.insert(new_char, counts.get(&new_char).unwrap() + 1);
    } else {
      counts.insert(new_char, 1);
    }
    if i >= count {
      let old_char = data.chars().nth(i - count).unwrap();
      counts.insert(old_char, counts.get(&old_char).unwrap() - 1);
    }
    if i >= count - 1 {
      if counts.values().all(|v| *v <= 1) {
        return i + 1;
      }
    }
  }
  panic!("No valid marker found");
}

pub fn run() {
  let contents = advent::parse_input(6);
  println!("Part 1:");
  println!("{}", find_marker(contents.clone(), 4));
  println!("Part 2:");
  println!("{}", find_marker(contents, 14));
}
