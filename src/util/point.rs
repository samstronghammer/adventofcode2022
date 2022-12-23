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

  pub fn adj8(self) -> Vec<Point> {
    return [
      self + Point::UP + Point::LEFT,
      self + Point::UP,
      self + Point::UP + Point::RIGHT,
      self + Point::LEFT,
      self + Point::RIGHT,
      self + Point::DOWN + Point::LEFT,
      self + Point::DOWN,
      self + Point::DOWN + Point::RIGHT,
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

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct Point3 {
  pub x: i32,
  pub y: i32,
  pub z: i32,
}

impl Point3 {
  pub fn new(x: i32, y: i32, z: i32) -> Self {
    Point3 { x: x, y: y, z: z }
  }

  pub const X_UP: Point3 = Point3 { x: 1, y: 0, z: 0 };
  pub const X_DOWN: Point3 = Point3 { x: -1, y: 0, z: 0 };
  pub const Y_UP: Point3 = Point3 { x: 0, y: 1, z: 0 };
  pub const Y_DOWN: Point3 = Point3 { x: 0, y: -1, z: 0 };
  pub const Z_UP: Point3 = Point3 { x: 0, y: 0, z: 1 };
  pub const Z_DOWN: Point3 = Point3 { x: 0, y: 0, z: -1 };

  pub fn adj6(self) -> Vec<Point3> {
    return [
      self + Point3::X_UP,
      self + Point3::X_DOWN,
      self + Point3::Y_UP,
      self + Point3::Y_DOWN,
      self + Point3::Z_UP,
      self + Point3::Z_DOWN,
    ]
    .to_vec();
  }

  pub fn manhattan_distance(self, other: Point3) -> i32 {
    return (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs();
  }
}

impl ops::Add<Point3> for Point3 {
  type Output = Point3;
  fn add(self, rhs: Point3) -> Point3 {
    return Point3 {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    };
  }
}

impl ops::Sub<Point3> for Point3 {
  type Output = Point3;
  fn sub(self, rhs: Point3) -> Point3 {
    return Point3 {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
    };
  }
}

impl ops::Div<Point3> for Point3 {
  type Output = Point3;
  fn div(self, rhs: Point3) -> Point3 {
    return Point3 {
      x: self.x / rhs.x,
      y: self.y / rhs.y,
      z: self.z / rhs.z,
    };
  }
}
