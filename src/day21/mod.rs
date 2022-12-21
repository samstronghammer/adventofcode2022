use crate::util::advent;
use std::collections::HashMap;

#[derive(Clone)]
struct Monkey {
  id: String,
  equation: Option<Equation>,
  value: Option<i64>,
}

#[derive(Clone)]
struct Equation(String, char, String);

fn calc_equation(monkeys: &HashMap<String, Monkey>, equation: Equation) -> i64 {
  let left = calc_monkey(monkeys, equation.0.clone());
  let right = calc_monkey(monkeys, equation.2.clone());
  return match equation.1 {
    '*' => left * right,
    '+' => left + right,
    '-' => left - right,
    '/' => left / right,
    '=' => left - right, // Providing the difference is more information, which is helpful for searching the space
    _ => panic!("unknown operand"),
  };
}

fn calc_monkey(monkeys: &HashMap<String, Monkey>, id: String) -> i64 {
  let monkey = &monkeys[&id];
  if monkey.value.is_some() {
    return monkey.value.unwrap();
  }
  return calc_equation(monkeys, monkey.equation.clone().unwrap());
}

pub fn run() {
  let contents = advent::parse_input(21);
  let lines: Vec<&str> = contents.split('\n').collect();

  let mut monkeys: HashMap<String, Monkey> = HashMap::new();

  for line in lines {
    let toks: Vec<&str> = line.split([':', ' ']).collect();
    let id = toks[0].to_string();
    let equation = if toks.len() == 3 {
      None
    } else {
      Some(Equation(
        toks[2].to_string(),
        toks[3].chars().nth(0).unwrap(),
        toks[4].to_string(),
      ))
    };
    let value = if toks.len() == 3 {
      Some(i64::from_str_radix(toks[2], 10).unwrap())
    } else {
      None
    };
    monkeys.insert(
      id.clone(),
      Monkey {
        id: id,
        equation: equation,
        value: value,
      },
    );
  }

  let mut monkeys_p2 = monkeys.clone();
  let root = monkeys_p2.get_mut(&"root".to_string()).unwrap();
  root.equation = Some(Equation(
    root.equation.clone().unwrap().0,
    '=',
    root.equation.clone().unwrap().2,
  ));

  println!("Part 1:");
  println!("{}", calc_monkey(&monkeys, "root".to_string()));
  println!("Part 2:");
  let mut i = 3441198825701; // I found this magic number by doing manual binary search, narrowing the search space by orders of magnitude at a time until I found something close enough that it was able to find the solution quickly.
  loop {
    monkeys_p2.get_mut(&"humn".to_string()).unwrap().value = Some(i);
    let result = calc_monkey(&monkeys_p2, "root".to_string());
    if result == 0 {
      println!("{}", i);
      break;
    }
    i += 1;
  }
}
