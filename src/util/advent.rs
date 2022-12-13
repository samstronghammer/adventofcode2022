use std::fs;

pub fn parse_input(day: u8) -> String {
  println!("Starting Day {:02}", day);
  let path = format!("src/day{:02}/in.txt", day);
  let error_string = format!("Should have been able to read file {}", path);
  return fs::read_to_string(path).expect(&error_string);
}
