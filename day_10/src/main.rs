use std::env;
use std::fs;

#[derive(Debug)]
#[derive(PartialEq)]
enum Instruction {
    Add(i32),
    Noop
}

enum State {
    Idle,
    Adding(i32)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("Problem 1: {}", problem_1(&contents));
    println!("Problem 2: \n\n{}\n\n", problem_2(&contents));
}

fn problem_1(input: &str,) -> i32 {
    let mut signal_strength = 0;
    let mut register_value = 1;

    let mut program_counter = 0;

    let mut instruction_iter = input.split('\n').map(|l| parse_instruction(l));
    let mut cpu_state = State::Idle;
    loop {
        program_counter += 1;

        if program_counter == 20 || (program_counter > 20 && (program_counter - 20) % 40 == 0) {
            signal_strength += program_counter * register_value;
        }

        match cpu_state {
            State::Idle => {
                match instruction_iter.next() {
                    None => {
                        // program done
                        break;
                    },
                    Some(Instruction::Noop) => {
                        // do nothing
                    },
                    Some(Instruction::Add(val)) => {
                        cpu_state = State::Adding(val);
                    }
                };
            },
            State::Adding(val) => {
                // this is the second tick
                register_value += val;
                cpu_state = State::Idle;
            }
        };
    }

    signal_strength
}

fn problem_2(input: &str) -> String {
    let mut output_display = String::from("");
    let mut register_value = 1;

    let mut program_counter = 0;

    let mut instruction_iter = input.split('\n').map(|l| parse_instruction(l)).peekable();
    let mut cpu_state = State::Idle;
    loop {
        program_counter += 1;

        if instruction_iter.peek().is_none() {
            // program done, exit
            println!("program done");
            break;
        }

        let crt_index_match = (program_counter - 1) % 40;
        
        if program_counter > 1 && crt_index_match == 0 {
            output_display += "\n";
        }
        if crt_index_match >= std::cmp::max(0, register_value - 1) && crt_index_match <= std::cmp::max(0, register_value + 1) {
            output_display += "#";
        } else {
            output_display += ".";
        }

        match cpu_state {
            State::Idle => {
                match instruction_iter.next() {
                    None => {
                        unreachable!();
                    },
                    Some(Instruction::Noop) => {
                        // do nothing
                    },
                    Some(Instruction::Add(val)) => {
                        cpu_state = State::Adding(val);
                    }
                };
            },
            State::Adding(val) => {
                // this is the second tick
                register_value += val;
                cpu_state = State::Idle;
            }
        };
    }

    output_display
}

fn parse_instruction(line: &str) -> Instruction {
    if line.starts_with("noop") {
        return Instruction::Noop;
    } else {
        return Instruction::Add(line.split(' ').last().unwrap().parse::<i32>().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first() {
        let input = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        assert_eq!(13140, problem_1(&input));
    }

    #[test]
    fn parses() {
        assert_eq!(Instruction::Noop, parse_instruction("noop"));
        assert_eq!(Instruction::Add(1), parse_instruction("addx 1"));
        assert_eq!(Instruction::Add(-1), parse_instruction("addx -1"));
    }

    #[test]
    fn second() {
        let input = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        let target = "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....";
        let res = problem_2(&input);
        for (i, tc) in res.chars().enumerate() {
            if target.chars().nth(i).unwrap() != tc {
                panic!("Gross");
            }
        }
        assert_eq!(target, &problem_2(&input));
    }
}