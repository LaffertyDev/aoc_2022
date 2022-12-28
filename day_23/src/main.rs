use std::env;
use std::fs;

use direction::DirectionConsiderations;
use grove::Grove;

mod direction;
mod tile;
mod grove;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("Problem 1: {}", problem_1(&contents));
    println!("Problem 2: {}", problem_2(&contents));
}

fn problem_1(input: &str) -> u32 {
    let mut grove = Grove::from_input(input);
    let directions_to_consider = vec![DirectionConsiderations::North, DirectionConsiderations::South, DirectionConsiderations::West, DirectionConsiderations::East];
    let mut directions_iter = directions_to_consider.iter().cycle();


    for _i in 0..10 {
        let mut directions = vec![];
        directions.push(directions_iter.next().unwrap());
        directions.push(directions_iter.next().unwrap());
        directions.push(directions_iter.next().unwrap());
        directions.push(directions_iter.next().unwrap());
        grove.step(directions);
        // continue the cycle by 1
        directions_iter.next();
    }

    grove.count_empty_tiles()
}

fn problem_2(input: &str) -> u32 {
    let mut grove = Grove::from_input(input);
    let directions_to_consider = vec![DirectionConsiderations::North, DirectionConsiderations::South, DirectionConsiderations::West, DirectionConsiderations::East];
    let mut directions_iter = directions_to_consider.iter().cycle();


    for i in 0..10000 {
        let mut directions = vec![];
        directions.push(directions_iter.next().unwrap());
        directions.push(directions_iter.next().unwrap());
        directions.push(directions_iter.next().unwrap());
        directions.push(directions_iter.next().unwrap());
        if !grove.step(directions) {
            return i + 1;
        }
        // continue the cycle by 1
        directions_iter.next();
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty() {
        let input = "\
.......
.......
.......
...#...
.......
.......
.......";
        assert_eq!(0, problem_1(&input));
    }

    #[test]
    fn basic_works() {
        let input = "\
.......
.#...#.
.......
.......
.......
.#...#.
.......";
        assert_eq!(21, problem_1(&input));
    }

    #[test]
    fn first() {
        let input = "\
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";
        assert_eq!(110, problem_1(&input));
    }

    #[test]
    fn second() {
        let input = "\
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";
        assert_eq!(20, problem_2(&input));
    }
}