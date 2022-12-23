use crate::util::{advent, point::Point};
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
enum Dir {
  NW,
  N,
  NE,
  W,
  E,
  SW,
  S,
  SE,
}

fn dir_to_point(dir: &Dir) -> Point {
  match dir {
    Dir::NW => Point::new(-1, -1),
    Dir::N => Point::new(0, -1),
    Dir::NE => Point::new(1, -1),
    Dir::W => Point::new(-1, 0),
    Dir::E => Point::new(1, 0),
    Dir::SW => Point::new(-1, 1),
    Dir::S => Point::new(0, 1),
    Dir::SE => Point::new(1, 1),
  }
}

fn elves_to_empty_ground(elves: &HashSet<Point>) -> i32 {
  let min_x = elves.iter().map(|p| p.x).min().unwrap();
  let max_x = elves.iter().map(|p| p.x).max().unwrap();
  let min_y = elves.iter().map(|p| p.y).min().unwrap();
  let max_y = elves.iter().map(|p| p.y).max().unwrap();
  return (max_x - min_x + 1) * (max_y - min_y + 1) - i32::try_from(elves.len()).unwrap();
}

fn elf_tic(elves: &HashSet<Point>, tic_number: usize) -> (HashSet<Point>, bool) {
  let considerations = [
    [Dir::N, Dir::NE, Dir::NW].to_vec(),
    [Dir::S, Dir::SE, Dir::SW].to_vec(),
    [Dir::W, Dir::NW, Dir::SW].to_vec(),
    [Dir::E, Dir::NE, Dir::SE].to_vec(),
  ]
  .to_vec();

  let mut proposals: HashMap<Point, Point> = HashMap::new(); // elf -> proposal
  let mut num_proposals: HashMap<Point, u8> = HashMap::new(); // proposal -> num
  for elf in elves.iter() {
    let mut proposal = *elf;
    if !elf.adj8().iter().all(|p| !elves.contains(p)) {
      for j in 0..4 {
        let index = (tic_number + j) % 4;
        let consideration = &considerations[index];
        if consideration
          .iter()
          .all(|dir| !elves.contains(&(*elf + dir_to_point(dir))))
        {
          // Consideration is correct.
          proposal = dir_to_point(&consideration[0]) + *elf;
          break;
        }
      }
    }
    proposals.insert(*elf, proposal);
    if !num_proposals.contains_key(&proposal) {
      num_proposals.insert(proposal, 0);
    }
    num_proposals.insert(proposal, num_proposals[&proposal] + 1);
  }
  let mut new_elves: HashSet<Point> = HashSet::new();
  let mut no_moves = true;
  for elf in elves.iter() {
    let proposal = proposals[elf];
    let num_collisions = num_proposals[&proposal];
    if num_collisions == 0 {
      panic!("0 elves??");
    } else if num_collisions == 1 {
      if proposal != *elf {
        no_moves = false;
      }
      new_elves.insert(proposal);
    } else {
      new_elves.insert(*elf);
    }
  }

  return (new_elves, no_moves);
}

pub fn run() {
  let contents = advent::parse_input(23);
  let lines: Vec<&str> = contents.split("\n").collect();
  let mut elves: HashSet<Point> = HashSet::new();

  for y in 0..lines.len() {
    let line = lines[y];
    for x in 0..line.len() {
      let c = line.chars().nth(x).unwrap();
      if c == '#' {
        elves.insert(Point::new(x.try_into().unwrap(), y.try_into().unwrap()));
      }
    }
  }

  let mut p1 = 0;
  let mut p2 = 0;
  let mut i = 0;

  loop {
    if i == 10 {
      p1 = elves_to_empty_ground(&elves);
    }
    let (new_elves, no_moves) = elf_tic(&elves, i);
    elves = new_elves;
    if no_moves {
      p2 = i + 1;
      break;
    }
    i += 1;
  }

  println!("Part 1:");
  println!("{}", p1);
  println!("Part 2:");
  println!("{}", p2);
}
