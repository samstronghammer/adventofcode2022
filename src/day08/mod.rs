use crate::util::{advent, point::Point};
use std::collections::HashMap;

// I think these functions could be merged, but dealing with the short circuit cases would be annoying.

fn is_visible(point: Point, trees: &HashMap<Point, u8>) -> bool {
  let directions: Vec<Point> = [Point::DOWN, Point::RIGHT, Point::LEFT, Point::UP].to_vec();
  let test_height = *trees.get(&point).unwrap();
  for direction in directions {
    let mut curr = point + direction;
    loop {
      if !trees.contains_key(&curr) {
        return true;
      }
      let height = trees[&curr];
      if height >= test_height {
        break;
      }
      curr = curr + direction;
    }
  }
  return false;
}

fn scenic_score(point: Point, trees: &HashMap<Point, u8>) -> u32 {
  let directions: Vec<Point> = [Point::DOWN, Point::RIGHT, Point::LEFT, Point::UP].to_vec();
  let test_height = *trees.get(&point).unwrap();
  let mut score = 1;
  for direction in directions {
    let mut curr = point + direction;
    let mut count = 0;
    loop {
      if !trees.contains_key(&curr) {
        break;
      }
      count += 1;
      let height = trees[&curr];
      if height >= test_height {
        break;
      }
      curr = curr + direction;
    }
    score *= count;
  }
  return score;
}

pub fn run() {
  let contents = advent::parse_input(8);
  let lines: Vec<&str> = contents.split("\n").collect();
  let mut trees: HashMap<Point, u8> = HashMap::new();
  let num_lines = lines.len();
  let num_columns = lines.get(0).unwrap().len();

  for y in 0..num_lines {
    let line = lines.get(num_lines - y - 1).unwrap();
    for x in 0..num_columns {
      let c = line.chars().nth(x).unwrap();
      let height = u8::from_str_radix(&c.to_string(), 10).unwrap() + 1; //add 1 because 0 height is inconvenient for the next loops
      trees.insert(
        Point::new(x.try_into().unwrap(), y.try_into().unwrap()),
        height,
      );
    }
  }

  let num_visible = trees.keys().filter(|k| is_visible(**k, &trees)).count();
  let best_score = trees
    .keys()
    .map(|k| scenic_score(*k, &trees))
    .max()
    .unwrap();

  println!("Part 1:");
  println!("{}", num_visible);
  println!("Part 2:");
  println!("{}", best_score);
}
