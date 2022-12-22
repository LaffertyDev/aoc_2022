#![recursion_limit = "10000"]

use std::env;
use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
#[derive(PartialEq)]
enum Operation {
    Add,
    Subtract,
    Divide,
    Multiply
}

impl Operation {
    fn parse(op: &str) -> Operation {
        match op {
            "+" => Operation::Add,
            "-" => Operation::Subtract,
            "*" => Operation::Multiply,
            "/" => Operation::Divide,
            _ => unreachable!()
        }
    }

    fn invert(&self) -> Operation {
        match self {
            Operation::Add => Operation::Subtract,
            Operation::Subtract => Operation::Add,
            Operation::Multiply => Operation::Divide,
            Operation::Divide => Operation::Multiply,
        }
    }

    fn operate(&self, a: i64, b: i64) -> i64 {
        match self {
            Operation::Add => a + b,
            Operation::Subtract => a - b,
            Operation::Multiply => a * b,
            Operation::Divide => a / b,
        }
    }
}

struct MonkeyDependency<'a> {
    monkey_1: &'a str,
    operation: Operation,
    monkey_2: &'a str
}

impl MonkeyDependency<'_> {
    fn get_operation_value(&self, monkeys: &HashMap<&str, Monkey>) -> i64 {
        let monkey_1 = monkeys.get(self.monkey_1).unwrap();
        let monkey_2 = monkeys.get(self.monkey_2).unwrap();

        let monkey_1_result = monkey_1.get_monkey_result(monkeys);
        let monkey_2_result = monkey_2.get_monkey_result(monkeys);

        self.operation.operate(monkey_1_result, monkey_2_result)
    }
}

struct Monkey<'a> {
    id: &'a str,
    shout_value: Option<i64>,
    dependency: Option<MonkeyDependency<'a>>
}

impl Monkey<'_> {
    fn get_monkey_result(&self, monkeys: &HashMap<&str, Monkey>) -> i64 {
        if let Some(shout) = self.shout_value {
            return shout;
        }

        return self.dependency.as_ref().unwrap().get_operation_value(monkeys);
    }

    fn get_value_inverted(&self, monkeys: &HashMap<&str, Monkey>) -> i64 {
        if self.id != "humn" && self.shout_value.is_some() {
            return self.shout_value.unwrap();
        }

        // the child node will figure out what to do here
        if self.id == "root" {
            return 0;
        }

        let parent_node = monkeys.values().find(|m| m.dependency.is_some() && (m.dependency.as_ref().unwrap().monkey_1 == self.id || m.dependency.as_ref().unwrap().monkey_2 == self.id)).unwrap();
        let parent_dep = parent_node.dependency.as_ref().unwrap();
        if parent_dep.monkey_1 == self.id {
            // I am my parents left node
            let sibling = monkeys.get(parent_dep.monkey_2).unwrap();

            // ez math
            // p = self + sib -> self = p - sib
            // p = self * sib -> self = p / sib
            // p = self / sib -> self = p * sib
            // p = self - sib -> self = p + sib
            let inverted_operation = parent_dep.operation.invert();
            let sibling_inverted_value = sibling.get_monkey_result(monkeys);
            if parent_node.id == "root" {
                // special case
                // we only need to return my siblings value
                return sibling_inverted_value
            }

            let parent_inverted_value = parent_node.get_value_inverted(monkeys);
            return inverted_operation.operate(parent_inverted_value, sibling_inverted_value);
        } else {
            // I am my parents right node
            let sibling = monkeys.get(parent_dep.monkey_1).unwrap();
            // slightly more complicated

            // p = sib + self -> self = p - sib
            // p = sib - self -> self = sib - p
            // p = sib * self -> self = p / sib
            // p = sib / self -> self = sib / p
            let sibling_inverted_value = sibling.get_monkey_result(monkeys);
            if parent_node.id == "root" {
                // special case
                // we only need to return my siblings value
                return sibling_inverted_value
            }

            let parent_inverted_value = parent_node.get_value_inverted(monkeys);
            match parent_dep.operation {
                Operation::Add => return Operation::Subtract.operate(parent_inverted_value, sibling_inverted_value),
                Operation::Subtract => return Operation::Subtract.operate(sibling_inverted_value, parent_inverted_value),
                Operation::Multiply => return Operation::Divide.operate(parent_inverted_value, sibling_inverted_value),
                Operation::Divide => return Operation::Divide.operate(sibling_inverted_value, parent_inverted_value),
            };
        }
    }
}

