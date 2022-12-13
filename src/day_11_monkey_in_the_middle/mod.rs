use std::fs;

use itertools::Itertools;

enum Operator {
    Add,
    Multiply,
}

enum Operand {
    Number(u64),
    Old,
}

struct Monkey {
    items: Vec<u64>,
    operator: Operator,
    operand: Operand,
    test_division: u64,
    test_true_monkey: usize,
    test_false_monkey: usize,
    inspection_count: u64,
}

impl Monkey {
    fn new(input: &str) -> Monkey {
        Monkey {
            items: input
                .lines()
                .nth(1)
                .unwrap()
                .get(18..)
                .unwrap()
                .split(", ")
                .map(|item| item.parse::<u64>().unwrap())
                .collect(),
            operator: match input.lines().nth(2).unwrap().chars().nth(23) {
                Some('*') => Operator::Multiply,
                Some('+') => Operator::Add,
                _ => panic!("unknown operator"),
            },
            operand: match input
                .lines()
                .nth(2)
                .unwrap()
                .get(25..)
                .unwrap()
                .parse::<u64>()
            {
                Ok(n) => Operand::Number(n),
                _ => Operand::Old,
            },
            test_division: input
                .lines()
                .nth(3)
                .unwrap()
                .get(21..)
                .unwrap()
                .parse::<u64>()
                .unwrap(),
            test_true_monkey: input
                .lines()
                .nth(4)
                .unwrap()
                .chars()
                .last()
                .unwrap()
                .to_digit(10)
                .unwrap() as usize,
            test_false_monkey: input
                .lines()
                .nth(5)
                .unwrap()
                .chars()
                .last()
                .unwrap()
                .to_digit(10)
                .unwrap() as usize,
            inspection_count: 0,
        }
    }

    fn inspect_item(&mut self, common_modulo: Option<u64>) -> (u64, usize) {
        self.inspection_count += 1;

        let mut item = self.items.remove(0);
        let operand = match self.operand {
            Operand::Number(n) => n,
            Operand::Old => item,
        };

        item = match self.operator {
            Operator::Add => item + operand,
            Operator::Multiply => item * operand,
        };

        item = match common_modulo {
            Some(n) => item % n,
            None => item / 3,
        };

        if item % self.test_division == 0 {
            (item, self.test_true_monkey)
        } else {
            (item, self.test_false_monkey)
        }
    }
}

pub fn part1(input: &str) -> u64 {
    let mut monkeys: Vec<Monkey> = vec![];

    input.split("\n\n").collect_vec().iter().for_each(|x| {
        monkeys.push(Monkey::new(x));
    });

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while monkeys.iter().nth(i).unwrap().items.len() > 0 {
                let (item, target_monkey) = monkeys.iter_mut().nth(i).unwrap().inspect_item(None);

                monkeys
                    .iter_mut()
                    .nth(target_monkey)
                    .unwrap()
                    .items
                    .push(item.into());
            }
        }
    }

    let mut inspection_counts = monkeys
        .iter()
        .map(|x| x.inspection_count)
        .collect::<Vec<u64>>();

    inspection_counts.sort_by(|a, b| b.cmp(&a));

    inspection_counts.iter().nth(0).unwrap() * inspection_counts.iter().nth(1).unwrap()
}

pub fn part2(input: &str) -> u64 {
    let mut monkeys: Vec<Monkey> = vec![];

    input.split("\n\n").collect_vec().iter().for_each(|x| {
        monkeys.push(Monkey::new(x));
    });

    let common_modulo = monkeys
        .iter()
        .map(|monkey| monkey.test_division)
        .reduce(|a, b| a * b);

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while monkeys.iter().nth(i).unwrap().items.len() > 0 {
                let (item, target_monkey) = monkeys
                    .iter_mut()
                    .nth(i)
                    .unwrap()
                    .inspect_item(common_modulo);

                monkeys
                    .iter_mut()
                    .nth(target_monkey)
                    .unwrap()
                    .items
                    .push(item.into());
            }
        }
    }

    let mut inspection_counts = monkeys
        .iter()
        .map(|x| x.inspection_count)
        .collect::<Vec<u64>>();

    inspection_counts.sort_by(|a, b| b.cmp(&a));

    inspection_counts.iter().nth(0).unwrap() * inspection_counts.iter().nth(1).unwrap()
}

pub fn solve() {
    let input = fs::read_to_string("src/day_11_monkey_in_the_middle/input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
      If true: throw to monkey 2
      If false: throw to monkey 3

  Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
      If true: throw to monkey 2
      If false: throw to monkey 0

  Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
      If true: throw to monkey 1
      If false: throw to monkey 3

  Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
      If true: throw to monkey 0
      If false: throw to monkey 1";

    #[test]
    fn part1_works() {
        let result = part1(INPUT);
        assert_eq!(result, 10605);
    }

    #[test]
    fn part2_works() {
        let result = part2(INPUT);
        assert_eq!(result, 2713310158);
    }
}
