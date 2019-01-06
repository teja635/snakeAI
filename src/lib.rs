extern crate rand; 

use std::error::Error;
use std::collections::HashSet;
use std::iter::FromIterator;
use rand::{Rng, thread_rng};

pub enum Direction {
	Up, 
	Down, 
	Left, 
	Right,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Coordinate {
	pub x: u8,
	pub y: u8,
}

pub struct Snake {
	pub snake: Vec<Coordinate>,
	food: Coordinate,
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

		let snake = vec![Coordinate {x: start_x, y: start_y},
										 Coordinate {x: start_x + 1, y: start_y},
										 Coordinate {x: start_x + 2, y: start_y},
										 Coordinate {x: start_x + 3, y: start_y}]; 

		Ok(Snake {snake: snake, 
							food: Coordinate {x: 20, y: 20}, 
							game_over: false, 
							width: width, 
							height: height, 
							direction: Direction::Down})
	}

	pub fn mov(&mut self) {
		let x = self.snake[self.snake.len() - 1].x;
		let y = self.snake[self.snake.len() - 1].y;

		let new_piece = match self.direction {
			Direction::Up => Coordinate {x: x, y: y - 1},
			Direction::Down => Coordinate {x: x, y: y + 1},
			Direction::Left => Coordinate {x: x - 1, y: y},
			Direction::Right => Coordinate {x: x + 1, y: y},
		};

		if new_piece.x != self.food.x || new_piece.y != self.food.y {
			self.snake.rotate_left(1);
			let e = self.snake.pop().unwrap();
		}
		else {
			self.new_food();
		} self.snake.push(new_piece);
	}

	fn new_food(&mut self) {
		let curr_snake: HashSet<Coordinate> = HashSet::from_iter(self.snake.iter().cloned());
		let mut candidate = Coordinate {x: thread_rng().gen_range(0, 62),y: thread_rng().gen_range(0, 22)};
		while curr_snake.contains(&candidate) {
			candidate.x = thread_rng().gen_range(0, 62);
			candidate.y = thread_rng().gen_range(0, 22);
		}
		self.food = candidate; 
	}

	pub fn get_snake(&mut self) -> &mut Vec<Coordinate> {
		&mut self.snake
	}
	
	pub fn get_food(&mut self) -> Coordinate {
		self.food
	}

	pub fn change_direction(&mut self, dir: Direction) {
		self.direction = dir;
	}
}
