use std::env;
use std::fs;

#[derive(Debug)]
#[derive(PartialEq)]
enum MonkeyOperationType {
    Multiply,
    Add,
    Subtract,
    Divide
}

#[derive(Debug)]
#[derive(PartialEq)]
enum MonkeyOperation {
    Old,
    Hardset(u64)
}

#[derive(Debug)]
struct MonkeyTest {
    value: u64,
    true_target: u64,
    false_target: u64
}

#[derive(Debug)]
#[allow(dead_code)]
struct Monkey {
    index: u64,
    items: Vec<u64>,
    items_inspected: u64,
    operation: MonkeyOperation,
    operation_type: MonkeyOperationType,
    test: MonkeyTest
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("Problem 1: {}", problem_1(&contents));
    println!("Problem 2: {}", problem_2(&contents));
}

fn problem_1(input: &str,) -> u64 {
    let mut monkeys = parse_input(input);

    for _i in 0..20 {
        for current_monkey_idx in 0..monkeys.len() {
            let current_monkey = monkeys.get_mut(current_monkey_idx).unwrap();
            let mut items_to_move = vec![];
            for item in &current_monkey.items {
                let old_stress_value = item;
                let stress_target = match current_monkey.operation {
                    MonkeyOperation::Old => old_stress_value.clone(),
                    MonkeyOperation::Hardset(target) => target.clone()
                };

                let new_stress = match current_monkey.operation_type {
                    MonkeyOperationType::Multiply => old_stress_value * stress_target,
                    MonkeyOperationType::Divide => old_stress_value / stress_target,
                    MonkeyOperationType::Add => old_stress_value + stress_target,
                    MonkeyOperationType::Subtract => old_stress_value - stress_target,
                };

                let bored_stress_value = new_stress / 3;

                let item_target = if bored_stress_value % current_monkey.test.value == 0 { current_monkey.test.true_target } else { current_monkey.test.false_target };
                items_to_move.push((item_target, bored_stress_value.clone()));
                current_monkey.items_inspected += 1;
            }

            // move items
            current_monkey.items.drain(..);
            for item in items_to_move {
                monkeys[item.0 as usize].items.push(item.1);
            }
        }
    }

    let mut items_collected = monkeys.iter().map(|m| m.items_inspected).collect::<Vec<u64>>();
    items_collected.sort_by(|a, b| b.cmp(a));
    return items_collected[0] * items_collected[1];
}

fn problem_2(input: &str) -> u64 {
    let mut monkeys = parse_input(input);
    let largest_diviser: u64 = monkeys.iter().map(|m| m.test.value).product();

    for _i in 0..10000 {
        for current_monkey_idx in 0..monkeys.len() {
            let current_monkey = monkeys.get_mut(current_monkey_idx).unwrap();
            let mut items_to_move = vec![];
            for item in &current_monkey.items {
                let old_stress_value = item;
                let stress_target = match current_monkey.operation {
                    MonkeyOperation::Old => old_stress_value.clone(),
                    MonkeyOperation::Hardset(target) => target.clone()
                };

                let new_stress = match current_monkey.operation_type {
                    MonkeyOperationType::Multiply => old_stress_value * stress_target % largest_diviser,
                    MonkeyOperationType::Divide => old_stress_value / stress_target % largest_diviser,
                    MonkeyOperationType::Add => old_stress_value + stress_target % largest_diviser,
                    MonkeyOperationType::Subtract => old_stress_value - stress_target % largest_diviser,
                };

                let item_target = if new_stress % current_monkey.test.value == 0 { current_monkey.test.true_target } else { current_monkey.test.false_target };
                items_to_move.push((item_target, new_stress.clone()));
                current_monkey.items_inspected += 1;
            }

            // move items
            current_monkey.items.drain(..);
            for item in items_to_move {
                monkeys[item.0 as usize].items.push(item.1);
            }
        }
    }

    let mut items_collected = monkeys.iter().map(|m| m.items_inspected).collect::<Vec<u64>>();
    items_collected.sort_by(|a, b| b.cmp(a));
    return items_collected[0] * items_collected[1];
}

fn parse_input(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(|ml| parse_monkey(ml)).collect::<Vec<Monkey>>()
}

fn parse_monkey(input: &str) -> Monkey {
    let mut line_iter = input.split('\n');
    let monkey_index = line_iter.next().unwrap().split(' ').last().unwrap().replace(':', "").parse::<u64>().unwrap();
    let starting_items = line_iter.next().unwrap().split("Starting items: ").last().unwrap().split(", ").map(|i| i.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let mut operation = line_iter.next().unwrap().split("Operation: new = old ").last().unwrap().split(' ');
    let operator = operation.next().unwrap();
    let operator_type = match operator {
        "*" => MonkeyOperationType::Multiply,
        "/" => MonkeyOperationType::Divide,
        "+" => MonkeyOperationType::Add,
        "-" => MonkeyOperationType::Subtract,
        _ => panic!()
    };
    let operator_expression = operation.next().unwrap();
    let operator_expression = if operator_expression == "old" { MonkeyOperation::Old } else { MonkeyOperation::Hardset(operator_expression.parse::<u64>().unwrap()) };

    let test_value = line_iter.next().unwrap().split("Test: divisible by ").last().unwrap().parse::<u64>().unwrap();
    let true_test_target = line_iter.next().unwrap().split("If true: throw to monkey ").last().unwrap().parse::<u64>().unwrap();
    let false_test_target = line_iter.next().unwrap().split("If false: throw to monkey ").last().unwrap().parse::<u64>().unwrap();

    return Monkey {
        index: monkey_index,
        items: starting_items,
        items_inspected: 0,
        operation: operator_expression,
        operation_type: operator_type,
        test: MonkeyTest {
            value: test_value,
            true_target: true_test_target,
            false_target: false_test_target
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses() {
        let input = "\
Monkey 0:
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
        let monkeys = parse_input(&input);
        let test_monkey = &monkeys[0];
        assert_eq!(4, monkeys.len());
        assert_eq!(MonkeyOperation::Old, monkeys[2].operation);
        assert_eq!(0, test_monkey.index);
        assert_eq!(MonkeyOperationType::Multiply, test_monkey.operation_type);
        assert_eq!(MonkeyOperation::Hardset(19), test_monkey.operation);
        assert_eq!(79, test_monkey.items[0]);
        assert_eq!(98, test_monkey.items[1]);
        assert_eq!(23, test_monkey.test.value);
        assert_eq!(2, test_monkey.test.true_target);
        assert_eq!(3, test_monkey.test.false_target);
    }

    #[test]
    fn first() {
        let input = "\
Monkey 0:
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
        assert_eq!(10605, problem_1(&input));
    }

    #[test]
    fn second() {
        let input = "\
Monkey 0:
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
        assert_eq!(2713310158, problem_2(&input));
    }
}