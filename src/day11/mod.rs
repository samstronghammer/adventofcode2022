use crate::util::advent;
use eval::Expr;

#[derive(Clone)]
struct Monkey {
  items: Vec<i64>,
  operation: Expr,
  test_divisible: i64,
  true_monkey: i64,
  false_monkey: i64,
  num_inspections: i64,
}

// The Eval operation was very slow, so I hard coded the functions.
// It still works if you set optimized to false for monkey_business.
fn monkey_operation(index: usize, value: i64) -> i64 {
  match index {
    0 => value * 13,
    1 => value + 2,
    2 => value + 6,
    3 => value * value,
    4 => value + 3,
    5 => value * 7,
    6 => value + 4,
    7 => value + 7,
    _ => panic!("Unknown monkey index"),
  }
}

fn monkey_business(mut monkeys: Vec<Monkey>, num_rounds: u32, part1: bool, optimized: bool) -> i64 {
  let num_monkeys = monkeys.len();
  let safe_mod: i64 = monkeys.iter().map(|x| x.test_divisible).product();

  for _ in 0..num_rounds {
    for j in 0..num_monkeys {
      let mut worry_levels = [].to_vec();
      let mut goal_monkeys = [].to_vec();
      {
        let monkey = &mut monkeys[j];
        for item in &monkey.items {
          let mut worry_level = *item;
          if optimized {
            worry_level = monkey_operation(j, worry_level);
          } else {
            worry_level = monkey
              .operation
              .clone()
              .value("old", worry_level)
              .exec()
              .unwrap()
              .as_i64()
              .unwrap();
          }
          if part1 {
            worry_level /= 3;
          } else {
            worry_level %= safe_mod;
          }
          worry_levels.push(worry_level);
          if worry_level % monkey.test_divisible == 0 {
            goal_monkeys.push(monkey.true_monkey);
          } else {
            goal_monkeys.push(monkey.false_monkey);
          }
          monkey.num_inspections += 1;
        }
        monkey.items.clear();
      }
      for k in 0..worry_levels.len() {
        monkeys[goal_monkeys[k] as usize]
          .items
          .push(worry_levels[k]);
      }
    }
  }
  let mut inspections: Vec<i64> = monkeys.iter().map(|m| m.num_inspections).collect();
  inspections.sort();
  return inspections.iter().rev().take(2).product();
}

pub fn run() {
  let contents = advent::parse_input(11);
  let raw_monkeys: Vec<&str> = contents.split("\n\n").collect();
  let mut monkeys: Vec<Monkey> = [].to_vec();
  for raw_monkey in raw_monkeys {
    let lines: Vec<&str> = raw_monkey.split("\n").collect();
    let starting_toks: Vec<&str> = lines[1].split([',', ' ']).collect();
    let mut starting_items: Vec<i64> = [].to_vec();
    for start_tok in starting_toks {
      if start_tok.len() > 0 && start_tok.len() < 3 {
        starting_items.push(i64::from_str_radix(start_tok, 10).unwrap());
      }
    }
    let operation_toks: Vec<&str> = lines[2].split(" = ").collect();
    let test_divisible_toks: Vec<&str> = lines[3].split(" ").collect();
    let true_toks: Vec<&str> = lines[4].split(" ").collect();
    let false_toks: Vec<&str> = lines[5].split(" ").collect();

    monkeys.push(Monkey {
      items: starting_items,
      operation: Expr::new(String::from(operation_toks[1])),
      test_divisible: i64::from_str_radix(test_divisible_toks.last().unwrap(), 10).unwrap(),
      true_monkey: i64::from_str_radix(true_toks.last().unwrap(), 10).unwrap(),
      false_monkey: i64::from_str_radix(false_toks.last().unwrap(), 10).unwrap(),
      num_inspections: 0,
    })
  }

  println!("Part 1:");
  println!("{}", monkey_business(monkeys.clone(), 20, true, true));
  println!("Part 2:");
  println!("{}", monkey_business(monkeys.clone(), 10000, false, true));
}
