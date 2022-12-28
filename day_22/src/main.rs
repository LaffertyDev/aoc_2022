use crate::panel::Panel;
use crate::cube::Cube;
use crate::directions::Directions;
use std::env;
use std::fs;

mod directions;
mod panel;
mod cube;
mod grid_tile;
mod facing;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("Problem 1: {}", problem_1(&contents));
    println!("Problem 2: {}", problem_2(&contents));
}

fn parse_input(input: &str) -> (Panel, Cube, Directions) {
    let mut input_iter = input.split("\n\n");

    let panel_cube_data = input_iter.next().unwrap();

    let panel = Panel::parse_from_input(panel_cube_data);
    let cube = Cube::parse_from_input(panel_cube_data);
    let directions = Directions::from_string(input_iter.next().unwrap().split('\n').filter(|f| f.len() > 0).last().unwrap());

    (panel, cube, directions)
}

fn problem_1(input: &str) -> usize {
    let (panel, _cube, directions) = parse_input(input);

    let (row, col, facing) = panel.navigate(&directions);

    return (row + 1) * 1000 + (col + 1) * 4 + facing.get_value() as usize;
}

fn problem_2(input: &str) -> usize {
    let (_panel, cube, directions) = parse_input(input);

    let (row, col, facing) = cube.navigate(&directions);

    return (row + 1) * 1000 + (col + 1) * 4 + facing.get_value() as usize;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first() {
        let input = "
        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";
        assert_eq!(6032, problem_1(&input));
    }

    #[test]
    fn second() {
        let input = "
        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";
        assert_eq!(5031, problem_2(&input));
    }
}