impl Monkey<'_> {
    fn parse_from_input(line: &str) -> Monkey {
        let mut line_iter = line.split(": ");
        let monkey_id = line_iter.next().unwrap();
        let remaining = line_iter.next().unwrap();

        let monkey_shout_value = remaining.parse::<i64>(); // if this fails, that means we 
        
        let monkey_operator = if !monkey_shout_value.is_ok() {
            // parse the operator
            let monkey_1 = &remaining[0..=3];
            let operator_char = &remaining[5..=5];
            let monkey_2 = &remaining[7..=10];
            Some(MonkeyDependency {
                monkey_1: monkey_1,
                operation: Operation::parse(operator_char),
                monkey_2: monkey_2
            })
        } else {
            None
        };

        Monkey {
            id: monkey_id,
            shout_value: monkey_shout_value.ok(),
            dependency: monkey_operator
        }

    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("Problem 1: {}", problem_1(&contents));
    println!("Problem 2: {}", problem_2(&contents));
}

fn parse_input(input: &str) -> HashMap<&str, Monkey> {
    let mut map = HashMap::new();

    for l in input.split('\n').filter(|l| l.len() > 0) {
        let monkey = Monkey::parse_from_input(l);
        map.insert(monkey.id, monkey);
    }

    map
}

fn problem_1(input: &str) -> i64 {
    let monkey_map = parse_input(&input);
    let root_monkey = monkey_map.get("root").unwrap();
    root_monkey.get_monkey_result(&monkey_map)
}

fn problem_2(input: &str) -> i64 {
    let monkey_map = parse_input(&input);
    let human_monkey = monkey_map.get("humn").unwrap();

    human_monkey.get_value_inverted(&monkey_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_monkey_op_parses() {
        let input = "root: pppw + sjmn";
        let monkey = Monkey::parse_from_input(&input);
        assert_eq!("root", monkey.id);
        assert_eq!("pppw", monkey.dependency.as_ref().unwrap().monkey_1);
        assert_eq!("sjmn", monkey.dependency.as_ref().unwrap().monkey_2);
        assert_eq!(Operation::Add, monkey.dependency.as_ref().unwrap().operation);
        assert!(monkey.shout_value.is_none());
    }

    #[test]
    fn parse_monkey_shout_parses() {
        let input = "dbpl: 5";
        let monkey = Monkey::parse_from_input(&input);
        assert_eq!("dbpl", monkey.id);
        assert_eq!(&5, monkey.shout_value.as_ref().unwrap());
        assert!(monkey.dependency.is_none());
    }

    #[test]
    fn first() {
        let input = "\
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
        assert_eq!(152, problem_1(&input));
    }

    #[test]
    fn second() {
        let input = "\
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

        // find which side contains the human
        // compute the value for the other side

        // 


        // human == ptdq + 3
        // human == (lgvd / ljgn) + 3
        // human == (lgvd / 2) + 3
        // human == ((cczh - sllz) / 2) + 3
        // human == ((cczh - 4) / 2) + 3
        // human == (((pppw * lfqf) - 4) / 2) + 3
        // human == (((pppw * 4) - 4) / 2) + 3

        // human == 301

        // humn: (TBD)
        // ptdq: human - dvpt
        // dvpt == 3

        // lgvd = ljgn * ptdq
        // ljgn == 2

        // cczh = sllz + lgvd
        // sllz = 4

        // pppw = cczh / lfqf
        // lfqf: 4


        // pppw == 150
        // root = pppw + sjmn

        // sjmn = 30 * 5
        // sjmn = drzm * dbpl
        // dbpl = 5
        

        // drzm = 30
        // drzm = hmdt - zczc
        // hmdt = 32
        // zczc = 2
        
        // Invert it...
        assert_eq!(301, problem_2(&input));
    }
}