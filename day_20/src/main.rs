use std::env;
use std::fs;

use std::collections::LinkedList;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("Problem 1: {}", problem_1(&contents));
    println!("Problem 2: {}", problem_2(&contents));
}

fn parse_input(input: &str) -> Vec<i64> {
    return input.split('\n').filter(|l| l.len() > 0).map(|l| l.parse::<i64>().unwrap()).collect::<Vec<i64>>();
}

fn mix_cipher(data: Vec<i64>, times_to_mix: u32) -> Vec<i64> {
    let mut cipher_mutation = LinkedList::new();
    for (key, d) in data.iter().enumerate() {
        cipher_mutation.push_back((key, d.clone()));
    }

    for _i in 0..times_to_mix {
        for (key, amount_to_shift) in data.iter().enumerate() {
            if amount_to_shift == &0 {
                continue; // nothing to do
            }

            // A
            // B C D E
            // start index is 0
            // 1 -> B; dest index should be 1 (insert before)
            // 2 -> C; dest index should be 2 (C is at 1, D is at 2, so insert before D)
            // 3 -> D
            // 4 (0) -> E (insert before B at index 0)

            // first we find the node
            let index_of_node_needing_moved = cipher_mutation.iter().enumerate().find(|(_i, c)| c.0 == key).map(|(i, _c)| i).unwrap();

            // then we remove the node we're interested in from the set, we'll re-add it later
            let mut nodes = cipher_mutation.split_off(index_of_node_needing_moved);
            nodes.pop_front();
            cipher_mutation.extend(nodes);

            let bounded_amount_to_shift = amount_to_shift % cipher_mutation.len() as i64;

            let destination_index = if bounded_amount_to_shift < 0 && bounded_amount_to_shift.abs() as usize > index_of_node_needing_moved {
                let remainder = (index_of_node_needing_moved as i64 + bounded_amount_to_shift).abs() as usize;
                cipher_mutation.len() - remainder
            } else {
                (((index_of_node_needing_moved as i64) + bounded_amount_to_shift) % cipher_mutation.len() as i64) as usize
            };

            // we now have our index that we're going to insert behind
            let mut destination_node_and_all_after = cipher_mutation.split_off(destination_index);
            destination_node_and_all_after.push_front((key, *amount_to_shift));
            cipher_mutation.extend(destination_node_and_all_after);
        }
    }


    let mixed_data = cipher_mutation.into_iter().map(|(_key, shift)| shift).collect::<Vec<i64>>();

    mixed_data
}

fn find_coordinates(mixed_data: &Vec<i64>) -> i64 {
    let start = mixed_data.iter().enumerate().find(|(_i, d)| *d == &0).map(|(i, _d)| i).unwrap();
    let x = (start + 1000) % (mixed_data.len());
    let y = (start + 2000) % (mixed_data.len());
    let z = (start + 3000) % (mixed_data.len());
    mixed_data[x] + mixed_data[y] + mixed_data[z]
}

fn problem_1(input: &str) -> i64 {
    let numbers = parse_input(&input);
    let mixed = mix_cipher(numbers, 1);

    find_coordinates(&mixed)
}

