use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("Problem 1: {}", problem_1(&contents));
    println!("Problem 2: {}", problem_2(&contents));
}

fn problem_1(_input: &str) -> u32 {
    0
}

fn problem_2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first() {
        let input = "";
        assert_eq!(0, problem_1(&input));
    }

    #[test]
    fn second() {
        let input = "";
        assert_eq!(0, problem_2(&input));
    }
}