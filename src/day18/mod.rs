use crate::util::{advent, point::Point3};
use pathfinding::prelude::dijkstra_all;
use std::collections::{HashMap, HashSet};

fn in_range(point: Point3, min: Point3, max: Point3) -> bool {
  return min.x <= point.x
    && point.x <= max.x
    && min.y <= point.y
    && point.y <= max.y
    && min.z <= point.z
    && point.z <= max.z;
}

pub fn run() {
  let contents = advent::parse_input(18);
  let lines: Vec<&str> = contents.split("\n").collect();
  let mut lava: HashSet<Point3> = HashSet::new();

  for line in lines {
    let toks: Vec<&str> = line.split(',').collect();
    let new_point = Point3::new(
      i32::from_str_radix(toks[0], 10).unwrap(),
      i32::from_str_radix(toks[1], 10).unwrap(),
      i32::from_str_radix(toks[2], 10).unwrap(),
    );
    lava.insert(new_point);
  }

  let min_corner = lava
    .iter()
    .fold(Point3::new(i32::MAX, i32::MAX, i32::MAX), |acc, p| {
      Point3::new(acc.x.min(p.x - 1), acc.y.min(p.y - 1), acc.z.min(p.z - 1))
    });

  let max_corner = lava
    .iter()
    .fold(Point3::new(i32::MIN, i32::MIN, i32::MIN), |acc, p| {
      Point3::new(acc.x.max(p.x + 1), acc.y.max(p.y + 1), acc.z.max(p.z + 1))
    });

  let successors = |point: &Point3| -> Vec<(Point3, i32)> {
    return point
      .adj6()
      .iter()
      .filter(|p| !lava.contains(p) && in_range(**p, min_corner, max_corner))
      .map(|p| {
        return (*p, 1);
      })
      .collect();
  };

  let result: HashMap<Point3, (Point3, i32)> = dijkstra_all(&min_corner, successors);

  let mut count_1 = 0;
  let mut count_2 = 0;
  for point in lava.iter() {
    for adj in point.adj6() {
      if !lava.contains(&adj) {
        count_1 += 1;
        if result.contains_key(&&adj) {
          count_2 += 1;
        }
      }
    }
  }

  println!("Part 1:");
  println!("{}", count_1);
  println!("Part 2:");
  println!("{}", count_2);
}
