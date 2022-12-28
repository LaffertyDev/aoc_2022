use crate::directions::Directions;
use crate::directions::Direction;
use crate::grid_tile::GridTile;
use crate::facing::Facing;

#[derive(Debug, Eq, PartialEq)]
pub struct Panel {
    grid: Vec<Vec<GridTile>>
}

impl Panel {
    pub fn parse_from_input(input: &str) -> Panel {
    	let min_width = input.split('\n').map(|l| l.len()).max().unwrap();
    	return Panel {
    		grid: input
    			.split('\n')
    			.filter(|l| l.len() > 0)
    			.map(|line| {
    				// append spaces to lines that don't have enough
					return line.to_owned() + &" ".repeat(min_width - line.len());
    			})
    			.map(|row| row.chars().map(|tile| GridTile::parse_from_char(tile))
    			.collect::<Vec<GridTile>>()).collect::<Vec<Vec<GridTile>>>()
    	}
    }

	pub fn get_start_tile(&self) -> (usize, usize, Facing) {
		return (0, self.grid[0].iter().enumerate().filter(|(_i, t)| t == &&GridTile::Floor).next().unwrap().0, Facing::Right);
	}

	pub fn navigate(&self, directions: &Directions) -> (usize, usize, Facing) {
		let (start_row, start_col, start_facing) = self.get_start_tile();

		let mut current_row = start_row;
		let mut current_col = start_col;
		let mut current_facing = start_facing;

		for d in &directions.directions {
			match d {
				Direction::Distance(amount) => {
					let mut amount_to_move_remaining = *amount;
					while amount_to_move_remaining > 0 {
						let next_direction = self.get_next_position(current_row, current_col, &current_facing);
						match next_direction {
							Some(next_direction) => {
								current_row = next_direction.0;
								current_col = next_direction.1;
								amount_to_move_remaining -= 1;
							},
							None => {
								amount_to_move_remaining = 0;
							}
						}
					}
				},
				Direction::Rotate(rot) => {
					current_facing = rot.get_new_direction(&current_facing);
				},
			}
		}

		(current_row, current_col, current_facing)
	}

	fn get_next_position(&self, row: usize, column: usize, facing_direction: &Facing) -> Option<(usize, usize)> {
		match facing_direction {
			Facing::Top => {
				// go above, watching for walls and wrapping as appropriate
				// if above me is the zeroth index, get the next index that is a floor, unless we see a wall
				let mut found_row = row;
				loop {
					found_row = if found_row == 0 { self.grid.len() - 1 } else { found_row - 1};
					match self.grid[found_row][column] {
						GridTile::Wall => {
							return None;
						},
						GridTile::Floor => {
							return Some((found_row, column));
						},
						GridTile::Empty => (),
					};
				}
			},
			Facing::Down => {
				let mut found_row = row;
				loop {
					found_row = if found_row == self.grid.len() - 1 { 0 } else { found_row + 1 };
					match self.grid[found_row][column] {
						GridTile::Wall => {
							return None;
						},
						GridTile::Floor => {
							return Some((found_row, column));
						},
						GridTile::Empty => (),
					};
				}
			},
			Facing::Right => {
				let mut found_col = column;
				loop {
					found_col = if found_col == self.grid[row].len() - 1 { 0 } else { found_col + 1 };
					match self.grid[row][found_col] {
						GridTile::Wall => {
							return None;
						},
						GridTile::Floor => {
							return Some((row, found_col));
						},
						GridTile::Empty => (),
					};
				}
			},
			Facing::Left => {
				let mut found_col = column;
				loop {
					found_col = if found_col == 0 { self.grid[row].len() - 1 } else { found_col - 1 };
					match self.grid[row][found_col] {
						GridTile::Wall => {
							return None;
						},
						GridTile::Floor => {
							return Some((row, found_col));
						},
						GridTile::Empty => (),
					};
				}
			},
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn parses_parses() {
        let input = "        ...#";

        assert_eq!(Panel {
        	grid: vec![vec![GridTile::Empty, GridTile::Empty, GridTile::Empty, GridTile::Empty, GridTile::Empty, GridTile::Empty, GridTile::Empty, GridTile::Empty, GridTile::Floor, GridTile::Floor, GridTile::Floor, GridTile::Wall]]
        }, Panel::parse_from_input(input));
    }

    #[test]
    fn get_start_position_gets() {
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
        ......#.";

		let panel = Panel::parse_from_input(input);

		assert_eq!((0, 8, Facing::Right), panel.get_start_tile());
    }

    #[test]
    fn get_next_position_gets() {
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
        ......#.";

		let panel = Panel::parse_from_input(input);

		assert_eq!(Some((0, 8)), panel.get_next_position(0, 0, &Facing::Right));
		assert_eq!(Some((3, 11)), panel.get_next_position(3, 10, &Facing::Right));
		assert_eq!(Some((3, 8)), panel.get_next_position(3, 11, &Facing::Right));
		assert_eq!(None, panel.get_next_position(0, 10, &Facing::Right));

		assert_eq!(Some((10, 15)), panel.get_next_position(11, 15, &Facing::Top));
		assert_eq!(Some((11, 15)), panel.get_next_position(8, 15, &Facing::Top));
		assert_eq!(None, panel.get_next_position(11, 9, &Facing::Top));

		assert_eq!(Some((8, 15)), panel.get_next_position(11, 15, &Facing::Down));
		assert_eq!(Some((9, 15)), panel.get_next_position(8, 15, &Facing::Down));
		assert_eq!(None, panel.get_next_position(9, 9, &Facing::Down));

		assert_eq!(Some((1, 11)), panel.get_next_position(1, 8, &Facing::Left));
		assert_eq!(Some((1, 10)), panel.get_next_position(1, 11, &Facing::Left));
		assert_eq!(None, panel.get_next_position(1, 10, &Facing::Left));

		assert_eq!(Some((1, 8)), panel.get_next_position(1, 11, &Facing::Right));
		assert_eq!(Some((1, 11)), panel.get_next_position(1, 10, &Facing::Right));
		assert_eq!(None, panel.get_next_position(1, 8, &Facing::Right));

    }
}