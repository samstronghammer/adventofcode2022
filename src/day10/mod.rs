use std::fs;

pub fn run() {
  let contents =
    fs::read_to_string("src/day10/in.txt").expect("Should have been able to read the file");
  let lines: Vec<&str> = contents.split("\n").collect();
  let mut x_values: Vec<i32> = [].to_vec();
  let mut x = 1;

  for line in lines {
    let toks: Vec<&str> = line.split(" ").collect();
    if toks[0] == "noop" {
      x_values.push(x);
    } else {
      x_values.push(x);
      x_values.push(x);
      x += i32::from_str_radix(toks[1], 10).unwrap();
    }
  }

  let signal_strengths: Vec<i32> = (0..x_values.len())
    .map(|i| ((i as i32) + 1) * x_values[i])
    .collect();

  println!("Part 1:");
  println!(
    "{}",
    signal_strengths[19]
      + signal_strengths[59]
      + signal_strengths[99]
      + signal_strengths[139]
      + signal_strengths[179]
      + signal_strengths[219]
  );
  println!("Part 2:");
  for row in 0..6 {
    for col in 0..40 {
      let i = row * 40 + col;
      let lit_pixel: bool = (x_values[i] - i32::try_from(col).unwrap()).abs() < 2;
      if lit_pixel {
        print!("#");
      } else {
        print!(".");
      }
    }
    print!("\n");
  }
}