fn problem_2(input: &str) -> i64 {
    let numbers = parse_input(&input);

    let numbers = numbers.iter().map(|n| n * 811589153).collect::<Vec<i64>>();
    let mixed = mix_cipher(numbers, 10);

    find_coordinates(&mixed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_coordinates_finds() {
        assert_eq!(0, find_coordinates(&vec![0]));
        assert_eq!(0, find_coordinates(&vec![0, 1]));
        assert_eq!(1 + 3 + 0, find_coordinates(&vec![0, 1, 3]));
        assert_eq!(0, find_coordinates(&vec![0, 1, 3, 4]));
        assert_eq!(0, find_coordinates(&vec![1, 0, 3, 4])); // mod == 0
        assert_eq!(0, find_coordinates(&vec![1, 3, 0, 4])); // mod == 0
        assert_eq!(3 + 2 + 1, find_coordinates(&vec![1, 2, 3, 0, 4, 5, 6])); // mod == 7
        assert_eq!(3 + 2 + 1, find_coordinates(&vec![4, 5, 6, 1, 2, 3, 0])); // mod == 7
        assert_eq!(3 + 2 + 1, find_coordinates(&vec![0, 4, 5, 6, 1, 2, 3])); // mod == 7
    }

    #[test]
    fn consecutive_ones_correct() {
        // 1a, 1b, 1c, 3 start
        // 1b, 1a, 1c, 3 1a
        // 1a, 1b, 1c, 3 1b
        // 1a, 1b, 3, 1c 1c
        // 1a, 1b, 3, 1c 3 SAME AS 1c, 1a, 1b, 3 SAME AS 3, 1c, 1a, 1b
        assert_eq!(vec![3, 1, 1, 1], mix_cipher(vec![1, 1, 1, 3], 1));
    }

    #[test]
    fn goes_left_no_wrap_fine() {
        assert_eq!(vec![-2, 0, 0, 0], mix_cipher(vec![0, 0, -2, 0], 1));
        assert_eq!(vec![0, -2, 0, 0], mix_cipher(vec![0, 0, 0, -2], 1));
        assert_eq!(vec![-1, 0, 0, 0], mix_cipher(vec![0, -1, 0, 0], 1));
    }

    #[test]
    fn wraps_left_correct() {
        assert_eq!(vec![0, 0, -1, 0], mix_cipher(vec![0, 0, 0, -1], 1));
        assert_eq!(vec![0, -2, 0, 0], mix_cipher(vec![0, 0, 0, -2], 1));
        assert_eq!(vec![-3, 0, 0, 0], mix_cipher(vec![0, 0, 0, -3], 1));
        assert_eq!(vec![0, 0, -4, 0], mix_cipher(vec![0, 0, 0, -4], 1));
        assert_eq!(vec![0, 0, -3, 0], mix_cipher(vec![0, 0, -3, 0], 1));
        assert_eq!(vec![0, -4, 0, 0], mix_cipher(vec![0, 0, -4, 0], 1));
        assert_eq!(vec![-5, 0, 0, 0], mix_cipher(vec![0, 0, -5, 0], 1));
        assert_eq!(vec![0, 0, -6, 0], mix_cipher(vec![0, 0, -6, 0], 1));
    }

    #[test]
    fn handles_wrapping_fine() {
        assert_eq!(vec![0,0,3,0], mix_cipher(vec![0,0,3,0], 1));
        assert_eq!(vec![0,0,6,0], mix_cipher(vec![0,0,6,0], 1));
        assert_eq!(vec![0,0,9,0], mix_cipher(vec![0,0,9,0], 1));
    }

    #[test]
    fn wraps_right_correct() {
        // A B C D 00
        // B A C D 01
        // B C A D 02
        // B C D A 03 SAME AS A B C D
        // B A C D 04
        assert_eq!(vec![0,1,0,0], mix_cipher(vec![1,0,0,0], 1));
        assert_eq!(vec![0,0,2,0], mix_cipher(vec![2,0,0,0], 1));
        assert_eq!(vec![3,0,0,0], mix_cipher(vec![3,0,0,0], 1));
        assert_eq!(vec![0,4,0,0], mix_cipher(vec![4,0,0,0], 1));
        assert_eq!(vec![0,0,5,0], mix_cipher(vec![5,0,0,0], 1));
        assert_eq!(vec![6,0,0,0], mix_cipher(vec![6,0,0,0], 1));
        assert_eq!(vec![0,7,0,0], mix_cipher(vec![7,0,0,0], 1));
        assert_eq!(vec![0,0,8,0], mix_cipher(vec![8,0,0,0], 1));
        assert_eq!(vec![9,0,0,0], mix_cipher(vec![9,0,0,0], 1));

        assert_eq!(vec![0,0,1,0], mix_cipher(vec![0,1,0,0], 1));
        assert_eq!(vec![2,0,0,0], mix_cipher(vec![0,2,0,0], 1));
        assert_eq!(vec![0,3,0,0], mix_cipher(vec![0,3,0,0], 1));
        assert_eq!(vec![0,0,4,0], mix_cipher(vec![0,4,0,0], 1));
        assert_eq!(vec![5,0,0,0], mix_cipher(vec![0,5,0,0], 1));
        assert_eq!(vec![0,6,0,0], mix_cipher(vec![0,6,0,0], 1));
        assert_eq!(vec![0,0,7,0], mix_cipher(vec![0,7,0,0], 1));
        assert_eq!(vec![8,0,0,0], mix_cipher(vec![0,8,0,0], 1));
        assert_eq!(vec![0,9,0,0], mix_cipher(vec![0,9,0,0], 1));

        assert_eq!(vec![0,1,0,0], mix_cipher(vec![0,0,0,1], 1));
        assert_eq!(vec![0,0,2,0], mix_cipher(vec![0,0,0,2], 1));
        assert_eq!(vec![3,0,0,0], mix_cipher(vec![0,0,0,3], 1));
        assert_eq!(vec![0,4,0,0], mix_cipher(vec![0,0,0,4], 1));
        assert_eq!(vec![0,0,5,0], mix_cipher(vec![0,0,0,5], 1));
        assert_eq!(vec![6,0,0,0], mix_cipher(vec![0,0,0,6], 1));
        assert_eq!(vec![0,7,0,0], mix_cipher(vec![0,0,0,7], 1));
        assert_eq!(vec![0,0,8,0], mix_cipher(vec![0,0,0,8], 1));
        assert_eq!(vec![9,0,0,0], mix_cipher(vec![0,0,0,9], 1));
    }

    #[test]
    fn first() {
        let input = "\
1
2
-3
3
-2
0
4";
        assert_eq!(3, problem_1(&input));
    }

    #[test]
    fn second() {
        let input = "\
1
2
-3
3
-2
0
4";
        assert_eq!(1623178306, problem_2(&input));
    }
}