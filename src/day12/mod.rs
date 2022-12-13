use crate::util::advent;
use crate::util::point::Point;
use pathfinding::prelude::dijkstra_all;
use std::collections::HashMap;

pub fn run() {
  let contents = advent::parse_input(12);
  let lines: Vec<&str> = contents.split("\n").collect();
  let mut elevations: HashMap<Point, i32> = HashMap::new();
  let mut start: Point = Point::new(0, 0);
  let mut end: Point = Point::new(0, 0);

  for row in 0..lines.len() {
    let line = lines[row];
    for col in 0..line.len() {
      let c = line.chars().nth(col).unwrap();
      let elevation = match c {
        'S' => 0,
        'E' => 25,
        _ => (c as i32) - 97,
      };
      let point = Point::new(col as i32, (lines.len() - row) as i32);
      if c == 'S' {
        start = point;
      }
      if c == 'E' {
        end = point;
      }
      elevations.insert(point, elevation);
    }
  }

  let successors = |point: &Point| -> Vec<(Point, i32)> {
    let curr_elevation = elevations[point];
    return point
      .adj4()
      .iter()
      .map(|p| {
        if !elevations.contains_key(p) {
          return (*p, i32::MAX);
        }
        let elevation = elevations[p];
        if elevation < curr_elevation - 1 {
          return (*p, i32::MAX);
        }
        return (*p, 1);
      })
      .filter(|pair| pair.1 != i32::MAX)
      .collect();
  };

  let result: HashMap<Point, (Point, i32)> = dijkstra_all(&end, successors);
  let best_distance = result
    .iter()
    .map(|(k, v)| if elevations[k] == 0 { v.1 } else { i32::MAX })
    .min()
    .unwrap();

  println!("Part 1:");
  println!("{}", result[&start].1);
  println!("Part 2:");
  println!("{}", best_distance);
}
