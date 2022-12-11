use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    amount: u32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("Problem 1: {}", problem_1(&contents));
    println!("Problem 2: {}", problem_2(&contents, 10));
}

fn problem_1(input: &str,) -> u32 {
    let mut head_pos = (0,0);
    let mut tail_pos = (0,0);
    let mut tail_positions = HashSet::<(i32, i32)>::new();
    tail_positions.insert((0,0));

    for instruction in input.split('\n').map(|l| parse_move(l)) {
        for _i in 0..instruction.amount {
            let direction_vector = match instruction.direction {
                Direction::Up => (0, 1),
                Direction::Down => (0, -1),
                Direction::Left => (-1, 0),
                Direction::Right => (1, 0)
            };

            head_pos.0 += direction_vector.0;
            head_pos.1 += direction_vector.1;
            if is_detached_from_head(head_pos, tail_pos) {
                tail_pos = head_pos;
                tail_pos.0 -= direction_vector.0;
                tail_pos.1 -= direction_vector.1;
                tail_positions.insert(tail_pos.clone());
            }
        }
    }

    return tail_positions.len().try_into().unwrap();
}

fn problem_2(input: &str, tail_length: usize) -> u32 {
    if input.len() < tail_length {
        return 0;
    }

    let mut body_positions = vec![(0,0); tail_length];
    let mut tail_positions = HashSet::<(i32, i32)>::new();
    for instruction in input.split('\n').map(|l| parse_move(l)) {
        for _move_amount in 0..instruction.amount {
            let direction_vector = match instruction.direction {
                Direction::Up => (0, 1),
                Direction::Down => (0, -1),
                Direction::Left => (-1, 0),
                Direction::Right => (1, 0)
            };

            let current_head = body_positions.last().unwrap();
            let next_head = (current_head.0 + direction_vector.0, current_head.1 + direction_vector.1);

            let neck = body_positions.get(body_positions.len() - 2).unwrap();

            if is_detached_from_head(next_head, *neck) {
                *body_positions.last_mut().unwrap() = next_head;
                for x in (0..body_positions.len() - 1).rev() {
                    body_positions[x] = resolve_next_position(body_positions[x+1], body_positions[x]);
                }
            } else {
                // simply update the current heads position
                let mut current_head = body_positions.last_mut().unwrap();
                current_head.0 = next_head.0;
                current_head.1 = next_head.1;
            }

            tail_positions.insert(body_positions[0]);
        }
    }

    return tail_positions.len().try_into().unwrap();
}

fn parse_move(instruction: &str) -> Instruction {
    let mut split_instruction = instruction.split(' ');
    let d = split_instruction.next().unwrap();
    let a = split_instruction.next().unwrap().parse::<u32>().unwrap();

    let direction = match d {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => panic!()
    };

    Instruction {
        direction: direction,
        amount: a
    }
}

fn is_detached_from_head(head: (i32, i32), tail: (i32, i32)) -> bool {
    let x_dist_abs = (head.0 - tail.0).abs();
    let y_dist_abs = (head.1 - tail.1).abs();

    x_dist_abs > 1 || y_dist_abs > 1 || (x_dist_abs + y_dist_abs) > 2
}

fn resolve_next_position(parent: (i32, i32), child: (i32, i32)) -> (i32, i32) {
    let x_dis = parent.0 - child.0;
    let y_dis = parent.1 - child.1;

    let movement = if x_dis.abs() + y_dis.abs() > 2 {
        // diagonal
        (1 * (x_dis / x_dis.abs()), 1 * (y_dis / y_dis.abs()))
    } else if x_dis.abs() > 1 {
        (1 * (x_dis / x_dis.abs()), 0)
    } else if y_dis.abs() > 1 {
        (0, 1 * (y_dis / y_dis.abs()))
    } else {
        (0,0)
    };

    return (child.0 + movement.0, child.1 + movement.1);
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first() {
        let input = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(13, problem_1(&input));
        assert_eq!(13, problem_2(&input, 2));
    }

    #[test]
    fn second() {
        let input = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(36, problem_2(&input, 10));
    }

    #[test]
    fn test_is_detached_from_head() {
        assert!(!is_detached_from_head((0,0), (0,0)));
        assert!(!is_detached_from_head((0,1), (0,0)));
        assert!(!is_detached_from_head((1,0), (0,0)));
        assert!(!is_detached_from_head((1,0), (0,0)));
        assert!(!is_detached_from_head((1,1), (0,0)));
        assert!(!is_detached_from_head((0,0), (1,0)));
        assert!(!is_detached_from_head((0,0), (1,1)));
        assert!(!is_detached_from_head((0,-1), (0,0)));
        assert!(!is_detached_from_head((-1,0), (0,0)));
        assert!(!is_detached_from_head((-1,1), (0,0)));
        assert!(!is_detached_from_head((0,0), (0,-1)));
        assert!(!is_detached_from_head((0,0), (-1,0)));
        assert!(!is_detached_from_head((0,0), (-1,-1)));
        assert!(is_detached_from_head((0,2), (0,0)));
        assert!(is_detached_from_head((0,-2), (0,0)));
        assert!(is_detached_from_head((2,0), (0,0)));
        assert!(is_detached_from_head((0,-2), (0,0)));
        assert!(is_detached_from_head((-2,-2), (0,0)));
        assert!(is_detached_from_head((2,2), (0,0)));
        assert!(is_detached_from_head((0,0), (0,2)));
        assert!(is_detached_from_head((0,0), (0,-2)));
        assert!(is_detached_from_head((0,0), (2,0)));
        assert!(is_detached_from_head((0,0), (-2,0)));
        assert!(is_detached_from_head((0,0), (-2,-2)));
        assert!(is_detached_from_head((0,0), (2,2)));
    }

    #[test]
    fn resolves_next_position() {
        assert_eq!((0,0), resolve_next_position((0,0), (0,0)));
        assert_eq!((0,0), resolve_next_position((0,1), (0,0)));
        assert_eq!((0,0), resolve_next_position((1,0), (0,0)));
        assert_eq!((0,0), resolve_next_position((1,1), (0,0)));
        assert_eq!((0,1), resolve_next_position((0,0), (0,1)));
        assert_eq!((1,0), resolve_next_position((0,0), (1,0)));
        assert_eq!((1,1), resolve_next_position((0,0), (1,1)));
        assert_eq!((0,1), resolve_next_position((0,2), (0,0)));
        assert_eq!((1,0), resolve_next_position((2,0), (0,0)));
        assert_eq!((1,1), resolve_next_position((2,2), (0,0)));
        assert_eq!((0,1), resolve_next_position((0,0), (0,2)));
        assert_eq!((1,0), resolve_next_position((0,0), (2,0)));
        assert_eq!((1,1), resolve_next_position((0,0), (2,2)));
        assert_eq!((0,-1), resolve_next_position((0,-2), (0,0)));
        assert_eq!((-1,0), resolve_next_position((-2,0), (0,0)));
        assert_eq!((-1,-1), resolve_next_position((-2,-2), (0,0)));
        assert_eq!((0,-1), resolve_next_position((0,0), (0,-2)));
        assert_eq!((-1,0), resolve_next_position((0,0), (-2,0)));
        assert_eq!((-1,-1), resolve_next_position((0,0), (-2,-2)));
        assert_eq!((4,1), resolve_next_position((4,2), (3,0)));
    }
}