use crate::util::{advent, point::Point};
use pathfinding::directed::astar::astar;
use std::collections::{HashMap, HashSet};

pub fn run() {
  let contents = advent::parse_input(24);
  let lines: Vec<&str> = contents.split("\n").collect();
  let mut walls: HashSet<Point> = HashSet::new();
  let start = Point::new(1, 0);
  walls.insert(start + Point::DOWN);
  let mut blizzards: Vec<(Point, Point)> = Vec::new();

  for y in 0..lines.len() {
    let line = lines[y];
    for x in 0..line.len() {
      let c = line.chars().nth(x).unwrap();
      let point = Point::new(x.try_into().unwrap(), y.try_into().unwrap());
      if c == '#' {
        walls.insert(point);
      } else if c == '>' {
        blizzards.push((point, Point::RIGHT));
      } else if c == 'v' {
        blizzards.push((point, Point::UP));
      } else if c == '<' {
        blizzards.push((point, Point::LEFT));
      } else if c == '^' {
        blizzards.push((point, Point::DOWN));
      }
    }
  }

  let room_width: i32 = i32::try_from(lines[0].len() - 2).unwrap();
  let room_height: i32 = i32::try_from(lines.len() - 2).unwrap();
  let goal = Point::new(room_width, room_height + 1);
  walls.insert(goal + Point::UP);

  let mut blizzard_cache: HashMap<u32, HashSet<Point>> = HashMap::new();

  for i in 0..3000 {
    let mut cache: HashSet<Point> = HashSet::new();
    for blizzard in blizzards.iter() {
      let new_x =
        ((((blizzard.0.x + blizzard.1.x * i - 1) % room_width) + room_width) % room_width) + 1;
      let new_y =
        ((((blizzard.0.y + blizzard.1.y * i - 1) % room_height) + room_height) % room_height) + 1;
      cache.insert(Point::new(new_x, new_y));
    }
    blizzard_cache.insert(u32::try_from(i).unwrap(), cache);
  }

  let successors = |curr: &(Point, u32)| -> Vec<((Point, u32), u32)> {
    let mut options = curr.0.adj4();
    options.push(curr.0);
    return options
      .iter()
      .map(|p| {
        return ((*p, curr.1 + 1), 1);
      })
      .filter(|pair| {
        !blizzard_cache[&(pair.0).1].contains(&(pair.0).0) && !walls.contains(&(pair.0).0)
      })
      .collect();
  };

  let heuristic = |curr: &(Point, u32)| -> u32 {
    return u32::try_from(goal.manhattan_distance(curr.0)).unwrap();
  };

  let success = |curr: &(Point, u32)| -> bool {
    return curr.0 == goal;
  };

  let heuristic_2 = |curr: &(Point, u32)| -> u32 {
    return u32::try_from(start.manhattan_distance(curr.0)).unwrap();
  };

  let success_2 = |curr: &(Point, u32)| -> bool {
    return curr.0 == start;
  };

  let result: (Vec<(Point, u32)>, u32) =
    astar(&(start, 0), successors, heuristic, success).unwrap(); // Start to end

  let result_2: (Vec<(Point, u32)>, u32) =
    astar(&(goal, result.1), successors, heuristic_2, success_2).unwrap(); // End to start

  let result_3: (Vec<(Point, u32)>, u32) = astar(
    &(start, result.1 + result_2.1),
    successors,
    heuristic,
    success,
  )
  .unwrap(); // Back to end again

  println!("Part 1:");
  println!("{}", result.1);
  println!("Part 2:");
  println!("{}", result.1 + result_2.1 + result_3.1);
}
