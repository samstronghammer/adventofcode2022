use crate::util::advent;

pub fn run() {
  let contents = advent::parse_input(2);
  let lines = contents.split("\n");
  let mut score: u32 = 0;
  let mut score2: u32 = 0;

  // For all of this, Rock is 0, Paper is 1, and Scissors is 2.
  // So, one value beats another if it is one greater (mod 3, of course).

  for line in lines.clone() {
    let elf_play: u32 = (line.chars().nth(0).unwrap() as u32) - 65;
    let my_play: u32 = (line.chars().nth(2).unwrap() as u32) - 88;
    let win = my_play == ((elf_play + 1) % 3);
    score += my_play + 1;
    if win {
      score += 6;
    }
    if my_play == elf_play {
      score += 3;
    }
  }

  for line in lines {
    let elf_play: u32 = (line.chars().nth(0).unwrap() as u32) - 65;
    let result: u32 = (line.chars().nth(2).unwrap() as u32) - 88;
    let my_play: u32 = (elf_play + result + 2) % 3; // Convert backward from what the result should be to the correct play
    let win = my_play == ((elf_play + 1) % 3);
    score2 += my_play + 1;
    if win {
      score2 += 6;
    }
    if my_play == elf_play {
      score2 += 3;
    }
  }

  println!("Part 1:");
  println!("{}", score);
  println!("Part 2:");
  println!("{}", score2);
}
