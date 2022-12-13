use crate::util::advent;
use crate::util::point::Point;
use std::collections::HashSet;

fn char_to_vec(c: char) -> Point {
  match c {
    'U' => return Point::UP,
    'R' => return Point::RIGHT,
    'D' => return Point::DOWN,
    'L' => return Point::LEFT,
    _ => panic!("unknown char"),
  }
}

const TWO: Point = Point { x: 2, y: 2 };
const ZERO: Point = Point { x: 0, y: 0 };

fn apply_vec(rope: &mut Vec<Point>, vec: Point) {
  let l = rope.len();
  rope[0] = rope[0] + vec;
  for i in 1..l {
    let delta = rope[i - 1] - rope[i];
    // Uses integer division to handle edge cases. If the manhattan distance between
    // the previous knot and this one is 2 or less, we can safely divide by 2.
    // Otherwise, we need to move diagonally using signum for direction.
    let knot_delta = if delta.x.abs() + delta.y.abs() <= 2 {
      delta / TWO
    } else {
      Point::new(delta.x.signum(), delta.y.signum())
    };
    // Can short circuit here. If this knot doesn't move, we don't need
    // to check the next knots.
    if knot_delta == ZERO {
      break;
    }
    rope[i] = rope[i] + knot_delta;
  }
}

pub fn run() {
  let contents = advent::parse_input(9);
  let lines: Vec<&str> = contents.split("\n").collect();
  let mut visited1: HashSet<Point> = HashSet::new();
  let mut visited2: HashSet<Point> = HashSet::new();
  let mut rope1: Vec<Point> = vec![Point::new(0, 0); 2];
  let mut rope2: Vec<Point> = vec![Point::new(0, 0); 10];

  visited1.insert(Point::new(0, 0));
  visited2.insert(Point::new(0, 0));

  for line in lines {
    let toks: Vec<&str> = line.split(" ").collect();
    let direction = char_to_vec(toks.get(0).unwrap().chars().nth(0).unwrap());
    let magnitude = i32::from_str_radix(toks.get(1).unwrap(), 10).unwrap();
    for _ in 0..magnitude {
      apply_vec(&mut rope1, direction);
      apply_vec(&mut rope2, direction);
      visited1.insert(*rope1.last().unwrap());
      visited2.insert(*rope2.last().unwrap());
    }
  }

  println!("Part 1:");
  println!("{}", visited1.len());
  println!("Part 2:");
  println!("{}", visited2.len());
}
