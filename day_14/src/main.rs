use std::env;
use std::fs;

#[derive(Debug)]
#[derive(PartialEq)]
enum CaveTile {
    Sand,
    Empty,
    Wall,
    SandSource
}

#[derive(Debug)]
struct CaveMap {
    floor: Option<usize>,
    grid: Vec<Vec<CaveTile>>
}

impl CaveMap {
    fn new(floor: Option<usize>) -> CaveMap {
        CaveMap {
            floor: floor,
            grid: vec![]
        }
    }

    fn insert_tile(&mut self, x: usize, y: usize, tile: CaveTile) -> () {
        if self.grid.len() < y + 1 {
            let y_elements_needed = y + 1 - self.grid.len();
            for _i in 0..y_elements_needed {
                self.grid.push(vec![]);
            }
        }

        if self.grid[y].len() < x + 1 {
            let x_elements_needed = x + 1 - self.grid[y].len();
            for _i in 0..x_elements_needed {
                self.grid[y].push(CaveTile::Empty);
            }
        }

        self.grid[y][x] = tile;
    }

    fn get_tile(&self, x: usize, y: usize) -> &CaveTile {
        if self.grid.len() <= y {
            return &CaveTile::Empty;
        }
        if self.grid[y].len() <= x {
            return &CaveTile::Empty;
        }

        &self.grid[y][x]
    }

    fn _get_sand_producers(&self) -> Vec<(usize, usize)> {
        let mut sand_producers = vec![];
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if self.get_tile(x,y) == &CaveTile::SandSource {
                    sand_producers.push((x, y));
                }
            }
        }
        sand_producers
    }

    fn physics_tick(&mut self) -> bool {
        // product sand and drop it
        //let sand_producers = self.get_sand_producers();

        let sand_producer = (500, 0);
        // drop sand
        let sand_pos = self.get_next_position(sand_producer.0, sand_producer.1); 
        if sand_pos.is_none() || sand_pos.unwrap() == sand_producer {
            return true; // we reached the top for problem 2
        }

        let mut sand_pos = sand_pos.unwrap();
        loop {
            match self.get_next_position(sand_pos.0, sand_pos.1) {
                None => {
                    // we have hit the edge
                    return true;
                },
                Some(next_pos) => {
                    if sand_pos == next_pos {
                        // sand is no longer moving, done with this tick
                        //println!("Inserted sand: {}, {}", sand_pos.0, sand_pos.1);
                        self.insert_tile(sand_pos.0, sand_pos.1, CaveTile::Sand);
                        return false;
                    } else {
                        sand_pos = next_pos;
                    }
                }
            }
        }
    }

    fn grid_pos_is_solid(&self, x: usize, y: usize) -> bool {
        if self.floor.is_some() {
            if self.floor.unwrap() == y {
                return true;
            }
        }

        let tile = self.get_tile(x, y);
        return tile == &CaveTile::Wall || tile == &CaveTile::Sand || tile == &CaveTile::SandSource;
    }

    fn get_next_position(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        if self.floor.is_none() && y >= self.grid.len() {
            return None; // we are below the lowest wall, so we have hit the abyss
        }

        if !self.grid_pos_is_solid(x, y + 1) {
            // we can drop here!
            return Some((x, y + 1));
        }

        if !self.grid_pos_is_solid(x - 1, y + 1) {
            return Some((x - 1, y));
        }

        if !self.grid_pos_is_solid(x + 1, y + 1) {
            return Some((x + 1, y));
        }

        // sand cannot move, return same position
        return Some((x, y))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("Problem 1: {}", problem_1(&contents));
    println!("Problem 2: {}", problem_2(&contents));
}

fn parse_map(input: &str, include_floor: bool) -> CaveMap {
    let direction_segments = input.split('\n')
        .filter(|line| line.len() > 0)
        .map(|line| line.split(" -> ")
            .map(|points| points
                .split(',')
                .map(|point| point.parse::<usize>().unwrap()).collect::<Vec<usize>>()
                ).collect::<Vec<Vec<usize>>>()
            )
        .collect::<Vec<Vec<Vec<usize>>>>();

    let mut highest_y = 0;
    for segment in direction_segments.iter() {
        for pair in segment {
            highest_y = highest_y.max(pair[1]);
        }
    }

    let floor = if include_floor { Some(highest_y + 2) } else { None };

    let mut cave_map = CaveMap::new(floor);
    for segment in direction_segments {
        for i in 0..segment.len() - 1 {
            let start_pair = &segment[i];
            let end_pair = &segment[i+1];

            if start_pair[0] == end_pair[0] {
                // grow y
                let x = start_pair[0];
                let start = start_pair[1].min(end_pair[1]);
                let end = start_pair[1].max(end_pair[1]);
                for y in start..=end {
                    cave_map.insert_tile(x, y, CaveTile::Wall);
                }
            } else if start_pair[1] == end_pair[1] {
                // grow x
                let y = start_pair[1];
                let start = start_pair[0].min(end_pair[0]);
                let end = start_pair[0].max(end_pair[0]);
                for x in start..=end {
                    cave_map.insert_tile(x, y, CaveTile::Wall);
                }
            } else {
                unreachable!();
            }
        }
    }

    cave_map.insert_tile(500, 0, CaveTile::SandSource);
    cave_map
}

fn problem_1(input: &str) -> u32 {
    let mut map = parse_map(input, false);
    let mut sand_produced = 0;
    while !map.physics_tick() {
        sand_produced += 1;
    }
    sand_produced
}

fn problem_2(input: &str) -> u32 {
    let mut map = parse_map(input, true);
    let mut sand_produced = 0;
    while !map.physics_tick() {
        sand_produced += 1;
    }
    // my simulator will not overwrite a sand tile, but the problem expects it
    sand_produced + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first() {
        let input = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";
        assert_eq!(24, problem_1(&input));
    }

    #[test]
    fn second() {
        let input = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";
        assert_eq!(93, problem_2(&input));
    }
}