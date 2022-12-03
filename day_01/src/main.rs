use std::env;
use std::fs;
use std::vec;

// Read all lines in the input, one-by-one, adding the total number of calories until a new line is found
// Then find the max so far, and return it
fn main() {
    // In case the Elves get hungry and need extra snacks, they need to know which Elf to ask:
    // they'd like to know how many Calories are being carried by the Elf carrying the most Calories.
    // In the example above, this is 24000 (carried by the fourth Elf).

    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("Problem #1: {}", get_highest_calories(&contents));
    let top_calories = get_top_calories(&contents, 3);
    println!("Problem #2: {:?}:{}", top_calories, top_calories.iter().sum::<u32>())
}

fn get_highest_calories(contents: &str) -> u32 {
    let mut current_elf_contents = 0u32;
    let mut max_elf_contents = 0u32;
    for line in contents.split('\n') {
        if line == "" {
            // done parsing this elfs contents
            max_elf_contents = std::cmp::max(current_elf_contents, max_elf_contents);
            current_elf_contents = 0;
        } else {
            current_elf_contents += line.parse::<u32>().unwrap();
        }
    }

    max_elf_contents
}

fn get_top_calories(contents: &str, count: usize) -> Vec<u32> {
    let mut elf_contents = get_calories_per_elf(contents);
    elf_contents.sort_unstable();

    let mut top_elfs = vec![0; count];
    if elf_contents.len() < count {
        return vec![]
    }

    top_elfs[0..count].clone_from_slice(&elf_contents[elf_contents.len()-count..elf_contents.len()]);
    top_elfs
}

fn get_calories_per_elf(contents: &str) -> Vec<u32> {
    let mut elf_calories: Vec<u32> = vec![];
    let mut current_elf_contents = 0;
    for line in contents.split('\n') {
        if line == "" {
            // done parsing this elfs contents
            elf_calories.push(current_elf_contents);
            current_elf_contents = 0;
        } else {
            current_elf_contents += line.parse::<u32>().unwrap();
        }
    }

    return elf_calories;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_single_equals() {
        let data = "1\n";

        assert_eq!(1u32, get_highest_calories(data))
    }

    #[test]
    fn part1_double_adds() {
        let data = "1\n1\n";

        assert_eq!(2u32, get_highest_calories(data))
    }

    #[test]
    fn part1_gets_highest_second() {
        let data = "1\n\n2\n";

        assert_eq!(2u32, get_highest_calories(data))
    }

    #[test]
    fn part1_gets_highest_first() {
        let data = "2\n\n1\n";

        assert_eq!(2u32, get_highest_calories(data))
    }

    #[test]
    fn part1_compares_multiple_elfs() {
        let data = "2\n\n1\n1\n1\n";

        assert_eq!(3u32, get_highest_calories(data))
    }

    #[test]
    fn part2_gets_highest_count() {
        let data = "1\n\n2\n\n3\n\n4\n\n5\n\n";
        assert_eq!(vec![3,4,5], get_top_calories(data, 3))
    }

    #[test]
    fn part2_notenough_returns0() {
        let data = "1\n";
        assert_eq!(Vec::<u32>::new(), get_top_calories(data, 3))
    }
}
