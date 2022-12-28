use crate::directions::Directions;
use crate::directions::Direction;
use crate::grid_tile::GridTile;
use crate::facing::Facing;

#[derive(Debug, Eq, PartialEq)]
pub struct Cube {
	face_size: usize,
    grid: Vec<Vec<GridTile>>
}

#[derive(Debug)]
enum Face {
	Front,
	Left,
	Back,
	Right,
	Top,
	Bottom
}


fn real_face_local_to_global(local_row: usize, local_column: usize, face_size: usize, to: Face) -> (usize, usize) {
	match to {
		Face::Top => {
			return (local_row + (face_size * 0), local_column + (face_size * 1));
		},
		Face::Front => {
			return (local_row + (face_size * 1), local_column + (face_size * 1));
		},
		Face::Left => {
			return (local_row + (face_size * 2), local_column + (face_size * 0));
		},
		Face::Back => {
			return (local_row + (face_size * 3), local_column + (face_size * 0));
		},
		Face::Bottom => {
			return (local_row + (face_size * 2), local_column + (face_size * 1));
		},
		Face::Right => {
			return (local_row + (face_size * 0), local_column + (face_size * 2));
		},
	}
}


fn test_face_local_to_global(local_row: usize, local_column: usize, face_size: usize, to: Face) -> (usize, usize) {
	match to {
		Face::Top => {
			return (local_row + (face_size * 0), local_column + (face_size * 2));
		},
		Face::Front => {
			return (local_row + (face_size * 1), local_column + (face_size * 2));
		},
		Face::Left => {
			return (local_row + (face_size * 1), local_column + (face_size * 1));
		},
		Face::Back => {
			return (local_row + (face_size * 1), local_column + (face_size * 0));
		},
		Face::Bottom => {
			return (local_row + (face_size * 2), local_column + (face_size * 2));
		},
		Face::Right => {
			return (local_row + (face_size * 2), local_column + (face_size * 3));
		},
	}
}

