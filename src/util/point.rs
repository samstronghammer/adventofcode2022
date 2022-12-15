use std::ops;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct Point {
  pub x: i32,
  pub y: i32,
}

impl Point {
  pub fn new(x: i32, y: i32) -> Self {
    Point { x: x, y: y }
  }

  pub const UP: Point = Point { x: 0, y: 1 };
  pub const RIGHT: Point = Point { x: 1, y: 0 };
  pub const DOWN: Point = Point { x: 0, y: -1 };
  pub const LEFT: Point = Point { x: -1, y: 0 };

  pub fn adj4(self) -> Vec<Point> {
    return [
      self + Point::UP,
      self + Point::LEFT,
      self + Point::RIGHT,
      self + Point::DOWN,
    ]
    .to_vec();
  }

  pub fn manhattan_distance(self, other: Point) -> i32 {
    return (self.x - other.x).abs() + (self.y - other.y).abs();
  }
}

impl ops::Add<Point> for Point {
  type Output = Point;
  fn add(self, rhs: Point) -> Point {
    return Point {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    };
  }
}

impl ops::Sub<Point> for Point {
  type Output = Point;
  fn sub(self, rhs: Point) -> Point {
    return Point {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
    };
  }
}

impl ops::Div<Point> for Point {
  type Output = Point;
  fn div(self, rhs: Point) -> Point {
    return Point {
      x: self.x / rhs.x,
      y: self.y / rhs.y,
    };
  }
}
