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

fn parse_input(input: &str) -> Vec<i32> {
    return input.split('\n').filter(|l| l.len() > 0).map(|l| l.parse::<i32>().unwrap()).collect::<Vec<i32>>();
}

fn mix_cipher(data: Vec<i32>) -> Vec<i32> {
    let mut cipher_mutation = LinkedList::new();
    for (key, d) in data.iter().enumerate() {
        cipher_mutation.push_back((key, d.clone()));
    }

    for (key, amount_to_shift) in data.iter().enumerate() {
        let bounded_amount_to_shift = amount_to_shift % (data.len() as i32);

        // first we find the node
        let index_of_node_needing_moved = cipher_mutation.iter().enumerate().find(|(_i, c)| c.0 == key).map(|(i, _c)| i).unwrap();

        // a true modulo command would be useful here
        let destination_index = if bounded_amount_to_shift.is_negative() && (bounded_amount_to_shift.abs() as usize) > index_of_node_needing_moved {
            // we wrapped to the left
            data.len() - ((bounded_amount_to_shift + index_of_node_needing_moved as i32).abs() as usize % (data.len()))
        } else {
            (bounded_amount_to_shift + index_of_node_needing_moved as i32).abs() as usize % (data.len())
        };

        if index_of_node_needing_moved == destination_index {
            // do nothing
            continue;
        }

        let is_going_right = amount_to_shift.is_positive();

        let removal_index_offset;
        if is_going_right {
            // we're going to the RIGHT, so nodes we replace will be to the LEFT
            if destination_index == 0 {
                // what is current in front needs to be put back to the back
                // because w're going right, it swaps with the final element
                let front_node = cipher_mutation.pop_front().unwrap();
                cipher_mutation.push_front((key, *amount_to_shift));
                cipher_mutation.push_back(front_node);
                removal_index_offset = 0; // zero because we swapped
            } else if destination_index == cipher_mutation.len() - 1 {
                // because we're going to the right, the final element just gets swapped
                cipher_mutation.push_back((key, *amount_to_shift));
                removal_index_offset = 0; // 0 because we're just pushing into the back
            } else {
                // we're going in the middle
                // the node we're targetting needs to be BEHIND us
                let mut destination_node_and_all_after = cipher_mutation.split_off(destination_index);
                let front_node = destination_node_and_all_after.pop_front().unwrap();
                destination_node_and_all_after.push_front((key, *amount_to_shift));
                destination_node_and_all_after.push_front(front_node);
                cipher_mutation.extend(destination_node_and_all_after);
                if destination_index < index_of_node_needing_moved {
                    // move the front node to the back
                    // doesn't matter, but makes tests easier to reason about
                    let front = cipher_mutation.pop_front().unwrap();
                    cipher_mutation.push_back(front);
                }

                removal_index_offset = 0;
            }
        } else {
            // we're going to the LEFT, so nodes we replace will be to the RIGHT of me
            if destination_index == 0 {
                // if we're approaching from the right, then this element needs to be moved to the back
                // we can simply push back, removal index is incremented by one
                cipher_mutation.push_front((key, *amount_to_shift));
                removal_index_offset = 1;
            } else if destination_index == cipher_mutation.len() - 1 {
                // we're approaching this from the right
                let back_element = cipher_mutation.pop_back().unwrap();
                cipher_mutation.push_back((key, *amount_to_shift));
                cipher_mutation.push_front(back_element);
                removal_index_offset = 1;
            } else {
                // we're going in the middle
                // the node we're targetting needs to be BEHIND us
                let mut destination_node_and_all_after = cipher_mutation.split_off(destination_index);
                destination_node_and_all_after.push_front((key, *amount_to_shift));
                cipher_mutation.extend(destination_node_and_all_after);
                if destination_index > index_of_node_needing_moved {
                    // move the end to the beginning
                    // it doesn't actually matter but it makes tests easier to reason about
                    let back = cipher_mutation.pop_back().unwrap();
                    cipher_mutation.push_front(back);
                }

                removal_index_offset = 1;
            }
        }

        let index_needing_removed_after_shift = index_of_node_needing_moved + removal_index_offset % cipher_mutation.len();

        // Remove the element from the buffer
        if index_needing_removed_after_shift == 0 {
            // no stitching. Remove the node from the front
            // Then handle the destination
            cipher_mutation.pop_front();
        } else if index_needing_removed_after_shift == cipher_mutation.len() - 1 {
            // we're at the end of the array
            // no stitching, just pop off the element
            cipher_mutation.pop_back();
        } else {
            // we're in the middle
            // remove the element, stitch the arrays,
            let mut current_node_and_all_after = cipher_mutation.split_off(index_needing_removed_after_shift);
            current_node_and_all_after.pop_front();
            cipher_mutation.extend(current_node_and_all_after);
        }

     //   println!("{:?}", cipher_mutation.iter().map(|(_key, shift)| shift).collect::<Vec<&i32>>());
    }

    let mixed_data = cipher_mutation.into_iter().map(|(_key, shift)| shift).collect::<Vec<i32>>();

    mixed_data
}

