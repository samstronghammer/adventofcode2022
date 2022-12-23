use crate::util::{advent, point::Point};
use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
enum Side {
  S1,
  S2,
  S3,
  S4,
  S5,
  S6,
}

// Flip up and down because of coordinate space being based on rows
const UP: Point = Point::DOWN;
const DOWN: Point = Point::UP;
const LEFT: Point = Point::LEFT;
const RIGHT: Point = Point::RIGHT;

fn side_to_top_left(side: Side) -> Point {
  return match side {
    Side::S1 => Point::new(50, 0),
    Side::S2 => Point::new(100, 0),
    Side::S3 => Point::new(50, 50),
    Side::S4 => Point::new(0, 100),
    Side::S5 => Point::new(50, 100),
    Side::S6 => Point::new(0, 150),
  };
}

fn in_range(point: Point, min: Point, max: Point) -> bool {
  return min.x <= point.x && point.x <= max.x && min.y <= point.y && point.y <= max.y;
}

fn in_side(location: Point, side: Side) -> bool {
  let top_left = side_to_top_left(side);
  return in_range(location, top_left, top_left + Point::new(49, 49));
}

fn loc_to_side(location: Point) -> Side {
  return *[Side::S1, Side::S2, Side::S3, Side::S4, Side::S5, Side::S6]
    .iter()
    .find(|side| in_side(location, **side))
    .unwrap();
}

fn direction_to_password_value(direction: Point) -> i32 {
  return match direction {
    RIGHT => 0,
    DOWN => 1,
    LEFT => 2,
    UP => 3,
    _ => panic!("unknown direction"),
  };
}

// Location is 0-indexed
fn calc_password(location: Point, direction: Point) -> i32 {
  return 1000 * (location.y + 1) + 4 * (location.x + 1) + direction_to_password_value(direction);
}

// Hardcoded mapping between cube edges
fn wrap_cube(point: &Point, delta: &Point) -> (Side, Point, Point) {
  let curr_side = loc_to_side(*point);
  let side_delta = *point - side_to_top_left(curr_side.clone());
  return match (curr_side, *delta) {
    (Side::S1, UP) => (Side::S6, Point::new(0, side_delta.x), RIGHT),
    (Side::S1, LEFT) => (Side::S4, Point::new(0, 49 - side_delta.y), RIGHT),
    (Side::S2, UP) => (Side::S6, Point::new(side_delta.x, 49), UP),
    (Side::S2, RIGHT) => (Side::S5, Point::new(49, 49 - side_delta.y), LEFT),
    (Side::S2, DOWN) => (Side::S3, Point::new(49, side_delta.x), LEFT),
    (Side::S3, LEFT) => (Side::S4, Point::new(side_delta.y, 0), DOWN),
    (Side::S3, RIGHT) => (Side::S2, Point::new(side_delta.y, 49), UP),
    (Side::S4, UP) => (Side::S3, Point::new(0, side_delta.x), RIGHT),
    (Side::S4, LEFT) => (Side::S1, Point::new(0, 49 - side_delta.y), RIGHT),
    (Side::S5, RIGHT) => (Side::S2, Point::new(49, 49 - side_delta.y), LEFT),
    (Side::S5, DOWN) => (Side::S6, Point::new(49, side_delta.x), LEFT),
    (Side::S6, LEFT) => (Side::S1, Point::new(side_delta.y, 0), DOWN),
    (Side::S6, RIGHT) => (Side::S5, Point::new(side_delta.y, 49), UP),
    (Side::S6, DOWN) => (Side::S2, Point::new(side_delta.x, 0), DOWN),
    _ => panic!("Unrecognized pairing"),
  };
}

fn simulate(
  start_location: Point,
  tiles: &HashSet<Point>,
  walls: &HashSet<Point>,
  wrapping: &HashMap<(Point, Point), (Point, Point)>,
  magnitudes: &Vec<u32>,
  turns: &String,
) -> (Point, Point) {
  let mut location: Point = start_location.clone();
  let mut direction: Point = RIGHT;
  for i in 0..magnitudes.len() {
    let magnitude = magnitudes[i];
    for _ in 0..magnitude {
      let new_location = location + direction;
      if tiles.contains(&new_location) {
        location = new_location;
        continue;
      }
      if walls.contains(&new_location) {
        break;
      }
      if wrapping.contains_key(&(location, direction)) {
        (location, direction) = *wrapping.get(&(location, direction)).unwrap();
        continue;
      }
      // Will happen if wrapping doesn't work (aka wall on other side of wrap)
      break;
    }
    if i < turns.len() {
      let turn_char = turns.chars().nth(i).unwrap();
      if turn_char == 'L' {
        direction = Point::new(direction.y, -direction.x);
      } else if turn_char == 'R' {
        direction = Point::new(-direction.y, direction.x);
      } else {
        panic!("unknown char");
      }
    }
  }
  return (location, direction);
}

pub fn run() {
  let contents = advent::parse_input(22);
  let lines: Vec<&str> = contents.split('\n').collect();
  let mut tiles: HashSet<Point> = HashSet::new();
  let mut walls: HashSet<Point> = HashSet::new();
  let mut wrapping_1: HashMap<(Point, Point), (Point, Point)> = HashMap::new(); // map from (curr tile, direction) => (new tile, new direction)
  let mut wrapping_2: HashMap<(Point, Point), (Point, Point)> = HashMap::new(); // map from (curr tile, direction) => (new tile, new direction)
  let mut location: Point = Point::new(0, 0);

  for row in 0..(lines.len() - 2) {
    let line = lines[row];
    for col in 0..line.len() {
      let c = line.chars().nth(col).unwrap();
      let point = Point::new(col.try_into().unwrap(), row.try_into().unwrap());
      if c == '#' {
        walls.insert(point);
      } else if c == '.' {
        if tiles.len() == 0 {
          location = point;
        }
        tiles.insert(point);
      }
    }
  }

  for point in tiles.iter() {
    for adj in point.adj4() {
      if !tiles.contains(&adj) && !walls.contains(&adj) {
        let delta = adj - *point;
        {
          // P1
          let other_direction = delta / Point::new(-1, -1);
          let mut curr_point = other_direction + *point;
          loop {
            let next_point = curr_point + other_direction;
            if tiles.contains(&next_point) || walls.contains(&next_point) {
              curr_point = next_point;
            } else {
              if tiles.contains(&curr_point) {
                wrapping_1.insert((*point, delta), (curr_point, delta));
              }
              break;
            }
          }
        }
        {
          // P2
          let (new_side, side_delta, new_direction) = wrap_cube(point, &delta);
          let new_location = side_to_top_left(new_side) + side_delta;
          if tiles.contains(&new_location) {
            wrapping_2.insert((*point, delta), (new_location, new_direction));
          }
        }
      }
    }
  }

  let instruction_line = *lines.last().unwrap();
  let magnitudes = advent::get_numbers(instruction_line, &u32::from_str_radix, 10);
  let turns: String = instruction_line
    .chars()
    .filter(|c| *c == 'R' || *c == 'L')
    .collect();

  let (p1loc, p1dir) = simulate(location, &tiles, &walls, &wrapping_1, &magnitudes, &turns);
  let (p2loc, p2dir) = simulate(location, &tiles, &walls, &wrapping_2, &magnitudes, &turns);

  println!("Part 1:");
  println!("{}", calc_password(p1loc, p1dir));
  println!("Part 2:");
  println!("{}", calc_password(p2loc, p2dir));
}
