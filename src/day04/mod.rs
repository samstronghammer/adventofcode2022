use crate::util::advent;

fn contains_range(big: &Range, small: &Range) -> bool {
  return big.min <= small.min && small.max <= big.max;
}

fn overlap(r1: &Range, r2: &Range) -> bool {
  return r1.max >= r2.min && r2.max >= r1.min;
}

struct Range {
  min: u32,
  max: u32,
}

pub fn run() {
  let contents = advent::parse_input(4);

  let lines: Vec<&str> = contents.split("\n").collect();
  let mut contain_count: u32 = 0;
  let mut overlap_count: u32 = 0;

  for line in lines {
    let toks: Vec<u32> = line
      .split(&[',', '-'])
      .map(|x| u32::from_str_radix(x, 10).unwrap())
      .collect();
    if toks.len() != 4 {
      panic!("Unexpected number of tokens");
    }
    let r1 = Range {
      min: toks[0],
      max: toks[1],
    };
    let r2 = Range {
      min: toks[2],
      max: toks[3],
    };
    if contains_range(&r1, &r2) || contains_range(&r2, &r1) {
      contain_count += 1;
    }
    if overlap(&r1, &r2) {
      overlap_count += 1;
    }
  }

  println!("Part 1:");
  println!("{}", contain_count);
  println!("Part 2:");
  println!("{}", overlap_count);
}
