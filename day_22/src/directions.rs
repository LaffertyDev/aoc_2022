
use crate::facing::Facing;

#[derive(Debug, Eq, PartialEq)]
pub struct Directions {
    pub directions: Vec<Direction>
}

impl Directions {
    pub fn from_string(input: &str) -> Directions {
    	let mut current_num: Option<u32> = None;
    	let mut directions = vec![];

    	let mut char_iter = input.chars().peekable();
    	while let Some(c) = char_iter.next() {
    		match c {
    			'L' => {
    				directions.push(Direction::Rotate(Rotation::Counterclockwise));
    			},
    			'R' => {
    				directions.push(Direction::Rotate(Rotation::Clockwise));
    			},
    			val => {
    				let val = val.to_digit(10).unwrap();
    				if let Some(current_val) = current_num {
    					current_num = Some(current_val * 10 + val)
    				} else {
    					current_num = Some(val);
    				}

    				let peeked_next = char_iter.peek();
					if peeked_next.is_none() || !peeked_next.unwrap().is_digit(10) {
						directions.push(Direction::Distance(current_num.unwrap()));
						current_num = None;
					}
    			},
    		}
    	}

        Directions {
            directions: directions
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Direction {
    Distance(u32),
    Rotate(Rotation)
}

#[derive(Debug, Eq, PartialEq)]
pub enum Rotation {
    Clockwise,
    Counterclockwise
}

impl Rotation {
	pub fn get_new_direction(&self, facing: &Facing) -> Facing {
		match (facing, &self) {
			(Facing::Top, Rotation::Clockwise) => Facing::Right,
			(Facing::Right, Rotation::Clockwise) => Facing::Down,
			(Facing::Down, Rotation::Clockwise) => Facing::Left,
			(Facing::Left, Rotation::Clockwise) => Facing::Top,
			(Facing::Top, Rotation::Counterclockwise) => Facing::Left,
			(Facing::Left, Rotation::Counterclockwise) => Facing::Down,
			(Facing::Down, Rotation::Counterclockwise) => Facing::Right,
			(Facing::Right, Rotation::Counterclockwise) => Facing::Top,
		}
	}
}


#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn parses_parses() {
        let input = "10R5";
        assert_eq!(Directions {
        	directions: vec![Direction::Distance(10), Direction::Rotate(Rotation::Clockwise), Direction::Distance(5)]
        }, Directions::from_string(&input));
    }

    #[test]
    fn parses_complex() {
        let input = "10R5L5R10L4R5L5";
        assert_eq!(Directions {
        	directions: vec![
        		Direction::Distance(10)
        		, Direction::Rotate(Rotation::Clockwise)
        		, Direction::Distance(5)
        		, Direction::Rotate(Rotation::Counterclockwise)
        		, Direction::Distance(5)
        		, Direction::Rotate(Rotation::Clockwise)
        		, Direction::Distance(10)
        		, Direction::Rotate(Rotation::Counterclockwise)
        		, Direction::Distance(4)
        		, Direction::Rotate(Rotation::Clockwise)
        		, Direction::Distance(5)
        		, Direction::Rotate(Rotation::Counterclockwise)
        		, Direction::Distance(5)
        	]
        }, Directions::from_string(&input));
    }
}

