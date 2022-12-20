use crate::util::advent;
use std::collections::HashMap;

struct Node {
  id: u32,
  value: i64,
  next: Option<u32>,
  prev: Option<u32>,
}

struct Circle {
  nodes: HashMap<u32, Box<Node>>,
  id_0: u32,
}

impl Circle {
  pub fn mix(&mut self, num_times: u32) {
    let mod_value: i64 = i64::try_from(self.size()).unwrap() - 1;
    for _ in 0..num_times {
      for id in 0..(self.size()) {
        let number = self.nodes.get(&id).unwrap().value;
        if number == 0 {
          continue;
        }
        let move_count = ((number % mod_value) + mod_value) % mod_value;
        if move_count == 0 {
          continue;
        }
        let node = self.nodes.get(&id).unwrap();
        let old_prev_id = node.prev.unwrap();
        let old_next_id = node.next.unwrap();
        let mut prev_id = node.id;
        for _ in 0..move_count {
          prev_id = self.nodes.get(&prev_id).unwrap().next.unwrap();
        }
        let next_id = self.nodes.get(&prev_id).unwrap().next.unwrap();

        self.nodes.get_mut(&old_prev_id).unwrap().next = Some(old_next_id);
        self.nodes.get_mut(&old_next_id).unwrap().prev = Some(old_prev_id);
        self.nodes.get_mut(&prev_id).unwrap().next = Some(id);
        self.nodes.get_mut(&next_id).unwrap().prev = Some(id);
        let curr_node = self.nodes.get_mut(&id).unwrap();
        curr_node.prev = Some(prev_id);
        curr_node.next = Some(next_id);
      }
    }
  }

  pub fn size(&self) -> u32 {
    return u32::try_from(self.nodes.len()).unwrap();
  }

  pub fn get_grove_sum(&self) -> i64 {
    let mut curr_node = &self.nodes[&self.id_0];
    let mut sum = 0;
    for i in 0..3000 {
      curr_node = &self.nodes[&curr_node.next.unwrap()];
      if (i + 1) % 1000 == 0 {
        sum += curr_node.value;
      }
    }
    return sum;
  }

  pub fn from_numbers(numbers: Vec<i64>, factor: i64) -> Circle {
    let mut circle: Circle = Circle {
      nodes: HashMap::new(),
      id_0: 0,
    };
    let mut id_counter = 0;
    for number in numbers {
      let id = id_counter;
      if number == 0 {
        circle.id_0 = id;
      }
      let prev: Option<u32> = if id > 0 {
        Some(circle.nodes.get_mut(&(id - 1)).unwrap().id)
      } else {
        None
      };
      let new_node = Box::new(Node {
        id: id,
        value: number * factor,
        next: None,
        prev: prev,
      });
      circle.nodes.insert(id, new_node);
      if id > 0 {
        circle.nodes.get_mut(&(id - 1)).unwrap().next = Some(id);
      }
      id_counter += 1;
    }
    circle.nodes.get_mut(&0).unwrap().prev = Some(circle.size() - 1);
    circle.nodes.get_mut(&(circle.size() - 1)).unwrap().next = Some(0);
    return circle;
  }
}

pub fn run() {
  let contents = advent::parse_input(20);
  let numbers: Vec<i64> = contents
    .split("\n")
    .map(|line| i64::from_str_radix(line, 10).unwrap())
    .collect();
  let mut circle_1: Circle = Circle::from_numbers(numbers.clone(), 1);
  let mut circle_2: Circle = Circle::from_numbers(numbers, 811589153);

  circle_1.mix(1);
  circle_2.mix(10);

  println!("Part 1:");
  println!("{}", circle_1.get_grove_sum());
  println!("Part 2:");
  println!("{}", circle_2.get_grove_sum());
}
