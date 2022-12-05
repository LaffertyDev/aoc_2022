use std::env;
use std::fs;

#[derive(Debug)]
struct Orders {
    amount_to_move: u32,
    start_stack: usize,
    end_stack: usize
}

#[derive(Debug)]
#[derive(Clone)]
struct SortedCargo {
    cargo: Vec<char>
}

fn get_two_mut<T>(slice: &mut [T], index1: usize, index2: usize) -> (&mut T, &mut T) {
    assert!(index1 != index2 && index1 < slice.len() && index2 < slice.len());
    if index1 < index2 {
        let (start, end) = slice.split_at_mut(index2);
        (&mut start[index1], &mut end[0])
    } else {
        let (start, end) = slice.split_at_mut(index1);
        (&mut end[0], &mut start[index2])
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let (c, i) = parse_starter_stacks(&contents);

    let new_cargo_stack = sort_boxes(c.clone(), &i);
    let bulk_cargo_stack = sort_boxes_bulk(c.clone(), &i);

    println!("Problem 1: {}", get_top(new_cargo_stack));
    println!("Problem 2: {}", get_top(bulk_cargo_stack));
}

fn parse_starter_stacks(input: &str) -> (Vec<SortedCargo>, Vec<Orders>) {
    let mut filtered_input = input.split("\n\n");
    let start_set = filtered_input.next().unwrap();
    let move_set = filtered_input.next().unwrap();

    let split_start_set: Vec<&str> = start_set.split('\n').collect();
    let amount_of_cargo_piles = ((split_start_set[0].len() - 1) / 4)+ 1;

    let all_cargo = (0..amount_of_cargo_piles)
        .into_iter()
        .map(|cpi| 
            split_start_set
            .iter()
            .take(split_start_set.len() - 1) // last line is definitions
            .map(|s| s.chars().skip(cpi * 4 + 1).next())
            .filter(|c| c.is_some() && c.unwrap() != ' ')
            .map(|c| c.unwrap())
            .collect::<Vec<char>>()
        ).map(|cargo| SortedCargo {
            cargo: cargo
        }).collect::<Vec<SortedCargo>>();
 
    let instructions = move_set.split('\n')
        .map(|il| {
            let mut source = il.split(" from ");
            let amount = source.next().unwrap().chars().skip(5).collect::<String>().parse().unwrap();
            let mut mutation = source.next().unwrap().split(" to ");
            let from = mutation.next().unwrap().parse().unwrap();
            let to = mutation.next().unwrap().parse().unwrap();

            return Orders {
                amount_to_move: amount,
                start_stack: from,
                end_stack: to
            };
        }).collect::<Vec<Orders>>();

    return (all_cargo, instructions);
}

fn sort_boxes(mut starting_boxes: Vec<SortedCargo>, operations: &Vec<Orders>) -> Vec<SortedCargo> {
    for op in operations {
        let (start, end) = get_two_mut(&mut starting_boxes, op.start_stack - 1, op.end_stack - 1);
        let elements = start.cargo.drain(0..op.amount_to_move as usize);
        end.cargo.splice(0..0, elements.into_iter().rev());
    }

    return starting_boxes;
}

fn sort_boxes_bulk(mut starting_boxes: Vec<SortedCargo>, operations: &Vec<Orders>) -> Vec<SortedCargo> {
    for op in operations {
        let (start, end) = get_two_mut(&mut starting_boxes, op.start_stack - 1, op.end_stack - 1);
        let elements = start.cargo.drain(0..op.amount_to_move as usize);
        end.cargo.splice(0..0, elements.into_iter());
    }

    return starting_boxes;
}

fn get_top(boxes: Vec<SortedCargo>) -> String {
    return boxes.iter().map(|cs| cs.cargo.first().unwrap()).collect::<String>();
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses() {
        let data = "\
[A]     [C]                 [H]    
[W]     [J] [D]             [J] [I]
[F]     [N] [D]     [F]     [S] [W]
[R] [B] [F] [G]     [R]     [V] [Z]
[Z] [G] [Q] [C]     [W] [G] [F] [G]
[S] [Q] [V] [P] [E] [F] [D] [R] [S]
[M] [P] [R] [Z] [P] [D] [N] [N] [M]
[D] [W] [W] [F] [T] [H] [Z] [W] [R]
[D] [W] [W] [F] [T] [H] [Z] [W] [R]
[D] [W] [W] [F] [T] [H] [Z] [W] [R]
[D] [W] [W] [F] [T] [H] [Z] [W] [R]
[Z] [X] [Y] [T] [S] [R] [P] [O] [M]
 1   2   3   4   5   6   7   8   9 

move 11 from 1 to 2
move 11 from 2 to 1";
        let (cargo, instructions) = parse_starter_stacks(data);
        let mapped = sort_boxes(cargo, instructions);
        assert_eq!("ABCDEFGHI", get_top(mapped));
    }
}