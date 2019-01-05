use std::error::Error;

pub enum Direction {
	Up, 
	Down, 
	Left, 
	Right,
}

#[derive(Clone, Copy, Debug)]
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

		let snake = vec![SnakeBody {x: start_x, y: start_y},
										 SnakeBody {x: start_x + 1, y: start_y},
										 SnakeBody {x: start_x + 2, y: start_y},
										 SnakeBody {x: start_x + 3, y: start_y}]; 

		Ok(Snake {snake: snake, 
							food: (20, 20), 
							game_over: false, 
							width: width, 
							height: height, 
							direction: Direction::Down})
	}

	pub fn mov(&mut self) {
		let x = self.snake[self.snake.len() - 1].x;
		let y = self.snake[self.snake.len() - 1].y;

		let new_piece = match self.direction {
			Direction::Up => SnakeBody {x: x, y: y - 1},
			Direction::Down => SnakeBody {x: x, y: y + 1},
			Direction::Left => SnakeBody {x: x - 1, y: y},
			Direction::Right => SnakeBody {x: x + 1, y: y},
		};

		let (food_x, food_y) = self.food; 

		if new_piece.x != food_x || new_piece.y != food_y {
			self.snake.rotate_left(1);
			let e = self.snake.pop().unwrap();
		}
		else {
			self.new_food();
		}
		self.snake.push(new_piece);
	}

	fn new_food(&mut self) {
		self.food = (10, 10);
	}

	pub fn get_snake(&mut self) -> &mut Vec<SnakeBody> {
		&mut self.snake
	}
	
	pub fn get_food(&mut self) -> (u8, u8) {
		self.food
	}

	pub fn change_direction(&mut self, dir: Direction) {
		self.direction = dir;
	}
}
