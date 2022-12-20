use regex::Regex;
use std::fs;

pub fn parse_input(day: u8) -> String {
  println!("Starting Day {:02}", day);
  let path = format!("src/day{:02}/in.txt", day);
  let error_string = format!("Should have been able to read file {}", path);
  return fs::read_to_string(path).expect(&error_string);
}

pub fn get_numbers<T, E>(s: &str, parse: &dyn Fn(&str, u32) -> Result<T, E>, radix: u32) -> Vec<T>
where
  T: Clone,
  E: std::fmt::Debug,
{
  let mut return_vector = [].to_vec();
  let re = Regex::new(r"-?(([0-9]+(\.[0-9]+)?)|(\.[0-9]+))").unwrap();
  for cap in re.find_iter(s) {
    return_vector.push(parse(cap.as_str(), radix).unwrap());
  }
  return return_vector;
}
