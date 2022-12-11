use std::slice::Iter;

use aoc_2022_rust::aoc::input::lines;

#[derive(Clone)]
enum Operation {
    Add(i64),
    MultiplyBy(i64),
    MultiplyByItself,
}

#[derive(Clone)]
struct Test {
    divisible_by: i64,
    if_true: usize,
    if_false: usize,
}

#[derive(Clone)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    test: Test,
}

type Input = Vec<Monkey>;

fn parse(input: &Vec<String>) -> Input {
    let parse_items = |line: &String| -> Vec<i64> {
        return line
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(',')
            .map(|n| n.trim().parse::<i64>().unwrap())
            .collect();
    };

    let parse_operation = |line: &str| -> Operation {
        if line == "  Operation: new = old * old" {
            return Operation::MultiplyByItself;
        }
        if line.starts_with("  Operation: new = old + ") {
            return Operation::Add(
                line.strip_prefix("  Operation: new = old + ")
                    .unwrap()
                    .parse::<i64>()
                    .unwrap(),
            );
        }
        if line.starts_with("  Operation: new = old * ") {
            return Operation::MultiplyBy(
                line.strip_prefix("  Operation: new = old * ")
                    .unwrap()
                    .parse::<i64>()
                    .unwrap(),
            );
        }
        panic!("Unhandled line: {}", line);
    };

    let parse_test = |it: &mut Iter<String>| -> Test {
        return Test {
            divisible_by: it
                .next()
                .unwrap()
                .strip_prefix("  Test: divisible by ")
                .unwrap()
                .parse()
                .unwrap(),
            if_true: it
                .next()
                .unwrap()
                .strip_prefix("    If true: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap(),
            if_false: it
                .next()
                .unwrap()
                .strip_prefix("    If false: throw to monkey ")
                .unwrap()
                .parse()
                .unwrap(),
        };
    };

    let mut result = Input::new();

    let mut it = input.iter();
    while let Some(line) = it.next() {
        assert!(line.starts_with("Monkey "));

        let items = parse_items(it.next().unwrap());
        let operation = parse_operation(it.next().unwrap());
        let test = parse_test(&mut it);

        result.push(Monkey {
            items,
            operation,
            test,
        });
        it.next(); // empty line
    }

    return result;
}

fn do_operation(
    worry_level: &i64,
    adjust_level: &i64,
    factors: &i64,
    operation: &Operation,
) -> i64 {
    let new_worry_level = match operation {
        Operation::MultiplyByItself => worry_level * worry_level,
        Operation::Add(v) => worry_level + v,
        Operation::MultiplyBy(v) => worry_level * v,
    };
    return (new_worry_level % factors) / adjust_level;
}

fn handle_item(
    item: &i64,
    adjust_level: &i64,
    factors: &i64,
    operation: Operation,
    test: Test,
    monkeys: &mut Input,
) {
    let worry_level = do_operation(item, adjust_level, factors, &operation);
    if worry_level % test.divisible_by == 0 {
        monkeys[test.if_true].items.push(worry_level);
    } else {
        monkeys[test.if_false].items.push(worry_level);
    }
}

fn monkey_business_with_stress(mut monkeys: Input, stress: i64, iterations: i32) -> usize {
    let mut monkey_inspections = vec![0; monkeys.len()];
    let mut all_factors = 1;
    for m in &monkeys {
        all_factors *= m.test.divisible_by;
    }

    for _ in 0..iterations {
        for m in 0..monkeys.len() {
            monkey_inspections[m] += monkeys[m].items.len();
            for item in &monkeys[m].items.clone() {
                handle_item(
                    item,
                    &stress,
                    &all_factors,
                    monkeys[m].operation.clone(),
                    monkeys[m].test.clone(),
                    &mut monkeys,
                )
            }
            monkeys[m].items.clear();
        }
    }

    monkey_inspections.sort_by(|a, b| b.cmp(a));
    return monkey_inspections[0] * monkey_inspections[1];
}

fn first(monkeys: Input) -> usize {
    return monkey_business_with_stress(monkeys, 3, 20);
}

fn second(monkeys: Input) -> usize {
    return monkey_business_with_stress(monkeys, 1, 10000);
}

fn main() {
    let parsed = parse(&lines());
    println!("first = {}", first(parsed.clone()));
    println!("second = {}", second(parsed));
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
    fn test_first() {
        let input = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(10605, first(input));
    }

    #[test]
    fn test_second() {
        let input = parse(&INPUT.lines().map(|l| l.to_string()).collect());
        assert_eq!(2713310158, second(input));
    }
}
