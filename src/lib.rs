use std::error::Error;

enum Direction {
	Up, 
	Down, 
	Left, 
	Right,
}

#[derive(Debug)]
pub struct SnakeBody {
	pub x: u8,
	pub y: u8,
}

pub struct Snake {
	pub snake: Vec<SnakeBody>,
	food: (u8, u8),
	pub game_over: bool,
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

	pub fn mov(&mut self) {
		let e = self.snake.pop().unwrap();
		println!("{:?}", e);
		match self.direction {
			Direction::Up => self.snake.push(SnakeBody {x: e.x - 1, y: e.y}),
			Direction::Down => self.snake.push(SnakeBody {x: e.x + 1, y: e.y}),
			Direction::Left => self.snake.push(SnakeBody {x: e.x, y: e.y - 1}),
			Direction::Right => self.snake.push(SnakeBody {x: e.x, y: e.y + 1}),
		}
	}

	pub fn get_snake(&mut self) -> &mut Vec<SnakeBody> {
		&mut self.snake
	}
}
