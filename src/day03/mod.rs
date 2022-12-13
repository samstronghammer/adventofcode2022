use crate::util::advent;
use std::collections::HashSet;

fn find_common(s1: &str, s2: &str, s3: &str) -> char {
  let set1: HashSet<char> = HashSet::from_iter(s1.chars());
  let set2: HashSet<char> = HashSet::from_iter(s2.chars());
  for c in s3.chars() {
    if set1.contains(&c) && set2.contains(&c) {
      return c;
    }
  }
  panic!("Common character not found")
}

// Convert a character to it's priority value. Subtracts ascii offset.
fn char_to_priority(c: char) -> u32 {
  let c_num = c as u32;
  if c_num > 90 {
    return c_num - 96;
  } else {
    return c_num - 64 + 26;
  }
}

pub fn run() {
  let contents = advent::parse_input(3);

  let lines: Vec<&str> = contents.split("\n").collect();
  let num_lines = lines.len();
  let mut sum: u32 = 0;

  for i in 0..num_lines {
    let line = lines[i];
    let (part1, part2) = line.split_at(line.len() / 2);
    let c = find_common(part1, part2, part2);
    sum += char_to_priority(c);
  }

  let mut sum2: u32 = 0;

  for i in 0..(num_lines / 3) {
    let c = find_common(lines[i * 3], lines[i * 3 + 1], lines[i * 3 + 2]);
    sum2 += char_to_priority(c);
  }

  println!("Part 1:");
  println!("{}", sum);
  println!("Part 2:");
  println!("{}", sum2);
}
