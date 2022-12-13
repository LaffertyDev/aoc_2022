use std::env;
use std::fs;
use std::fmt;

#[derive(PartialEq, Clone)]
#[derive(Debug)]
struct GridPos {
    x: usize,
    y: usize
}

impl fmt::Display for GridPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:0>2},{:0>2}) ", self.x, self.y)
    }
}

impl GridPos {
    fn new(x: usize, y: usize) -> GridPos {
        GridPos {
            x: x,
            y: y
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

fn problem_1(input: &str) -> u32 {
    let grid = parse_grid(input);
    let start_pos = find_start(&grid);
    let end_pos = find_end(&grid);

    if let Some(path) = find_cheapest_path(&grid, &start_pos, &end_pos) {
        return (path.len() - 1) as u32; // start doesn't count
    } else {
        unreachable!();
    }
}

fn problem_2(input: &str) -> u32 {
    let grid = parse_grid(input);
    let starting_positions = find_potential_starting_positions(&grid);
    let end_pos = find_end(&grid);

    let mut smallest_path: u32 = u32::MAX;
    for start_pos in starting_positions {
        if let Some(path) = find_cheapest_path(&grid, &start_pos, &end_pos) {
            smallest_path = std::cmp::min(smallest_path, (path.len() - 1) as u32);
        }
    }

    smallest_path
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    return input.split('\n').map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
}

fn find_start(grid: &Vec<Vec<char>>) -> GridPos {
    for c in 0..grid.len() {
        for r in 0..grid[c].len() {
            if grid[c][r] == 'S' {
                return GridPos::new(r, c);
            }
        }
    }

    unreachable!();
}

fn find_potential_starting_positions(grid: &Vec<Vec<char>>) -> Vec<GridPos> {
    let mut starting_positions = vec![];
    for c in 0..grid.len() {
        for r in 0..grid[c].len() {
            if grid[c][r] == 'S' || grid[c][r] == 'a' {
                starting_positions.push(GridPos::new(r, c));
            }
        }
    }

    starting_positions
}

fn find_end(grid: &Vec<Vec<char>>) -> GridPos {
    for c in 0..grid.len() {
        for r in 0..grid[c].len() {
            if grid[c][r] == 'E' {
                return GridPos::new(r, c);
            }
        }
    }

    unreachable!();
}

fn reconstruct_path(start: &GridPos, end: &GridPos, quickest_map: &Vec<Vec<Option<GridPos>>>) -> Vec<GridPos> {
    let mut current_pos = end;
    let mut path = vec![current_pos.clone()];
    while current_pos != start {
        current_pos = &quickest_map[current_pos.y][current_pos.x].as_ref().unwrap();
        path.insert(0, current_pos.clone());
    }

    path
}

fn find_cheapest_path(map: &Vec<Vec<char>>, start: &GridPos, goal: &GridPos) -> Option<Vec<GridPos>> {
    if map.is_empty() || map[0].is_empty() {
        return None;
    }

    let mut nodes_to_expand: Vec<(GridPos, u32)> = vec![];
    nodes_to_expand.push((start.clone(), 0));
    let mut goal_cost: Vec<Vec<u32>> = vec![vec![]; map.len()];
    let mut quickest_map: Vec<Vec<Option<GridPos>>> = vec![vec![]; map.len()];
    for column in 0..map.len() {
        for _row in 0..map[column].len() {
            goal_cost[column].push(u32::MAX);
            quickest_map[column].push(None);
        }
    }
    goal_cost[start.y][start.x] = 0;

    while !nodes_to_expand.is_empty() {
        let expanding_node = nodes_to_expand.pop().unwrap();
        if &expanding_node.0 == goal {
            return Some(reconstruct_path(start, goal, &quickest_map));
        }

        for neighbor in get_eligible_neighbors(&expanding_node.0, &map) {
            let cost_to_reach_neighbor = goal_cost[expanding_node.0.y][expanding_node.0.x] + 1; // cost is always increased by 1
            if cost_to_reach_neighbor < goal_cost[neighbor.y][neighbor.x] {
                // the current cost for the node is less than what we've recorded so far
                quickest_map[neighbor.y][neighbor.x] = Some(expanding_node.0.clone());
                goal_cost[neighbor.y][neighbor.x] = cost_to_reach_neighbor;
                if !nodes_to_expand.iter().find(|n| n.0.x == expanding_node.0.x && n.0.y == expanding_node.0.y).is_some() {
                    nodes_to_expand.push((neighbor, cost_to_reach_neighbor + 1));
                    nodes_to_expand.sort_by(|a,b| b.1.cmp(&a.1)); // sort descending, algorithm could be better
                }
            }
        }
    }

    None
}

fn _print_map(quickest_map: &Vec<Vec<Option<GridPos>>>, map: &Vec<Vec<char>>) {
    for col in 0..quickest_map.len() {
        if col == 0 {
            print!("          ");
            for row in 0..quickest_map[col].len() {
                print!("   COL {:0>2}  ", row);
            }
            print!("\n");
        }

        print!("ROW {:0>2} ----- ", col);
        for row in 0..quickest_map[col].len() {
            if quickest_map[col][row].is_none() {
                print!("{} (  ,  ) ", map[col][row]);
            } else {
                print!("{} {}", map[col][row], quickest_map[col][row].as_ref().unwrap());
            }
        }

        print!(" ROW {:0>2}", col);
        print!("\n");
    }
}

fn get_eligible_neighbors(node: &GridPos, map: &Vec<Vec<char>>) -> Vec<GridPos> {
    let mut neighbors = vec![];
    let node_height = get_node_height(map[node.y][node.x]);

    // left, top, down, right
    if node.x > 0 {
        // can go left
        let left = node.x - 1;
        let left_height = get_node_height(map[node.y][left]);
        if can_reach(node_height, left_height) {
            neighbors.push(GridPos::new(left, node.y));
        }
    }

    if node.x < map[node.y].len() - 1 {
        // can go right
        let right = node.x + 1;
        let right_height = get_node_height(map[node.y][right]);
        if can_reach(node_height, right_height) {
            neighbors.push(GridPos::new(right, node.y));
        }
    }

    if node.y > 0 {
        // can go top
        let top = node.y - 1;
        let top_height = get_node_height(map[top][node.x]);
        if can_reach(node_height, top_height) {
            neighbors.push(GridPos::new(node.x, top));
        }
    }

    if node.y < map.len() - 1 {
        // can go down
        let down = node.y + 1;
        let down_height = get_node_height(map[down][node.x]);
        if can_reach(node_height, down_height) {
            neighbors.push(GridPos::new(node.x, down));
        }
    }

    neighbors
}

fn get_node_height(c: char) -> u32 {
    if c == 'S' {
        return 0;
    }

    if c == 'E' {
        return 25;
    }

    return (c as u32) - 97;
}

fn can_reach(start: u32, eligible: u32) -> bool {
    if start > eligible { // can always descend
        return true;
    }

    return eligible - start < 2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_height_gets() {
        assert_eq!(0, get_node_height('a'));
        assert_eq!(25, get_node_height('z'));
        assert_eq!(0, get_node_height('S'));
        assert_eq!(25, get_node_height('E'));
    }

    #[test]
    fn can_reach_reaches() {
        assert!(can_reach(1, 1));
        assert!(can_reach(1, 2));
        assert!(can_reach(2, 1));
        assert!(can_reach(2, 2));
        assert!(can_reach(2, 0));
        assert!(!can_reach(0, 2));
    }

    #[test]
    fn get_eligible_neighbors_topleft() {
        let test_grid = "\
Saaaa
aaaaa
aaaaa
aaaaa
aaaaE\
";
        let expected_neighbors = vec![GridPos::new(1,0), GridPos::new(0,1)];
        assert_eq!(expected_neighbors, get_eligible_neighbors(&GridPos::new(0,0), &parse_grid(&test_grid)));
    }

    #[test]
    fn get_eligible_neighbors_middle() {
        let test_grid = "\
Saaaa
aacaa
acaaa
aaaaa
aaaaE\
";
        let expected_neighbors = vec![GridPos::new(0,1), GridPos::new(1,0)];
        assert_eq!(expected_neighbors, get_eligible_neighbors(&GridPos::new(1,1), &parse_grid(&test_grid)));
    }

    #[test]
    fn get_eligible_neighbors_destination() {
        let test_grid = "\
Saaaa
aacaa
acaaa
aaaca
aayzE\
";
        let expected_neighbors = vec![GridPos::new(2,4), GridPos::new(4,4), GridPos::new(3,3)];
        assert_eq!(expected_neighbors, get_eligible_neighbors(&GridPos::new(3,4), &parse_grid(&test_grid)));
    }

    #[test]
    fn problem_1_test() {
        let test_grid = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

        assert_eq!(31, problem_1(&test_grid));
    }
}

















