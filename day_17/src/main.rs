use std::env;
use std::fs;
use std::collections::HashMap;

#[derive(PartialEq)]
#[derive(Debug)]
enum WindPattern {
    Left,
    Right
}

#[derive(Eq, Hash, PartialEq)]
struct PlayHistory {
    wind_pattern_index: usize,
    shape_index: Shape,
    tallest_y_states: [usize; 7]
}

impl WindPattern {
    fn get_shift_amount(&self) -> i32 {
        return match self {
            WindPattern::Left => -1,
            WindPattern::Right => 1
        };
    }

    fn get_next_wind_index(current_wind_index: usize, wind_length: usize) -> usize {
        (current_wind_index + 1) % wind_length
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
enum Shape {
    // ####
    Wide,
    // .#.
    // ###
    // .#.
    Plus,
    // ..#
    // ..#
    // ###
    ReverseL,
    // #
    // #
    // #
    // #
    Long,
    // ##
    // ##
    Square
}

struct Tower {
    tallest_y: usize,
    grid: Vec<[bool; 7]>
}

impl Tower {
    fn new() -> Tower {
        Tower {
            tallest_y: 0,
            grid: vec![]
        }
    }

    fn get_tallest_y(&self) -> usize {
        self.tallest_y
    }

    fn get_current_board_state(&self, wind_index: usize, shape: Shape) -> Option<PlayHistory> {
        let tallest_y = self.get_tallest_y();
        if tallest_y == 0 {
            return None;
        }

        let mut indices = [0; 7];
        for x in 0..self.grid[0].len() {
            for y in (0..=tallest_y).rev() {
                if self.grid[y][x] {
                    indices[x] = y;
                    break;
                }
            }
        }

        let shortest_y = indices.iter().min().unwrap().clone();
        for x in 0..indices.len() {
            indices[x] -= shortest_y;
        }

        Some(PlayHistory {
            wind_pattern_index: wind_index,
            shape_index: shape,
            tallest_y_states: indices
        })
    }

    fn can_shape_move(&self, shape: &Shape, wind_pattern: &WindPattern, shape_lowest_left_coordinate: usize, shape_bottom_edge_coordinate: usize) -> bool {
        let shape_dimensions = shape.get_shape_dimensions();

        match wind_pattern {
            WindPattern::Left => {
                if shape_lowest_left_coordinate == 0 {
                    return false; // nope, at edge
                }

                for y in 0..shape_dimensions.len() {
                    for x in 0..shape.get_shape_width() {
                        if self.grid[shape_bottom_edge_coordinate + y][shape_lowest_left_coordinate + x - 1] && shape_dimensions[y][x] {
                            // collide
                            return false;
                        }
                    }
                }
            },
            WindPattern::Right => {
                if shape_lowest_left_coordinate + shape.get_shape_width() - 1 + 1 > self.grid[0].len() - 1 { // already all the way to the right baybee
                    return false; // nope, at edge
                }

                for y in 0..shape_dimensions.len() {
                    for x in 0..shape.get_shape_width() {
                        if self.grid[shape_bottom_edge_coordinate + y][shape_lowest_left_coordinate + x + 1] && shape_dimensions[y][x] {
                            // collide
                            return false;
                        }
                    }
                }
            }
        };

        true
    }

    fn does_shape_collide_from_below(&self, shape: &Shape, shape_lowest_left_coordinate: usize, shape_bottom_edge_coordinate: usize) -> bool {
        let shape_dimensions = shape.get_shape_dimensions();
        if shape_bottom_edge_coordinate == 0 {
            return true; // nothing below the floor baybee
        }

        for y in 0..shape_dimensions.len() {
            for x in 0..shape.get_shape_width() {
                if self.grid[shape_bottom_edge_coordinate + y - 1][shape_lowest_left_coordinate + x] && shape_dimensions[y][x] {
                    // collide
                    return true;
                }
            }
        }

        false
    }

    fn do_collide(&mut self, shape: &Shape, shape_lowest_left_coordinate: usize, shape_bottom_edge_coordinate: usize) -> () {
        // shape at the current boundaries are marked as solidlet shape_dimensions = shape.get_shape_dimensions();
        let shape_dimensions = shape.get_shape_dimensions();
        for y in 0..shape_dimensions.len() {
            for x in 0..shape.get_shape_width() {
                if shape_dimensions[y][x] {
                    self.tallest_y = self.tallest_y.max(shape_bottom_edge_coordinate + y);
                    self.grid[shape_bottom_edge_coordinate + y][shape_lowest_left_coordinate + x] = true;
                }
            }
        }
    }
}

impl Shape {
    fn get_next_shape(index: usize) -> Shape {
        return match index % 5 {
            0 => Shape::Wide,
            1 => Shape::Plus,
            2 => Shape::ReverseL,
            3 => Shape::Long,
            4 => Shape::Square,
            _x => { unreachable!(); }
        };
    }

    fn get_shape_width(&self) -> usize {
        return match self {
            Shape::Wide => 4,
            Shape::Plus => 3,
            Shape::ReverseL => 3,
            Shape::Long => 1,
            Shape::Square => 2
        }
    }

    fn get_shape_dimensions(&self)  -> [[bool; 4]; 4] {
        return match self {
            Shape::Wide => [[true, true, true, true], [false, false, false, false], [false, false, false, false], [false, false, false, false]],
            Shape::Plus => [[false, true, false, false], [true, true, true, false], [false, true, false, false], [false, false, false, false]],
            Shape::ReverseL => [[true, true, true, false], [false, false, true, false], [false, false, true, false], [false, false, false, false]],
            Shape::Long => [[true, false, false, false], [true, false, false, false], [true, false, false, false], [true, false, false, false]],
            Shape::Square => [[true, true, false, false], [true, true, false, false], [false, false, false, false], [false, false, false, false]]
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("Problem 1: {}", problem_1(&contents));
    println!("Problem 2: {}", problem_2(&contents));
}

fn parse_input(input: &str) -> Vec<WindPattern> {
    input.trim_end().chars().map(|c| if c == '>' { WindPattern::Right } else { WindPattern::Left }).collect::<Vec<WindPattern>>()
}

fn get_tower_height(input: &str, height: usize) -> usize {
    let pattern = parse_input(&input);

    let mut tower = Tower::new();
    tower.grid.reserve(500_001);
    tower.grid.push([true; 7]);
    for _x in 0..500_000 {
        tower.grid.push([false; 7]);
    }

    let mut current_wind_index = 0;
    let mut play_history = HashMap::new(); // for the last states that happened, what is the play history for that state?
    let mut play_history_map = vec![];

    for x in 0..height {
        let shape = Shape::get_next_shape(x);

        // now check to see where the shape collides
        let tallest_y = tower.get_tallest_y();
        let spawn_pos = tallest_y + 4; // three units of gap

        let mut shape_bottom_index = spawn_pos;
        let mut shape_current_bottom_left_index = 2;

        loop {
            // first move by wind (user input lol)
            if tower.can_shape_move(&shape, &pattern[current_wind_index], shape_current_bottom_left_index, shape_bottom_index) {
                shape_current_bottom_left_index = ((shape_current_bottom_left_index as i32) + pattern[current_wind_index].get_shift_amount()) as usize;
            }
            current_wind_index = WindPattern::get_next_wind_index(current_wind_index, pattern.len());

            // check if we now collide with something below us
            if tower.does_shape_collide_from_below(&shape, shape_current_bottom_left_index, shape_bottom_index) {
                tower.do_collide(&shape, shape_current_bottom_left_index, shape_bottom_index);
                break; // advance to next shape
            }

            // descend one level
            shape_bottom_index -= 1;
        }

        // tallest height at the end of this rock is...
        play_history_map.push(tower.get_tallest_y());

        // have we seen this iteration before?
        if let Some(current_board_state) = tower.get_current_board_state(current_wind_index, shape) {
            if play_history.contains_key(&current_board_state) {
                // we found a cycle! this exact state has happened before, so we know we've looped
                let cycle_start_rock_index = play_history.get(&current_board_state).unwrap();
                let cycle_end_index = x - 1;

                let cycle_duration = cycle_end_index - cycle_start_rock_index + 1; // 2..5 contains 4 values
                let amount_of_repeated_cycles = (height - cycle_start_rock_index) / cycle_duration;
                let elements_remaining_at_end_of_cycle = (height - (cycle_start_rock_index)) % cycle_duration;

                // 15 - 6 == 9; -- 6,7,8,9,10,11,12,13,14
                // cycle_duration = (5 - 2 + 1) == 4
                // repeat_cycles = 9 / 4 == 2
                // elements_remaining = 9 % 4 == 1

                // 2, 3
                // 2,3,4

                // 0-14 (15 elements)
                // 2-5
                // 2,3,4,5 cycle, 6,7,8,9 cycle, 10,11,12,13 cycle
                // 0,1 do not cycle
                // 14 does not cycle
                // 4%5 == 4
                // 2 + 4 == 6 -- 6th element is the final element

                let tower_height_before_cycle = play_history_map.get(cycle_start_rock_index - 1).unwrap();
                let tower_heigh_during_cycle = play_history_map.get(cycle_end_index).unwrap() - tower_height_before_cycle;
                let new_height_after_tower = if elements_remaining_at_end_of_cycle > 0 { *play_history_map.get(cycle_start_rock_index + elements_remaining_at_end_of_cycle - 1).unwrap() - tower_height_before_cycle} else { 0usize };
                return tower_height_before_cycle + (tower_heigh_during_cycle * amount_of_repeated_cycles) + new_height_after_tower;
            }

            play_history.insert(current_board_state, x);
        }
    }

    *play_history_map.last().unwrap()
}

fn problem_1(input: &str) -> usize {
    get_tower_height(input, 2022)
}

fn problem_2(input: &str) -> usize {
    get_tower_height(input, 1000000000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_parses() {
        assert_eq!(WindPattern::Left, parse_input(&"<")[0]);
        assert_eq!(WindPattern::Right, parse_input(&">")[0]);
        assert_eq!(1, parse_input(&">\n").len());
    }

    #[test]
    fn shift_amount_shifts() {
        assert_eq!(-1, WindPattern::Left.get_shift_amount());
        assert_eq!(1, WindPattern::Right.get_shift_amount());
    }

    #[test]
    fn get_tallest_y_works() {
        let mut tower = Tower::new();
        tower.grid.push([true; 7]); // first element, the floor
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([true; 7]); 
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]); // last element
        tower.tallest_y = 3; // stupid, but hey it works
        assert_eq!(3, tower.get_tallest_y());
    }

    #[test]
    fn get_next_shape_gets() {
        assert_eq!(Shape::Wide, Shape::get_next_shape(0));
        assert_eq!(Shape::Plus, Shape::get_next_shape(1));
        assert_eq!(Shape::ReverseL, Shape::get_next_shape(2));
        assert_eq!(Shape::Long, Shape::get_next_shape(3));
        assert_eq!(Shape::Square, Shape::get_next_shape(4));
    }

    #[test]
    fn can_shape_move_left_moves_wall() {
        let mut tower = Tower::new();
        tower.grid.push([true; 7]); // first element, the floor
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        let shape = Shape::get_next_shape(0);
        assert!(tower.can_shape_move(&shape, &WindPattern::Left, 2, 4));
        assert!(tower.can_shape_move(&shape, &WindPattern::Left, 1, 4));
        assert!(!tower.can_shape_move(&shape, &WindPattern::Left, 0, 4));
    }

    #[test]
    fn can_shape_move_left_moves_collides() {
        let mut tower = Tower::new();
        tower.grid.push([true; 7]); // first element, the floor
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid[4][0] = true;
        let shape = Shape::get_next_shape(0);
        assert!(tower.can_shape_move(&shape, &WindPattern::Left, 2, 4));
        assert!(!tower.can_shape_move(&shape, &WindPattern::Left, 1, 4));
        assert!(!tower.can_shape_move(&shape, &WindPattern::Left, 0, 4));
    }

    #[test]
    fn can_shape_move_right_moves_collides() {
        let mut tower = Tower::new();
        tower.grid.push([true; 7]); // first element, the floor
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid[4][6] = true;
        let shape = Shape::get_next_shape(0);
        assert!(tower.can_shape_move(&shape, &WindPattern::Right, 1, 4));
        assert!(!tower.can_shape_move(&shape, &WindPattern::Right, 2, 4));
        assert!(!tower.can_shape_move(&shape, &WindPattern::Right, 3, 4));
        assert!(!tower.can_shape_move(&shape, &WindPattern::Right, 4, 4));
        assert!(!tower.can_shape_move(&shape, &WindPattern::Right, 5, 4));
        assert!(!tower.can_shape_move(&shape, &WindPattern::Right, 6, 4));
    }

    #[test]
    fn can_shape_move_right_moves_wall() {
        let mut tower = Tower::new();
        tower.grid.push([true; 7]); // first element, the floor
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        let shape = Shape::get_next_shape(0);
        assert!(tower.can_shape_move(&shape, &WindPattern::Right, 2, 4));
        assert!(!tower.can_shape_move(&shape, &WindPattern::Right, 3, 4));
        assert!(!tower.can_shape_move(&shape, &WindPattern::Right, 4, 4));
        assert!(!tower.can_shape_move(&shape, &WindPattern::Right, 5, 4));
        assert!(!tower.can_shape_move(&shape, &WindPattern::Right, 6, 4));
    }

    #[test]
    fn can_shape_move_right_long_wall() {
        let mut tower = Tower::new();
        tower.grid.push([true; 7]); // first element, the floor
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        let shape = Shape::Long;
        assert!(tower.can_shape_move(&shape, &WindPattern::Right, 2, 4));
        assert!(tower.can_shape_move(&shape, &WindPattern::Right, 3, 4));
        assert!(tower.can_shape_move(&shape, &WindPattern::Right, 4, 4));
        assert!(tower.can_shape_move(&shape, &WindPattern::Right, 5, 4));
        assert!(!tower.can_shape_move(&shape, &WindPattern::Right, 6, 4));
    }

    #[test]
    fn shape_collides_from_below() {
        let mut tower = Tower::new();
        tower.grid.push([true; 7]); // first element, the floor
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        let shape = Shape::get_next_shape(0);
        assert!(!tower.does_shape_collide_from_below(&shape, 2, 4));
        assert!(!tower.does_shape_collide_from_below(&shape, 2, 3));
        assert!(!tower.does_shape_collide_from_below(&shape, 2, 2));
        assert!(tower.does_shape_collide_from_below(&shape, 2, 1));
        assert!(tower.does_shape_collide_from_below(&shape, 2, 0));
    }

    #[test]
    fn collide_collides() {
        let mut tower = Tower::new();
        tower.grid.push([true; 7]); // first element, the floor
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        let shape = Shape::Plus;
        tower.do_collide(&shape, 0, 1);
        assert!(!tower.grid[1][0]);
        assert!(tower.grid[1][1]);
        assert!(!tower.grid[1][2]);
        assert!(!tower.grid[1][3]);
        assert!(tower.grid[2][0]);
        assert!(tower.grid[2][1]);
        assert!(tower.grid[2][2]);
        assert!(!tower.grid[2][3]);
        assert!(!tower.grid[3][0]);
        assert!(tower.grid[3][1]);
        assert!(!tower.grid[3][2]);
        assert!(!tower.grid[3][3]);
        assert!(!tower.grid[4][0]);
        assert!(!tower.grid[4][1]);
        assert!(!tower.grid[4][2]);
        assert!(!tower.grid[4][3]);
    }

    #[test]
    fn get_current_board_state_gets() {
        let mut tower = Tower::new();
        tower.grid.push([true; 7]); // first element, the floor
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.grid.push([false, false, true, false, false, false, false]);
        tower.grid.push([false; 7]);
        tower.grid.push([false; 7]);
        tower.tallest_y = 3;

        assert_eq!([0,0,3,0,0,0,0], tower.get_current_board_state(0, Shape::Wide).unwrap().tallest_y_states);
    }

    #[test]
    fn wind_next_loops() {
        assert_eq!(0, WindPattern::get_next_wind_index(299, 300));
    }

    #[test]
    fn get_tower_height_gets() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        assert_eq!(1, get_tower_height(&input, 1));
        assert_eq!(47, get_tower_height(&input, 27));
        assert_eq!(100, get_tower_height(&input, 62));
        assert_eq!(153, get_tower_height(&input, 97));
        assert_eq!(206, get_tower_height(&input, 132));
        assert_eq!(208, get_tower_height(&input, 133));
        assert_eq!(210, get_tower_height(&input, 134));
    }

    #[test]
    fn first() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        assert_eq!(3068, problem_1(&input));
    }

    #[test]
    fn second() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        assert_eq!(1514285714288, problem_2(&input));
    }
}