impl Cube {
	fn get_real_new_coordinates_from_face_with_direction(&self, row: usize, column: usize, direction: &Facing) -> (usize, usize, Facing) {
		println!("Current Face: {:?}", self.get_real_current_face_from_coordinates(row, column));
		let localized_row = row % self.face_size;
		let localized_col = column % self.face_size;
		let bottom_row = self.face_size - 1;
		let right_col = self.face_size - 1;
		match self.get_real_current_face_from_coordinates(row, column) {
			Face::Top => {
				match direction { 
					Facing::Top => {
						let local_coordinates = real_face_local_to_global(localized_col, 0, self.face_size, Face::Back);
						return (local_coordinates.0, local_coordinates.1, Facing::Right);
					}, // go to back face
					Facing::Left => {
						let local_coordinates = real_face_local_to_global(self.face_size - 1 - localized_row, 0, self.face_size, Face::Left);
						return (local_coordinates.0, local_coordinates.1, Facing::Right);
					}, // go to left face
					Facing::Down => {
						let local_coordinates = real_face_local_to_global(0, localized_col, self.face_size, Face::Right);
						return (local_coordinates.0, local_coordinates.1, Facing::Down);
					}, // go to front face
					Facing::Right => {
						let local_coordinates = real_face_local_to_global(localized_row, 0, self.face_size, Face::Front);
						return (local_coordinates.0, local_coordinates.1, Facing::Right);
					}, // go to right face
				}
			},
			Face::Front => {
				match direction {
					Facing::Top => {
						let local_coordinates = real_face_local_to_global(bottom_row, localized_col, self.face_size, Face::Top);
						return (local_coordinates.0, local_coordinates.1, Facing::Top);
					}, // go to top face
					Facing::Left => {
						let local_coordinates = real_face_local_to_global(0, localized_row, self.face_size, Face::Left);
						return (local_coordinates.0, local_coordinates.1, Facing::Down);
					}, // go to left face
					Facing::Down => {
						let local_coordinates = real_face_local_to_global(0, localized_col, self.face_size, Face::Bottom);
						return (local_coordinates.0, local_coordinates.1, Facing::Down);
					}, // go to bottom face
					Facing::Right => {
						let local_coordinates = real_face_local_to_global(bottom_row, localized_row, self.face_size, Face::Right);
						return (local_coordinates.0, local_coordinates.1, Facing::Top);
					}, // go to right face
				}
			},
			Face::Left => {
				match direction {
					Facing::Top => {
						let local_coordinates = real_face_local_to_global(localized_col, 0, self.face_size, Face::Front);
						return (local_coordinates.0, local_coordinates.1, Facing::Right);
					}, // go to top face
					Facing::Left => {
						let local_coordinates = real_face_local_to_global(self.face_size - 1 - localized_row, 0, self.face_size, Face::Top);
						return (local_coordinates.0, local_coordinates.1, Facing::Right);
					}, // go to back face
					Facing::Down => {
						let local_coordinates = real_face_local_to_global(0, localized_col, self.face_size, Face::Back);
						return (local_coordinates.0, local_coordinates.1, Facing::Down);
					}, // go to bottom face
					Facing::Right => {
						let local_coordinates = real_face_local_to_global(localized_row, 0, self.face_size, Face::Bottom);
						return (local_coordinates.0, local_coordinates.1, Facing::Right);
					}, // go to front face
				}
			},
			Face::Back => {
				match direction {
					Facing::Top => {
						let local_coordinates = real_face_local_to_global(bottom_row, localized_col, self.face_size, Face::Left);
						return (local_coordinates.0, local_coordinates.1, Facing::Top);
					}, // go to top face
					Facing::Left => {
						let local_coordinates = real_face_local_to_global(0, localized_row, self.face_size, Face::Top);
						return (local_coordinates.0, local_coordinates.1, Facing::Down);
					}, // go to back face
					Facing::Down => {
						let local_coordinates = real_face_local_to_global(0, localized_col, self.face_size, Face::Right);
						return (local_coordinates.0, local_coordinates.1, Facing::Down);
					}, // go to bottom face
					Facing::Right => {
						let local_coordinates = real_face_local_to_global(bottom_row, localized_row, self.face_size, Face::Bottom);
						return (local_coordinates.0, local_coordinates.1, Facing::Top);
					}, // go to front face
				}
			},
			Face::Bottom => {
				match direction {
					Facing::Top => {
						let local_coordinates = real_face_local_to_global(bottom_row, localized_col, self.face_size, Face::Front);
						return (local_coordinates.0, local_coordinates.1, Facing::Top);
					},
					Facing::Left => {
						let local_coordinates = real_face_local_to_global(localized_row, right_col, self.face_size, Face::Left);
						return (local_coordinates.0, local_coordinates.1, Facing::Left);
					},
					Facing::Down => {
						let local_coordinates = real_face_local_to_global(localized_col, right_col, self.face_size, Face::Back);
						return (local_coordinates.0, local_coordinates.1, Facing::Left);
					},
					Facing::Right => {
						let local_coordinates = real_face_local_to_global(self.face_size - 1 - localized_row, right_col, self.face_size, Face::Right);
						return (local_coordinates.0, local_coordinates.1, Facing::Left);
					},
				}
			},
			Face::Right => {
				match direction {
					Facing::Top => {
						let local_coordinates = real_face_local_to_global(bottom_row, localized_col, self.face_size, Face::Back);
						return (local_coordinates.0, local_coordinates.1, Facing::Top);
					},
					Facing::Left => {
						let local_coordinates = real_face_local_to_global(localized_row, right_col, self.face_size, Face::Top);
						return (local_coordinates.0, local_coordinates.1, Facing::Left);
					},
					Facing::Down => {
						let local_coordinates = real_face_local_to_global(localized_col, right_col, self.face_size, Face::Front);
						return (local_coordinates.0, local_coordinates.1, Facing::Left);
					},
					Facing::Right => {
						let local_coordinates = real_face_local_to_global(self.face_size - 1 - localized_row, right_col, self.face_size, Face::Bottom);
						return (local_coordinates.0, local_coordinates.1, Facing::Left);
					},
				}
			}
		}
	}

