use std::error::Error;

enum Direction {
	Up, 
	Down, 
	Left, 
	Right,
}

struct SnakeBody {
	x: u8,
	y: u8,
}

pub struct Snake {
	snake: Vec<SnakeBody>,
	food: (u8, u8),
	game_over: bool,
	width: u8,
	height: u8,
	direction: Direction,
}

impl Snake {
	pub fn new(start_x: u8, start_y: u8, width: u8, height: u8) -> Result<Snake, &'static str>{
		if start_x > width || start_y > height {
			return Err("Snake starts out of bounds");
		}

		let snake = vec![SnakeBody {x: start_x, y: start_y}]; 

		Ok(Snake {snake: snake, 
							food: (0, 0), 
							game_over: false, 
							width: width, 
							height: height, 
							direction: Direction::Down})
	}
}
