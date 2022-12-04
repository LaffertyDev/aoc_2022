use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let first_problem_contents = contents.clone();
    let duplicates = first_problem_contents.split('\n').map(|rucksack| find_duplicate(rucksack));
    let duplicate_values = duplicates.map(|d| get_value(d.unwrap()));

    let second_problem_contents = contents.clone();
    let rucksacks = second_problem_contents.split('\n').collect::<Vec<&str>>();

    println!("Problem 1: {}", duplicate_values.sum::<u32>());
    println!("Problem 2: {}", find_all_badges(rucksacks, 3));
}


fn find_duplicate(rucksack: &str) -> Option<char> {
    if rucksack.len()%2 == 1 {
        panic!("uneven rucksack");
    }

    let pouch1 = rucksack.chars().take(rucksack.len() / 2);
    let pouch2 = rucksack.chars().rev().take(rucksack.len() / 2);

    let mut pouch1 = pouch1.collect::<Vec<char>>();
    pouch1.sort();
    let mut pouch2 = pouch2.collect::<Vec<char>>();
    pouch2.sort();

    // could be made faster... O(N^2)
    for i in 0..pouch1.len() {
        for j in 0..pouch2.len() {
            if pouch1[i] == pouch2[j] {
                return Some(pouch1[i]);
            }
        }
    }

    println!("No duplicates found: {}", rucksack);

    return None;
}

fn find_all_badges(rucksacks: Vec<&str>, group_size: usize) -> u32 {
    if rucksacks.len() % group_size != 0 {
        panic!("Invalid rucksack group");
    }

    let mut badge_values = 0;
    for i in (0..rucksacks.len()).step_by(group_size) {
        badge_values += find_badge(&rucksacks[i..i+group_size]);
    }

    return badge_values;
}

fn find_badge(rucksack_group: &[&str]) -> u32 {
    let mut group_detection = vec![[false; 52]; rucksack_group.len()];
    for (i, group) in rucksack_group.iter().enumerate() {
        for item in group.chars() {
            let val = get_value(item);
            group_detection[i][val as usize - 1] = true;
        }
    }

    for i in 0..52 {
        let mut all_good = true;
        for x in 0..rucksack_group.len() {
            all_good &= group_detection[x][i];
        }

        if all_good {
            return (i + 1).try_into().unwrap();
        }
    }

    panic!("Did not find a pair");
}

fn get_value(item: char) -> u32 {
    let raw_ascii_val = item as u32;
    if raw_ascii_val > 96 { return raw_ascii_val - 97 + 1 };
    return raw_ascii_val - 65 + 26 + 1;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn finds_duplicates() {
        assert_eq!(None, find_duplicate("abcdef"));
        assert_eq!(Some('c'), find_duplicate("abcdec"));
        assert_eq!(Some('a'), find_duplicate("aaadea"));
        assert_eq!(Some('p'), find_duplicate("vJrwpWtwJgWrhcsFMMfFFhFp"));
        assert_eq!(Some('L'), find_duplicate("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"));
    }

    #[test]
    fn test_get_value() {
        assert_eq!(1, get_value('a'));
        assert_eq!(2, get_value('b'));
        assert_eq!(27, get_value('A'));
        assert_eq!(28, get_value('B'));
        assert_eq!(52, get_value('Z'));
    }

    #[test]
    fn find_badge() {
        let data = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(get_value('Z'), find_all_badges(data.split('\n').collect::<Vec<&str>>(), 3));
    }

    #[test]
    fn find_badge_simple() {
        let data = "Z\nZ\nZ";
        assert_eq!(get_value('Z'), find_all_badges(data.split('\n').collect::<Vec<&str>>(), 3));
    }
}