	fn get_test_new_coordinates_from_face_with_direction(&self, row: usize, column: usize, direction: &Facing) -> (usize, usize, Facing) {
		println!("Current Face: {:?}", self.get_test_current_face_from_coordinates(row, column));
		match self.get_test_current_face_from_coordinates(row, column) {
			// how the hell do you do this


			// need a way to map the edges of the cube

			// {this edge} -> {that edge}

			// then I just get {face, edge} I am approaching (from the direction)

			// and apply the transformation on the edge to get the new local coordinate
			Face::Top => {
				match direction { 
					Facing::Top => {
						let local_coordinates = test_face_local_to_global(0, self.face_size - 1 - column % self.face_size, self.face_size, Face::Back);
						return (local_coordinates.0, local_coordinates.1, Facing::Down);
					}, // go to back face
					Facing::Left => {
						let local_coordinates = test_face_local_to_global(0, row % self.face_size, self.face_size, Face::Left);
						return (local_coordinates.0, local_coordinates.1, Facing::Down);
					}, // go to left face
					Facing::Down => {
						let local_coordinates = test_face_local_to_global(0, column % self.face_size, self.face_size, Face::Front);
						return (local_coordinates.0, local_coordinates.1, Facing::Down);
					}, // go to front face
					Facing::Right => {
						let local_coordinates = test_face_local_to_global(self.face_size - 1 - row % self.face_size, self.face_size - 1, self.face_size, Face::Right);
						return (local_coordinates.0, local_coordinates.1, Facing::Left);
					}, // go to right face
				}
			},
			Face::Front => {
				match direction {
					Facing::Top => {
						let local_coordinates = test_face_local_to_global(self.face_size - 1, column % self.face_size, self.face_size, Face::Top);
						return (local_coordinates.0, local_coordinates.1, Facing::Top);
					}, // go to top face
					Facing::Left => {
						let local_coordinates = test_face_local_to_global(row % self.face_size, self.face_size - 1, self.face_size, Face::Left);
						return (local_coordinates.0, local_coordinates.1, Facing::Left);
					}, // go to left face
					Facing::Down => {
						let local_coordinates = test_face_local_to_global(0, column % self.face_size, self.face_size, Face::Bottom);
						return (local_coordinates.0, local_coordinates.1, Facing::Down);
					}, // go to bottom face
					Facing::Right => {
						let local_coordinates = test_face_local_to_global(0, self.face_size - 1 - row % self.face_size, self.face_size, Face::Right);
						return (local_coordinates.0, local_coordinates.1, Facing::Down);
					}, // go to right face
				}
			},
			Face::Left => {
				match direction {
					Facing::Top => {
						let local_coordinates = test_face_local_to_global(column % self.face_size, 0, self.face_size, Face::Top);
						return (local_coordinates.0, local_coordinates.1, Facing::Right);
					}, // go to top face
					Facing::Left => {
						let local_coordinates = test_face_local_to_global(row % self.face_size, self.face_size - 1, self.face_size, Face::Back);
						return (local_coordinates.0, local_coordinates.1, Facing::Left);
					}, // go to back face
					Facing::Down => {
						let local_coordinates = test_face_local_to_global(self.face_size - 1 - column % self.face_size, 0, self.face_size, Face::Bottom);
						return (local_coordinates.0, local_coordinates.1, Facing::Right);
					}, // go to bottom face
					Facing::Right => {
						let local_coordinates = test_face_local_to_global(row % self.face_size, 0, self.face_size, Face::Front);
						return (local_coordinates.0, local_coordinates.1, Facing::Right);
					}, // go to front face
				}
			},
			Face::Back => {
				match direction {
					Facing::Top => {
						let local_coordinates = test_face_local_to_global(0, self.face_size - 1 - column % self.face_size, self.face_size, Face::Top);
						return (local_coordinates.0, local_coordinates.1, Facing::Down);
					}, // go to top face
					Facing::Left => {
						let local_coordinates = test_face_local_to_global(self.face_size - 1, self.face_size - 1 - row % self.face_size, self.face_size, Face::Right);
						return (local_coordinates.0, local_coordinates.1, Facing::Top);
					}, // go to back face
					Facing::Down => {
						let local_coordinates = test_face_local_to_global(self.face_size - 1, self.face_size - 1 - column % self.face_size, self.face_size, Face::Bottom);
						return (local_coordinates.0, local_coordinates.1, Facing::Top);
					}, // go to bottom face
					Facing::Right => {
						let local_coordinates = test_face_local_to_global(row % self.face_size, 0, self.face_size, Face::Left);
						return (local_coordinates.0, local_coordinates.1, Facing::Right);
					}, // go to front face
				}
			},
			Face::Bottom => {
				match direction {
					Facing::Top => {
						let local_coordinates = test_face_local_to_global(self.face_size - 1, column % self.face_size, self.face_size, Face::Front);
						return (local_coordinates.0, local_coordinates.1, Facing::Top);
					},
					Facing::Left => {
						let local_coordinates = test_face_local_to_global(self.face_size - 1, self.face_size - 1 - row % self.face_size, self.face_size, Face::Left);
						return (local_coordinates.0, local_coordinates.1, Facing::Top);
					},
					Facing::Down => {
						let local_coordinates = test_face_local_to_global(self.face_size - 1, self.face_size - 1 - column % self.face_size, self.face_size, Face::Back);
						return (local_coordinates.0, local_coordinates.1, Facing::Top);
					},
					Facing::Right => {
						let local_coordinates = test_face_local_to_global(row % self.face_size, 0, self.face_size, Face::Right);
						return (local_coordinates.0, local_coordinates.1, Facing::Right);
					},
				}
			},
			Face::Right => {
				match direction {
					Facing::Top => {
						let local_coordinates = test_face_local_to_global(self.face_size - 1 - column % self.face_size, self.face_size - 1, self.face_size, Face::Front);
						return (local_coordinates.0, local_coordinates.1, Facing::Left);
					},
					Facing::Left => {
						let local_coordinates = test_face_local_to_global(row % self.face_size, self.face_size - 1, self.face_size, Face::Bottom);
						return (local_coordinates.0, local_coordinates.1, Facing::Left);
					},
					Facing::Down => {
						let local_coordinates = test_face_local_to_global(self.face_size - 1 - column % self.face_size, 0, self.face_size, Face::Back);
						return (local_coordinates.0, local_coordinates.1, Facing::Right);
					},
					Facing::Right => {
						let local_coordinates = test_face_local_to_global(self.face_size - 1 - row % self.face_size, self.face_size - 1, self.face_size, Face::Top);
						return (local_coordinates.0, local_coordinates.1, Facing::Left);
					},
				}
			}
		}
	}
	fn get_test_current_face_from_coordinates(&self, row: usize, column: usize) -> Face {
		// this is where I would need to deduce the cube shape
		// top is row + (0), col + (2 * face_length)
		// front is row + (1 * face_length), col + (2 * face_length)
		// left is row + (1 * face_length), col + (1 * face_length)
		// back is row + (1 * face_length), col + (0 * face_length)
		// bottom is row + (2 * face_length), col + (2 * face_length)
		// right is row + (2 * face_length), col + (3 * face_length)
		match (row / self.face_size, column / self.face_size) {
			(0, 2) => Face::Top, // top
			(1, 2) => Face::Front, // front
			(1, 1) => Face::Left, // left
			(1, 0) => Face::Back, // back
			(2, 2) => Face::Bottom, // bottom
			(2, 3) => Face::Right, // right
			// missing -- 0,0; 0, 1; 0, 3; 1, 3; 2, 0; 2,1
			_ => {
				println!("Row: {}, Col: {}", row, column);
				unreachable!();
			}
		}
	}
	fn get_real_current_face_from_coordinates(&self, row: usize, column: usize) -> Face {
		// this is where I would need to deduce the cube shape
		// top is row + (0), col + (2 * face_length)
		// front is row + (1 * face_length), col + (2 * face_length)
		// left is row + (1 * face_length), col + (1 * face_length)
		// back is row + (1 * face_length), col + (0 * face_length)
		// bottom is row + (2 * face_length), col + (2 * face_length)
		// right is row + (2 * face_length), col + (3 * face_length)
		match (row / self.face_size, column / self.face_size) {
			(0, 1) => Face::Top, // top
			(1, 1) => Face::Front, // front
			(2, 0) => Face::Left, // left
			(3, 0) => Face::Back, // back
			(2, 1) => Face::Bottom, // bottom
			(0, 2) => Face::Right, // right
			_ => {
				println!("Row: {}, Col: {}", row, column);
				unreachable!();
			}
		}
	}


