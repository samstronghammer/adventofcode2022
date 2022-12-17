use crate::util::{advent, pattern, point::Point};
use std::{collections::HashSet, hash::Hash, ops};

#[derive(Eq, PartialEq, Clone)]
struct Rock {
  points: HashSet<Point>,
}

impl Rock {
  pub fn new(rock_type_index: u8, bottom_left: Point) -> Rock {
    let mut points = HashSet::new();
    match rock_type_index {
      0 => {
        points.insert(bottom_left);
        points.insert(bottom_left + Point::RIGHT);
        points.insert(bottom_left + Point::RIGHT + Point::RIGHT);
        points.insert(bottom_left + Point::RIGHT + Point::RIGHT + Point::RIGHT);
      }
      1 => {
        points.insert(bottom_left + Point::RIGHT);
        points.insert(bottom_left + Point::UP);
        points.insert(bottom_left + Point::RIGHT + Point::UP);
        points.insert(bottom_left + Point::RIGHT + Point::RIGHT + Point::UP);
        points.insert(bottom_left + Point::RIGHT + Point::UP + Point::UP);
      }
      2 => {
        points.insert(bottom_left);
        points.insert(bottom_left + Point::RIGHT);
        points.insert(bottom_left + Point::RIGHT + Point::RIGHT);
        points.insert(bottom_left + Point::RIGHT + Point::RIGHT + Point::UP);
        points.insert(bottom_left + Point::RIGHT + Point::RIGHT + Point::UP + Point::UP);
      }
      3 => {
        points.insert(bottom_left);
        points.insert(bottom_left + Point::UP);
        points.insert(bottom_left + Point::UP + Point::UP);
        points.insert(bottom_left + Point::UP + Point::UP + Point::UP);
      }
      4 => {
        points.insert(bottom_left);
        points.insert(bottom_left + Point::RIGHT);
        points.insert(bottom_left + Point::UP);
        points.insert(bottom_left + Point::RIGHT + Point::UP);
      }
      _ => panic!("Invalid index: {}", rock_type_index),
    }
    return Rock { points: points };
  }
}

impl ops::Add<Point> for Rock {
  type Output = Rock;
  fn add(self, rhs: Point) -> Rock {
    return Rock {
      points: HashSet::from_iter(self.points.iter().map(|p| *p + rhs)),
    };
  }
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct RockIndexState(usize, u8);

#[derive(Clone)]

struct FallingRocks {
  gas_index: usize,
  rock_type_index: u8,
  fallen_rock: HashSet<Point>,
  gas: Vec<Point>,
}

impl Iterator for FallingRocks {
  type Item = (RockIndexState, i64);

  fn next(&mut self) -> Option<Self::Item> {
    let state = RockIndexState(self.gas_index, self.rock_type_index);
    let max_y = self.fallen_rock.iter().map(|p| p.y).max().unwrap_or(-1) + 1;
    let return_value = Some((state.clone(), i64::from(max_y)));
    let mut new_rock = Rock::new(self.rock_type_index, Point::new(2, max_y + 3));
    loop {
      // Gas
      let gas_vec = self.gas[self.gas_index];
      self.gas_index = (self.gas_index + 1) % self.gas.len();
      let gas_pos = new_rock.clone() + gas_vec;
      if gas_pos.points.intersection(&self.fallen_rock).count() == 0
        && gas_pos.points.iter().map(|p| p.x).all(|x| x <= 6 && x >= 0)
      {
        new_rock = gas_pos;
      }
      // Fall
      let fall_pos = new_rock.clone() + Point::DOWN;
      if fall_pos.points.intersection(&self.fallen_rock).count() == 0
        && fall_pos.points.iter().map(|p| p.y).all(|y| y >= 0)
      {
        new_rock = fall_pos;
      } else {
        for rock in new_rock.points {
          self.fallen_rock.insert(rock);
        }
        break;
      }
    }
    self.rock_type_index = (self.rock_type_index + 1) % 5;
    return return_value;
  }
}

pub fn run() {
  let contents = String::from(advent::parse_input(17));
  let gas: Vec<Point> = contents
    .chars()
    .map(|c| if c == '<' { Point::LEFT } else { Point::RIGHT })
    .collect();

  let mut p1_simulation = FallingRocks {
    gas_index: 0,
    rock_type_index: 0,
    fallen_rock: HashSet::new(),
    gas: gas.clone(),
  };

  let p2_simulation = p1_simulation.clone();

  p1_simulation.nth(2022);

  println!("Part 1:");
  println!(
    "{}",
    p1_simulation.fallen_rock.iter().map(|p| p.y).max().unwrap() + 1
  ); // + 1 because first layer is at 0;

  println!("Part 2:");
  println!(
    "{}",
    pattern::calc_big_pattern_index(p2_simulation, 1000000000000, 2022)
  );
}
