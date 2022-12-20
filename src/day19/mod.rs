use crate::util::advent;
use std::{
  cmp::{self, Ordering},
  collections::HashMap,
  hash::Hash,
  ops,
};

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Resources(u32, u32, u32, u32); // ore, clay, obsidian, geode

impl ops::Sub<Resources> for Resources {
  type Output = Resources;
  fn sub(self, rhs: Resources) -> Resources {
    return Resources(
      self.0 - rhs.0,
      self.1 - rhs.1,
      self.2 - rhs.2,
      self.3 - rhs.3,
    );
  }
}

impl ops::Add<Resources> for Resources {
  type Output = Resources;
  fn add(self, rhs: Resources) -> Resources {
    return Resources(
      self.0 + rhs.0,
      self.1 + rhs.1,
      self.2 + rhs.2,
      self.3 + rhs.3,
    );
  }
}

impl cmp::PartialOrd<Resources> for Resources {
  fn partial_cmp(&self, other: &Resources) -> Option<cmp::Ordering> {
    if self == other {
      return Some(Ordering::Equal);
    }
    if self.0 >= other.0 && self.1 >= other.1 && self.2 >= other.2 && self.3 >= other.3 {
      return Some(Ordering::Greater);
    }
    if self.0 <= other.0 && self.1 <= other.1 && self.2 <= other.2 && self.3 <= other.3 {
      return Some(Ordering::Less);
    }
    return None;
  }
}

#[derive(Eq, PartialEq, Clone)]
struct Blueprint {
  id: u32,
  cost: HashMap<u32, Resources>,
}

#[derive(PartialEq, Eq, Clone, Hash)]
struct MineState {
  resources: Resources,
  robots: Resources,
  id: u32,
  minutes_left: u32,
}

fn number_of_geodes(
  resources: Resources,
  robots: Resources,
  blueprint: &Blueprint,
  minutes_left: u32,
  cache: &mut HashMap<MineState, u32>,
  best_so_far: u32,
) -> u32 {
  // Short circuit bottom 3 cases
  if minutes_left == 0 {
    return resources.3;
  }
  if minutes_left == 1 {
    return resources.3 + robots.3;
  }
  if minutes_left == 2 {
    if resources.clone() > blueprint.cost[&3] {
      return resources.3 + robots.3 * 2 + 1;
    }
    return resources.3 + robots.3 * 2;
  }
  // Check cache, and whether it's impossible for this case to be better than optimal
  let state = MineState {
    resources: resources.clone(),
    robots: robots.clone(),
    id: blueprint.id,
    minutes_left: minutes_left,
  };
  if cache.contains_key(&state) {
    return cache[&state];
  }
  if best_so_far > resources.3 + robots.3 * minutes_left + (minutes_left * (minutes_left - 1)) / 2 {
    cache.insert(state, best_so_far); // Not technically true, but doesn't affect result
    return best_so_far;
  }
  let new_resources = resources.clone() + robots.clone();
  // If possible to make a geode miner, make one for sure and return.
  if resources.clone() >= blueprint.cost[&3] {
    let return_value = number_of_geodes(
      new_resources.clone() - blueprint.cost[&3],
      robots + Resources(0, 0, 0, 1),
      blueprint,
      minutes_left - 1,
      cache,
      best_so_far,
    );
    cache.insert(state, return_value);
    return return_value;
  }

  // First guess is making no miners, then try making a miner of each type and return the maximum
  let mut max_value = number_of_geodes(
    new_resources.clone(),
    robots.clone(),
    blueprint,
    minutes_left - 1,
    cache,
    best_so_far,
  );
  if resources.clone() >= blueprint.cost[&0]
    && robots.0 < blueprint.cost.values().map(|v| v.0).max().unwrap()
  {
    max_value = max_value.max(number_of_geodes(
      new_resources.clone() - blueprint.cost[&0],
      robots + Resources(1, 0, 0, 0),
      blueprint,
      minutes_left - 1,
      cache,
      max_value,
    ));
  }
  if resources.clone() >= blueprint.cost[&1]
    && robots.1 < blueprint.cost.values().map(|v| v.1).max().unwrap()
  {
    max_value = max_value.max(number_of_geodes(
      new_resources.clone() - blueprint.cost[&1],
      robots + Resources(0, 1, 0, 0),
      blueprint,
      minutes_left - 1,
      cache,
      max_value,
    ));
  }
  if resources.clone() >= blueprint.cost[&2]
    && robots.2 < blueprint.cost.values().map(|v| v.2).max().unwrap()
  {
    max_value = max_value.max(number_of_geodes(
      new_resources.clone() - blueprint.cost[&2],
      robots + Resources(0, 0, 1, 0),
      blueprint,
      minutes_left - 1,
      cache,
      max_value,
    ));
  }
  cache.insert(state, max_value);
  return max_value;
}

pub fn run() {
  let contents = advent::parse_input(19);
  let lines: Vec<&str> = contents.split("\n").collect();
  let mut blueprints: Vec<Blueprint> = [].to_vec();

  for line in lines {
    let numbers: Vec<u32> = advent::get_numbers(line, &u32::from_str_radix, 10);
    let mut cost: HashMap<u32, Resources> = HashMap::new();
    cost.insert(0, Resources(numbers[1], 0, 0, 0));
    cost.insert(1, Resources(numbers[2], 0, 0, 0));
    cost.insert(2, Resources(numbers[3], numbers[4], 0, 0));
    cost.insert(3, Resources(numbers[5], 0, numbers[6], 0));

    let new_blueprint = Blueprint {
      id: numbers[0],
      cost: cost,
    };
    blueprints.push(new_blueprint);
  }

  let quality_levels: Vec<u32> = blueprints
    .iter()
    .map(|bp| {
      let mut cache: HashMap<MineState, u32> = HashMap::new();
      let res = bp.id
        * number_of_geodes(
          Resources(0, 0, 0, 0),
          Resources(1, 0, 0, 0),
          bp,
          24,
          &mut cache,
          0,
        );
      return res;
    })
    .collect();

  println!("Part 1:");
  println!("{}", quality_levels.iter().sum::<u32>());
  println!("Part 2:");
  println!(
    "{}",
    blueprints
      .iter()
      .take(3)
      .map(|bp| {
        let mut cache: HashMap<MineState, u32> = HashMap::new();
        let res = number_of_geodes(
          Resources(0, 0, 0, 0),
          Resources(1, 0, 0, 0),
          bp,
          32,
          &mut cache,
          0,
        );
        return res;
      })
      .product::<u32>()
  );
}
