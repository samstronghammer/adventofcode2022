use crate::util::advent;
use std::cmp::Ordering;
use substring::Substring;

#[derive(Eq, Hash, PartialEq, Clone)]
struct Packet {
  is_terminal: bool,
  children: Vec<Packet>,
  value: Option<u8>,
}

fn string_to_children(s: &str) -> Vec<Packet> {
  let mut children = [].to_vec();
  let mut num_brackets = 0;
  let mut child_start = 0;
  for i in 0..s.len() {
    let c = s.chars().nth(i).unwrap();
    if c == '[' {
      num_brackets += 1;
      if num_brackets == 0 {
        child_start = i + 1;
      }
    } else if c == ']' {
      if num_brackets == 0 {
        children.push(string_to_packet(s.substring(child_start, i)));
      }
      num_brackets -= 1;
    } else if c == ',' {
      if num_brackets == 0 {
        children.push(string_to_packet(s.substring(child_start, i)));
        child_start = i + 1;
      }
    }
  }
  if num_brackets != 0 {
    panic!("Didn't match all brackets");
  }
  if s.len() > 0 {
    children.push(string_to_packet(s.substring(child_start, s.len())));
  }
  return children;
}

fn string_to_packet(s: &str) -> Packet {
  if s.len() == 0 {
    panic!("Empty String");
  }
  if s.chars().nth(0).unwrap() == '[' {
    return Packet {
      is_terminal: false,
      children: string_to_children(s.substring(1, s.len() - 1)),
      value: None,
    };
  } else {
    return Packet {
      is_terminal: true,
      children: [].to_vec(),
      value: Some(u8::from_str_radix(s, 10).unwrap()),
    };
  }
}

fn compare(left_packet: &Packet, right_packet: &Packet) -> Ordering {
  if left_packet.is_terminal && right_packet.is_terminal {
    return left_packet.value.unwrap().cmp(&right_packet.value.unwrap());
  }
  if !left_packet.is_terminal && !right_packet.is_terminal {
    let num_compare = usize::min(left_packet.children.len(), right_packet.children.len());
    for i in 0..num_compare {
      let comparison = compare(&left_packet.children[i], &right_packet.children[i]);
      if comparison != Ordering::Equal {
        return comparison;
      }
    }
    return left_packet.children.len().cmp(&right_packet.children.len());
  }
  if left_packet.is_terminal {
    return compare(
      &Packet {
        is_terminal: false,
        children: [left_packet.clone()].to_vec(),
        value: None,
      },
      right_packet,
    );
  }
  return compare(
    left_packet,
    &Packet {
      is_terminal: false,
      children: [right_packet.clone()].to_vec(),
      value: None,
    },
  );
}

pub fn run() {
  let contents = advent::parse_input(13);
  let raw_pairs: Vec<&str> = contents.split("\n\n").collect();
  let mut indices: Vec<usize> = [].to_vec();
  let mut packets: Vec<Packet> = [].to_vec();
  let divider1 = string_to_packet("[[2]]");
  let divider2 = string_to_packet("[[6]]");
  packets.push(divider1.clone());
  packets.push(divider2.clone());

  for i in 0..raw_pairs.len() {
    let pair = raw_pairs[i];
    let halves: Vec<&str> = pair.split("\n").collect();
    let left_packet = string_to_packet(halves[0]);
    let right_packet = string_to_packet(halves[1]);
    packets.push(left_packet.clone());
    packets.push(right_packet.clone());
    let ordering = compare(&left_packet, &right_packet);
    if ordering == Ordering::Less {
      indices.push(i + 1);
    }
  }

  packets.sort_by(compare);

  println!("Part 1:");
  println!("{}", indices.iter().sum::<usize>());
  println!("Part 2:");
  println!(
    "{}",
    (packets.iter().position(|v| divider1.eq(v)).unwrap() + 1)
      * (packets.iter().position(|v| divider2.eq(v)).unwrap() + 1)
  );
}
