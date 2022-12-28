use crate::tile::Tile;
use crate::direction::DirectionConsiderations;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Grove {
	grove: Vec<Vec<Tile>>
}

impl Grove {
	pub fn from_input(input: &str) -> Grove {
		let input_col_length = input.split('\n').filter(|l| l.len() > 0).next().unwrap().len();
		let dummies_to_inject = 100;

		let mut grove = vec![];

		for _i in 0..dummies_to_inject {
			grove.push(vec![Tile::Empty; input_col_length + dummies_to_inject * 2]);
		}
		for row in input.split('\n').filter(|l| l.len() > 0) {
			let mut tiles = vec![Tile::Empty; dummies_to_inject];
			for col in row.chars() {
				let tile = match col {
					'.' => Tile::Empty,
					'#' => Tile::Elf,
					_ => panic!()
				};

				tiles.push(tile);
			}
			for _i in 0..dummies_to_inject {
				tiles.push(Tile::Empty);
			}

			grove.push(tiles);
		}
		for _i in 0..dummies_to_inject {
			grove.push(vec![Tile::Empty; input_col_length + dummies_to_inject * 2]);
		}

		Grove {
			grove: grove
		}
	}

	pub fn _count_elves(&self) -> u32 {
		let mut elves = 0;
		for row in 0..self.grove.len() {
			for col in 0..self.grove[row].len() {
				if self.grove[row][col] == Tile::Elf {
					elves += 1;
				}
			}
		}

		elves
	}

	pub fn is_elf_alone(&self, row_idx: usize, column_idx: usize) -> bool {
		// northwest
		if self.grove[row_idx - 1][column_idx - 1] == Tile::Elf {
			return false;
		} 

		// north
		if self.grove[row_idx - 1][column_idx] == Tile::Elf {
			return false;
		}

		// northeast
		if self.grove[row_idx - 1][column_idx + 1] == Tile::Elf {
			return false;
		}

		// west
		if self.grove[row_idx][column_idx - 1] == Tile::Elf {
			return false;
		}

		// east
		if self.grove[row_idx][column_idx + 1] == Tile::Elf {
			return false;
		}

		// southwest
		if self.grove[row_idx + 1][column_idx - 1] == Tile::Elf {
			return false;
		}

		// south
		if self.grove[row_idx + 1][column_idx] == Tile::Elf {
			return false;
		}

		// southeast
		if self.grove[row_idx + 1][column_idx + 1] == Tile::Elf {
			return false;
		} 

		return true;
	}

	pub fn is_elf_in_direction(&self, row_idx: usize, column_idx: usize, direction: &DirectionConsiderations) -> bool {
		// northwest
		if self.grove[row_idx - 1][column_idx - 1] == Tile::Elf && (direction == &DirectionConsiderations::North || direction == &DirectionConsiderations::West) {
			return true;
		} 

		// north
		if self.grove[row_idx - 1][column_idx] == Tile::Elf && direction == &DirectionConsiderations::North {
			return true;
		} 

		// northeast
		if self.grove[row_idx - 1][column_idx + 1] == Tile::Elf && (direction == &DirectionConsiderations::North || direction == &DirectionConsiderations::East) {
			return true;
		} 

		// west
		if self.grove[row_idx][column_idx - 1] == Tile::Elf && direction == &DirectionConsiderations::West {
			return true;
		} 

		// east
		if self.grove[row_idx][column_idx + 1] == Tile::Elf && direction == &DirectionConsiderations::East {
			return true;
		} 

		// southwest
		if self.grove[row_idx + 1][column_idx - 1] == Tile::Elf && (direction == &DirectionConsiderations::South || direction == &DirectionConsiderations::West) {
			return true;
		} 

		// south
		if self.grove[row_idx + 1][column_idx] == Tile::Elf && direction == &DirectionConsiderations::South {
			return true;
		} 

		// southeast
		if self.grove[row_idx + 1][column_idx + 1] == Tile::Elf && (direction == &DirectionConsiderations::South || direction == &DirectionConsiderations::East) {
			return true;
		} 

		return false;
	}

	pub fn get_target_index(row_idx: usize, column_idx: usize, direction: &DirectionConsiderations) -> (usize, usize) {
		match direction {
			DirectionConsiderations::North => (row_idx - 1, column_idx),
			DirectionConsiderations::South => (row_idx + 1, column_idx),
			DirectionConsiderations::East => (row_idx, column_idx + 1),
			DirectionConsiderations::West => (row_idx, column_idx - 1),
		}
	}

	pub fn step(&mut self, directions_to_consider: Vec<&DirectionConsiderations>) -> bool {
		// Step 1 -- find all elves that should move
		let mut move_targets: HashMap::<(usize, usize), ((usize, usize), u32)> = HashMap::new();
		for row_idx in 0..self.grove.len() {
			for col_idx in 0..self.grove[row_idx].len() {
				if self.grove[row_idx][col_idx] == Tile::Elf {
					if !self.is_elf_alone(row_idx, col_idx) {
						for direction in directions_to_consider.iter() {
							if !self.is_elf_in_direction(row_idx, col_idx, direction) {
								// we can attempt to go in that direction!
								let dest = move_targets.entry(Self::get_target_index(row_idx, col_idx, direction)).or_insert(((row_idx, col_idx), 0));
								dest.1 += 1;
								break;
							}
						}
					}
				}
			}
		}

		let mut does_elf_move = false;
		for move_destination in move_targets.iter().filter(|f| f.1.1 == 1u32) {
			does_elf_move = true;
			let source = move_destination.1.0;
			let destination = move_destination.0;
			self.grove[source.0][source.1] = Tile::Empty;
			self.grove[destination.0][destination.1] = Tile::Elf;
		}

		return does_elf_move;
	}

	pub fn count_empty_tiles(&self) -> u32 {
		let mut top_most_row = usize::MAX;
		let mut bottom_most_row = 0;
		let mut left_most_col = usize::MAX;
		let mut right_most_col = 0;

		for row_idx in 0..self.grove.len() {
			for col_idx in 0..self.grove[row_idx].len() {
				if self.grove[row_idx][col_idx] == Tile::Elf {
					top_most_row = std::cmp::min(top_most_row, row_idx);
					bottom_most_row = std::cmp::max(bottom_most_row, row_idx);
					left_most_col = std::cmp::min(left_most_col, col_idx);
					right_most_col = std::cmp::max(right_most_col, col_idx);
				}
			}
		}

		let mut empty_tiles_in_elf_quad = 0;
		for row in top_most_row..=bottom_most_row {
			for col in left_most_col..=right_most_col {
				if self.grove[row][col] == Tile::Empty {
					empty_tiles_in_elf_quad += 1;
				}
			}
		}

		empty_tiles_in_elf_quad
	}
}