fn find_coordinates(mixed_data: &Vec<i32>) -> i32 {
    let start = mixed_data.iter().enumerate().find(|(_i, d)| *d == &0).map(|(i, _d)| i).unwrap();
    let x = (start + 1000) % (mixed_data.len());
    let y = (start + 2000) % (mixed_data.len());
    let z = (start + 3000) % (mixed_data.len());
    mixed_data[x] + mixed_data[y] + mixed_data[z]
}

fn problem_1(input: &str) -> i32 {
    let numbers = parse_input(&input);
    let mixed = mix_cipher(numbers);

    find_coordinates(&mixed)
}

fn problem_2(_input: &str) -> u32 {
    0
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
        // 1a, 3, 1b, 1c
        assert_eq!(vec![1, 3, 1, 1], mix_cipher(vec![1, 1, 1, 3]));
    }

    #[test]
    fn goes_left_no_wrap_fine() {
        assert_eq!(vec![-2, 0, 0, 0], mix_cipher(vec![0, 0, -2, 0]));
        assert_eq!(vec![0, -2, 0, 0], mix_cipher(vec![0, 0, 0, -2]));
        assert_eq!(vec![-1, 0, 0, 0], mix_cipher(vec![0, -1, 0, 0]));
    }

    #[test]
    fn wraps_left_correct() {
        assert_eq!(vec![0, 0, 0, -3], mix_cipher(vec![0, 0, -3, 0]));
        assert_eq!(vec![0, 0, -3, 0], mix_cipher(vec![0, -3, 0, 0]));
        assert_eq!(vec![0, -3, 0, 0], mix_cipher(vec![-3, 0, 0, 0]));
        assert_eq!(vec![-4, 0, 0, 0], mix_cipher(vec![-4, 0, 0, 0]));
        assert_eq!(vec![0, -4, 0, 0], mix_cipher(vec![0, -4, 0, 0]));
        assert_eq!(vec![0, 0, 0, -4], mix_cipher(vec![0, 0, 0, -4]));
        assert_eq!(vec![0, 0, 0, -7], mix_cipher(vec![0, 0, -7, 0]));
        assert_eq!(vec![0, 0, -7, 0], mix_cipher(vec![0, -7, 0, 0]));
        assert_eq!(vec![0, -7, 0, 0], mix_cipher(vec![-7, 0, 0, 0]));
    }

    #[test]
    fn handles_wrapping_fine() {
        assert_eq!(vec![0,0,4,0], mix_cipher(vec![0,0,4,0]));
        assert_eq!(vec![0,0,8,0], mix_cipher(vec![0,0,8,0]));
        assert_eq!(vec![0,0,12,0], mix_cipher(vec![0,0,12,0]));
    }

    #[test]
    fn wraps_right_correct() {
        assert_eq!(vec![0,0,0,5], mix_cipher(vec![0,0,5,0]));
        assert_eq!(vec![0,0,0,9], mix_cipher(vec![0,0,9,0]));
        assert_eq!(vec![0,0,0,13], mix_cipher(vec![0,0,13,0]));
        assert_eq!(vec![14,0,0,0], mix_cipher(vec![0,0,14,0]));
        assert_eq!(vec![0,15,0,0], mix_cipher(vec![0,0,15,0]));
        assert_eq!(vec![0,0,0,4], mix_cipher(vec![0,0,0,4]));
        assert_eq!(vec![5,0,0,0], mix_cipher(vec![0,0,0,5]));
        assert_eq!(vec![0,6,0,0], mix_cipher(vec![0,0,0,6]));
        assert_eq!(vec![4,0,0,0], mix_cipher(vec![4,0,0,0]));
        assert_eq!(vec![0,5,0,0], mix_cipher(vec![5,0,0,0]));
        assert_eq!(vec![0,0,6,0], mix_cipher(vec![6,0,0,0]));
        assert_eq!(vec![0,0,0,7], mix_cipher(vec![7,0,0,0]));
    }

    #[test]
    fn test_input_wraps_correctly () {
        assert_eq!(vec![1, 2, -3, 4, 0, 3, -2], mix_cipher(vec![1,2,-3,3,-2,0,4]));
    }

    #[test]
    fn wraps() {
        assert_eq!(vec![1, 2, -3, 4, 7, 3, -2], mix_cipher(vec![1,2,-3,3,-2,7,4]));
        assert_eq!(vec![1, 2, -3, 4, -7, 3, -2], mix_cipher(vec![1,2,-3,3,-2,-7,4]));
        assert_eq!(vec![1, 2, -3, 4, -14, 3, -2], mix_cipher(vec![1,2,-3,3,-2,-14,4]));
        assert_eq!(vec![1], mix_cipher(vec![1]));
        assert_eq!(vec![2], mix_cipher(vec![2]));
        assert_eq!(vec![2, 1], mix_cipher(vec![1, 2]));
        assert_eq!(vec![2, -1], mix_cipher(vec![-1, 2]));
        assert_eq!(vec![-2, 1], mix_cipher(vec![1, -2]));
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
        let input = "";
        assert_eq!(0, problem_2(&input));
    }
}