    pub fn parse_from_input(input: &str) -> Cube {


    	// TODO -- fix min-width and face_size
    	// 183087 was my last answer hardcoding, it was wrong
    	let min_width = input.split('\n').map(|l| l.len()).max().unwrap();
    	return Cube {
    		face_size: 50,
    		grid: input
    			.split('\n')
    			.filter(|l| l.len() > 0)
    			.map(|line| {
    				// append spaces to lines that don't have enough
					return line.to_owned() + &" ".repeat(min_width - line.len());
    			})
    			.map(|row| row.chars().map(|tile| GridTile::parse_from_char(tile))
    			.collect::<Vec<GridTile>>()).collect::<Vec<Vec<GridTile>>>(),
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
								current_facing = next_direction.2;
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

	fn get_next_position(&self, row: usize, column: usize, facing_direction: &Facing) -> Option<(usize, usize, Facing)> {
		let next_potential_coordinates = match facing_direction {
			Facing::Top => {
				if row % self.face_size == 0 {
					self.get_real_new_coordinates_from_face_with_direction(row, column, facing_direction)
				} else {
					(row - 1, column, *facing_direction)
				}
			},
			Facing::Down => {
				if row % self.face_size == self.face_size - 1 {
					self.get_real_new_coordinates_from_face_with_direction(row, column, facing_direction)
				} else {
					(row + 1, column, *facing_direction)
				}
			},
			Facing::Right => {
				if column % self.face_size == self.face_size - 1 {
					self.get_real_new_coordinates_from_face_with_direction(row, column, facing_direction)
				} else {
					(row, column + 1, *facing_direction)
				}
			},
			Facing::Left => {
				if column % self.face_size == 0 {
					self.get_real_new_coordinates_from_face_with_direction(row, column, facing_direction)
				} else {
					(row, column - 1, *facing_direction)
				}
			},
		};

		match self.grid[next_potential_coordinates.0][next_potential_coordinates.1] {
			GridTile::Wall => {
				return None;
			},
			GridTile::Floor => {
				return Some((next_potential_coordinates.0, next_potential_coordinates.1, next_potential_coordinates.2));
			},
			GridTile::Empty => panic!(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	// 5,2 -> 0,2

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

		let cube = Cube::parse_from_input(input);

		assert_eq!((0, 8, Facing::Right), cube.get_start_tile());
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

		let cube = Cube::parse_from_input(input);

    }
}