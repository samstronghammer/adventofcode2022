use crate::util::advent;

pub fn run() {
  let contents = advent::parse_input(1);
  let split = contents.split("\n\n");
  let mut calories: Vec<i64> = Vec::new();
  for elf in split {
    let mut sum_calories = 0;
    for cal in elf.split("\n") {
      sum_calories += cal.parse::<i64>().unwrap();
    }
    calories.push(sum_calories);
  }
  calories.sort();
  println!("Part 1:");
  println!("{}", calories.last().unwrap());
  println!("Part 2:");
  println!("{}", calories.iter().rev().take(3).sum::<i64>());
}
