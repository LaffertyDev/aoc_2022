use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("Problem 1: {}", find_stream(&contents, 4));
    println!("Problem 2: {}", find_stream(&contents, 14));
}

fn find_stream(input: &str, amount_looking_for: usize) -> usize {
    let input_mapped = input.chars().collect::<Vec<char>>();

    for i in 0..input_mapped.len() {
        if HashSet::<&char>::from_iter(&input_mapped[i..i+amount_looking_for]).len() == amount_looking_for {
            return i + amount_looking_for;
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(5, find_stream(&input, 4));
    }

    #[test]
    fn second() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(11, find_stream(&input, 4));
    }
}