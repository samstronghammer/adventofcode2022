use crate::util::{advent, point::Point};
use std::collections::HashSet;

fn string_to_point(s: &str) -> Point {
  let toks: Vec<&str> = s.split(",").collect();
  return Point::new(
    i32::from_str_radix(toks[0], 10).unwrap(),
    i32::from_str_radix(toks[1], 10).unwrap(),
  );
}

fn simulate_tic(
  rock: &HashSet<Point>,
  sand: &mut HashSet<Point>,
  curr: Point,
  start: Point,
  max_y: i32,
) -> Point {
  // Up is down, kind of annoying. Just how the y axis is set up.
  let possibilities = [
    curr + Point::UP,
    curr + Point::UP + Point::LEFT,
    curr + Point::UP + Point::RIGHT,
  ]
  .to_vec();
  let next = possibilities
    .iter()
    .find(|p| !rock.contains(p) && !sand.contains(p) && p.y < max_y + 2);
  if next.is_none() {
    sand.insert(curr);
    return start;
  } else {
    return *next.unwrap();
  }
}

pub fn run() {
  let contents = advent::parse_input(14);
  let lines: Vec<&str> = contents.split("\n").collect();
  let mut rock: HashSet<Point> = HashSet::new();
  let mut sand: HashSet<Point> = HashSet::new();
  let sand_start: Point = Point::new(500, 0);

  for line in lines {
    let toks: Vec<&str> = line.split(" -> ").collect();
    let mut curr = string_to_point(toks[0]);
    rock.insert(curr);
    for i in 1..toks.len() {
      let next_point = string_to_point(toks[i]);
      let delta = next_point - curr;
      let v = Point::new(delta.x.signum(), delta.y.signum());
      while curr != next_point {
        curr = curr + v;
        rock.insert(curr);
      }
    }
  }

  let max_y = rock.iter().map(|r| r.y).max().unwrap();

  let mut curr = sand_start;

  while curr.y <= max_y {
    curr = simulate_tic(&rock, &mut sand, curr, sand_start, max_y);
  }
  let part1 = sand.len();

  curr = sand_start;
  while !sand.contains(&sand_start) {
    curr = simulate_tic(&rock, &mut sand, curr, sand_start, max_y);
  }

  println!("Part 1:");
  println!("{}", part1);
  println!("Part 2:");
  println!("{}", sand.len());
}
