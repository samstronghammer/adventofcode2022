use crate::util::advent;
use pathfinding::prelude::dijkstra_all;
use std::collections::{HashMap, HashSet};

// Finds all valid paths which use up the given minutes remaining.
fn get_valid_paths(
  distances: &HashMap<String, HashMap<String, i32>>,
  starts_with: Vec<String>,
  minutes: i32,
) -> Vec<Vec<String>> {
  let mut valid_paths: Vec<Vec<String>> = [].to_vec();
  for next in &distances[starts_with.last().unwrap()] {
    let time_taken = *next.1 + 1;
    if starts_with.contains(next.0) || minutes < time_taken {
      continue;
    }
    let mut new_start = starts_with.clone();
    new_start.push(next.0.clone());
    for valid_path in get_valid_paths(distances, new_start, minutes - time_taken) {
      valid_paths.push(valid_path);
    }
  }
  if valid_paths.len() == 0 {
    valid_paths.push(starts_with);
  }
  return valid_paths;
}

// Convert path to the amount of pressure released
fn path_to_pressure(
  path: Vec<String>,
  distances: &HashMap<String, HashMap<String, i32>>,
  flow_rates: &HashMap<String, i32>,
  minutes: i32,
) -> i32 {
  let mut pressure = 0;
  let mut minutes_left = minutes;
  for i in 0..path.len() - 1 {
    let location = path[i].clone();
    let next_location = path[i + 1].clone();
    let time_to_open: i32 = distances[&location][&next_location] + 1;
    minutes_left -= time_to_open;
    pressure += flow_rates[&next_location] * minutes_left;
  }
  return pressure;
}

// Converts a path to a useful binary id. Allows for intersection checking with
// binary operations.
// ids MUST BE SORTED
fn path_to_binary(path: &Vec<String>, ids: &Vec<String>) -> u32 {
  let mut binary = 0;
  for id in ids {
    binary = binary << 1;
    if path.contains(&id) {
      binary |= 1;
    }
  }
  return binary;
}

pub fn run() {
  let contents = advent::parse_input(16);
  let lines: Vec<&str> = contents.split("\n").collect();
  let mut valves: HashMap<String, Vec<String>> = HashMap::new();
  let mut flow_rates: HashMap<String, i32> = HashMap::new();

  for line in lines {
    let clean_line = line.replace("tunnel leads to valve", "tunnels lead to valves");
    let toks: Vec<&str> = clean_line.split("; tunnels lead to valves ").collect();
    let first_half: Vec<&str> = toks[0].split([' ', '=']).collect();
    let id = String::from(first_half[1]);
    let rate = i32::from_str_radix(first_half[5], 10).unwrap();
    let to_valves: Vec<String> = toks[1].split(", ").map(|s| String::from(s)).collect();
    valves.insert(id.clone(), to_valves);
    flow_rates.insert(id, rate);
  }

  let mut important_valves: HashSet<String> = HashSet::new();
  valves.keys().filter(|k| flow_rates[*k] > 0).for_each(|k| {
    important_valves.insert(k.clone());
  });
  important_valves.insert("AA".to_string());

  let mut important_ids_vec: Vec<String> = important_valves
    .iter()
    .filter(|v| **v != "AA".to_string())
    .map(|v| v.clone())
    .collect();
  important_ids_vec.sort();

  let successors = |id: &String| -> Vec<(String, i32)> {
    return valves[id].iter().map(|v| (v.clone(), 1)).collect();
  };
  let mut distances: HashMap<String, HashMap<String, i32>> = HashMap::new();
  for valve in important_valves.clone() {
    let result: HashMap<String, (String, i32)> = dijkstra_all(&valve, successors);
    let mut sub_map: HashMap<String, i32> = HashMap::new();
    for (k, v) in result {
      if important_valves.contains(&k) {
        sub_map.insert(k, v.1);
      }
    }
    distances.insert(valve, sub_map);
  }

  println!("Part 1:");
  println!(
    "{}",
    get_valid_paths(&distances, ["AA".to_string()].to_vec(), 30)
      .iter()
      .map(|p| path_to_pressure(p.clone(), &distances, &flow_rates, 30))
      .max()
      .unwrap()
  );

  // List of all possible paths
  let paths = get_valid_paths(&distances, ["AA".to_string()].to_vec(), 26);
  // Annotate paths with their binary id and pressure values
  let paths_with_pressure: Vec<(Vec<String>, u32, i32)> = paths
    .iter()
    .map(|path| {
      (
        path.clone(),
        path_to_binary(path, &important_ids_vec),
        path_to_pressure(path.clone(), &distances, &flow_rates, 26),
      )
    })
    .collect();

  // Find the best ordering for each path with the same set of locations
  let mut best_path_map: HashMap<u32, (Vec<String>, u32, i32)> = HashMap::new();
  for path in paths_with_pressure.clone() {
    let id = path.1;
    if best_path_map.contains_key(&id) {
      let old_best = &best_path_map[&id];
      if path.2 > old_best.2 {
        best_path_map.insert(id, path);
      }
    } else {
      best_path_map.insert(id, path);
    }
  }

  // Convert best orderings to a vector again
  let best_paths_with_pressure: Vec<(Vec<String>, u32, i32)> =
    best_path_map.values().map(|v| v.clone()).collect();

  // Find highest pressure path (for extra shortcut later)
  let best_pressure = best_paths_with_pressure.iter().map(|p| p.2).max().unwrap();

  // Copy best paths to iterate against
  let best_paths_with_pressure_2 = best_paths_with_pressure.clone();

  // Iterate over every pairing of paths, finding the pairing that is both valid and has the highest pressure sum
  let mut best = 0;
  let l = best_paths_with_pressure.len();
  for i in 0..l {
    let p1 = &best_paths_with_pressure[i];
    if best > p1.2 + best_pressure {
      // Shortcut if even the best second path wouldn't do enough
      continue;
    }
    for j in (i + 1)..l {
      let p2 = &best_paths_with_pressure_2[j];
      if p1.1 & p2.1 > 0 {
        // Shortcut if the ids have any intersection (they share a node)
        continue;
      }
      best = best.max(p1.2 + p2.2);
    }
  }
  println!("Part 2:");
  println!("{}", best);
}
