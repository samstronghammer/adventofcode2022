use crate::util::{advent, point::Point};
use std::collections::HashSet;

fn cant_have_beacon(location: Point, sensor_beacon_info: &Vec<(Point, i32)>) -> bool {
  for info in sensor_beacon_info {
    let distance = info.0.manhattan_distance(location);
    if distance <= info.1 {
      return true;
    }
  }
  return false;
}

fn find_beacon(sensor_beacon_info: &Vec<(Point, i32)>) -> Point {
  let mut curr = Point::new(0, 0);
  loop {
    if curr.y > 4000000 {
      panic!("Didn't find beacon")
    }
    let intersecting_beacon_option = sensor_beacon_info
      .iter()
      .find(|info| info.0.manhattan_distance(curr) <= info.1);
    if intersecting_beacon_option.is_none() {
      break;
    }
    let intersecting_beacon = intersecting_beacon_option.unwrap();
    let delta_y = (intersecting_beacon.0.y - curr.y).abs();
    let next_x = intersecting_beacon.0.x + intersecting_beacon.1 - delta_y + 1;
    if next_x > 4000000 {
      curr = Point::new(0, curr.y + 1);
    } else {
      curr = Point::new(next_x, curr.y);
    }
  }
  return curr;
}

pub fn run() {
  let contents = advent::parse_input(15);
  let lines: Vec<&str> = contents.split("\n").collect();
  let mut sensor_beacon_info: Vec<(Point, i32)> = [].to_vec();
  let mut beacons: HashSet<Point> = HashSet::new();

  for line in lines {
    let toks: Vec<&str> = line.split([':', ',', '=']).collect();
    let sensor = Point::new(
      i32::from_str_radix(toks[1], 10).unwrap(),
      i32::from_str_radix(toks[3], 10).unwrap(),
    );
    let beacon = Point::new(
      i32::from_str_radix(toks[5], 10).unwrap(),
      i32::from_str_radix(toks[7], 10).unwrap(),
    );
    sensor_beacon_info.push((sensor, sensor.manhattan_distance(beacon)));
    beacons.insert(beacon);
  }

  let min_x = sensor_beacon_info
    .iter()
    .map(|pair| pair.0.x - pair.1)
    .min()
    .unwrap();

  let max_x = sensor_beacon_info
    .iter()
    .map(|pair| pair.0.x + pair.1)
    .max()
    .unwrap();

  let num_positions = (min_x..max_x)
    .filter(|x| {
      let point = Point::new(*x, 2000000);
      if beacons.contains(&point) {
        return false;
      }
      return cant_have_beacon(point, &sensor_beacon_info);
    })
    .count();

  let beacon = find_beacon(&sensor_beacon_info);

  println!("Part 1:");
  println!("{}", num_positions);
  println!("Part 2:");
  println!("{}", i64::from(beacon.x) * 4000000 + i64::from(beacon.y));
}
