use crate::util::advent;

fn snafu_to_int(snafu: String) -> i64 {
  let mut total = 0;
  for c in snafu.chars() {
    total *= 5;
    total += match c {
      '2' => 2,
      '1' => 1,
      '0' => 0,
      '-' => -1,
      '=' => -2,
      _ => panic!("bad char: {}", c),
    };
  }
  return total;
}

fn int_to_snafu(int: i64) -> String {
  let mut snafu = String::new();
  let mut remaining = int;
  while remaining != 0 {
    let curr_digit = remaining % 5;
    snafu = String::from(match curr_digit {
      0 => "0",
      1 => "1",
      2 => "2",
      3 => "=",
      4 => "-",
      _ => panic!("bad"),
    }) + &snafu;

    remaining -= match curr_digit {
      3 => -2,
      4 => -1,
      _ => 0,
    };

    remaining /= 5;
  }

  return snafu;
}

pub fn run() {
  let contents = advent::parse_input(25);
  let lines: Vec<&str> = contents.split("\n").collect();

  let mut total = 0;
  for line in lines {
    total += snafu_to_int(String::from(line));
  }

  println!("Part 1:");
  println!("{}", int_to_snafu(total));
  println!("Part 2:");
  println!("Happy Holidays! 🎅");
}
