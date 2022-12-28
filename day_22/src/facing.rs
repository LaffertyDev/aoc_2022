#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Copy, Clone)]
pub enum Facing {
	Top,
	Down,
	Right,
	Left
}

impl Facing {
	pub fn get_value(&self) -> u32 {
		match self {
			Facing::Right => 0,
			Facing::Down => 1,
			Facing::Left => 2,
			Facing::Top => 3,
		}
	}
}