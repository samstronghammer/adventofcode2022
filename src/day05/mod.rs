use crate::util::advent;
use std::collections::HashMap;

fn move_crates_p2(
  num: u32,
  from: char,
  to: char,
  stacks: &mut HashMap<char, Vec<char>>,
  part_1: bool,
) {
  let crate_vec = stacks.get_mut(&from).unwrap();
  let mut new_vec = crate_vec.split_off(crate_vec.len() - usize::try_from(num).unwrap());
  if part_1 {
    new_vec.reverse();
  }
  stacks.get_mut(&to).unwrap().append(&mut new_vec);
}

fn top_crate_string(stacks: &HashMap<char, Vec<char>>) -> String {
  let mut return_string = String::from("");
  for i in 0..stacks.len() {
    return_string.push(
      *(stacks
        .get(&char::from_digit((i + 1).try_into().unwrap(), 10).unwrap())
        .unwrap()
        .last()
        .unwrap()),
    )
  }
  return return_string;
}

pub fn run() {
  let contents = advent::parse_input(5);
  let lines: Vec<&str> = contents.split("\n").collect();
  let mut crates1: HashMap<char, Vec<char>> = HashMap::new();
  let mut crates2: HashMap<char, Vec<char>> = HashMap::new();

  for i in 0..lines.len() {
    let line = lines[i];
    if line.starts_with(" 1") {
      for j in 0..line.len() {
        let stack_char = line.chars().nth(j).unwrap();
        if stack_char == ' ' {
          continue;
        }
        let mut new_stack: Vec<char> = Vec::new();
        for k in 1..(i + 1) {
          let crate_char = lines[i - k].chars().nth(j).unwrap();
          if crate_char == ' ' {
            break;
          }
          new_stack.push(crate_char);
        }
        crates1.insert(stack_char, new_stack.clone());
        crates2.insert(stack_char, new_stack.clone());
      }
    } else if line.starts_with("move") {
      let toks: Vec<&str> = line.split(' ').collect();
      let num: u32 = u32::from_str_radix(toks[1], 10).unwrap();
      let from: char = toks[3].chars().nth(0).unwrap();
      let to: char = toks[5].chars().nth(0).unwrap();
      move_crates_p2(num, from, to, &mut crates1, true);
      move_crates_p2(num, from, to, &mut crates2, false);
    }
  }

  println!("Part 1:");
  println!("{}", top_crate_string(&crates1));
  println!("Part 2:");
  println!("{}", top_crate_string(&crates2));
}
