
#[derive(Debug, Eq, PartialEq)]
pub enum GridTile {
    Empty,
    Wall,
    Floor
}

impl GridTile {
	pub fn parse_from_char(tile: char) -> GridTile {
		match tile {
			' ' => GridTile::Empty,
			'.' => GridTile::Floor,
			'#' => GridTile::Wall,
			_ => panic!()
		}
	}
}