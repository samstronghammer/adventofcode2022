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

fn calc_equation(monkeys: &HashMap<String, Monkey>, equation: Equation) -> Option<i64> {
  let left_option = calc_monkey(monkeys, equation.0.clone());
  let right_option = calc_monkey(monkeys, equation.2.clone());
  if left_option.is_some() && right_option.is_some() {
    let left = left_option.unwrap();
    let right = right_option.unwrap();
    return Some(match equation.1 {
      '*' => left * right,
      '+' => left + right,
      '-' => left - right,
      '/' => left / right,
      '=' => {
        if left == right {
          1
        } else {
          0
        }
      }
      _ => panic!("unknown operand"),
    });
  }
  return None;
}

fn calc_monkey(monkeys: &HashMap<String, Monkey>, id: String) -> Option<i64> {
  let monkey = &monkeys[&id];
  if monkey.equation.is_some() {
    return calc_equation(monkeys, monkey.equation.clone().unwrap());
  }
  return monkey.value;
}

fn calc_expectation(monkeys: &HashMap<String, Monkey>, equation: Equation, expected: i64) -> i64 {
  let left = calc_monkey(monkeys, equation.0.clone());
  let right = calc_monkey(monkeys, equation.2.clone());
  if (left.is_some() && right.is_some()) || (left.is_none() && right.is_none()) {
    panic!("Must have exactly one side be none");
  }
  let (next_expectation, recurse_id) = match equation.1 {
    '*' => {
      if left.is_none() {
        (expected / right.unwrap(), equation.0.clone())
      } else {
        (expected / left.unwrap(), equation.2.clone())
      }
    }
    '+' => {
      if left.is_none() {
        (expected - right.unwrap(), equation.0.clone())
      } else {
        (expected - left.unwrap(), equation.2.clone())
      }
    }
    '-' => {
      if left.is_none() {
        (expected + right.unwrap(), equation.0.clone())
      } else {
        (left.unwrap() - expected, equation.2.clone())
      }
    }
    '/' => {
      if left.is_none() {
        (expected * right.unwrap(), equation.0.clone())
      } else {
        (left.unwrap() / expected, equation.2.clone())
      }
    }
    '=' => {
      // Ignores expected value, because we know it must be 1.
      if left.is_none() {
        (right.unwrap(), equation.0.clone())
      } else {
        (left.unwrap(), equation.2.clone())
      }
    }
    _ => panic!("unknown operand"),
  };
  if recurse_id == "humn" {
    return next_expectation;
  }
  let next_equation = &monkeys[&recurse_id].equation;
  if next_equation.is_none() {
    panic!("humn must be below equations");
  }
  return calc_expectation(monkeys, next_equation.clone().unwrap(), next_expectation);
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
  let root_equation = Equation(
    root.equation.clone().unwrap().0,
    '=',
    root.equation.clone().unwrap().2,
  );
  root.equation = Some(root_equation.clone());
  monkeys_p2.get_mut(&"humn".to_string()).unwrap().value = None;

  println!("Part 1:");
  println!("{}", calc_monkey(&monkeys, "root".to_string()).unwrap());
  println!("Part 2:");
  println!("{}", calc_expectation(&monkeys_p2, root_equation, 0));
}
