extern crate ncurses;

use ncurses::*; 
use range_check::Within;
use std::process;

use snake;
use snake::Snake; 
use snake::Direction; 
use snake::Coordinate; 

const WIDTH: usize = 64;
const HEIGHT: usize = 24;

#[derive(Clone, Copy)]
enum Piece {
	Wall, 
	Snake, 
	Food,
	Null,
}

pub struct Board {
	board: [Piece; WIDTH*HEIGHT],
}

fn check_oob(x: usize, y: usize) -> bool {
	x.is_within(1..WIDTH - 1) && y.is_within(1..HEIGHT - 1)
}

fn board_coords(loc: Coordinate) -> (usize, usize) {
	let x = (loc.x + 1) as usize;
	let y = (loc.y + 1) as usize;

	return (x, y)
}

impl Board {
	pub fn generate_board() -> Result<Board, &'static str> {
		let mut board = [Piece::Null; WIDTH*HEIGHT];
		for i in 0..WIDTH {
			board[i] = Piece::Wall;
			board[WIDTH*HEIGHT - 1 - i] = Piece::Wall;
		}
		for i in 0..HEIGHT {
			board[WIDTH * i] = Piece::Wall;
			if i > 0 { board[(WIDTH * i) - 1] = Piece::Wall; }
		}
		Ok(Board {board: board})
	}

	fn game_over(&mut self) {
		printw("=================GAME OVER=====================");
		endwin();
		process::exit(1);
	}

	fn print_board(&mut self) {
		for i in 0..HEIGHT {
			for k in 0..WIDTH {
				match self.board[i*WIDTH + k] {
					Piece::Wall => printw("#"),
					Piece::Snake => printw("0"), 
					Piece::Food => printw("F"),
					Piece::Null => printw(" "),
				};
			}
			printw("\n");
		}
		refresh();
	}

	pub fn initialize_screen(&mut self) {
		initscr();
		raw();
		halfdelay(1);

		keypad(stdscr(), true);
		noecho();

		self.print_board();
	}

	fn clear_screen(&mut self) {
		for i in 1..(HEIGHT - 1) {
			for k in 1..(WIDTH - 1) {
				self.board[i*64 + k] = Piece::Null;
			}
		}
	}

	fn put_snake_piece(&mut self, piece: Coordinate) {
		let (x, y) = board_coords(piece);
		if !check_oob(x, y) { self.game_over(); }
		self.board[y * WIDTH + x] = Piece::Snake;
	}

	fn put_food(&mut self, food: Coordinate) {
		let x = (food.x + 1) as usize;
		let y = (food.y + 1) as usize;
		self.board[y * WIDTH + x] = Piece::Food;
	}

	pub fn run(&mut self, mut snake: Snake) {
		loop {
			let ch = getch();
			
			clear();
			match snake.mov() {
				Ok(()) => {}, 
				Err(_error) => { self.game_over(); }
			}
			self.clear_screen();
			for piece in snake.get_snake() {
				self.put_snake_piece(*piece);
			}

			let food = snake.get_food();
			self.put_food(food);
			self.print_board();
			match ch {
				KEY_LEFT => snake.change_direction(Direction::Left),
				KEY_RIGHT => snake.change_direction(Direction::Right) ,
				KEY_UP => snake.change_direction(Direction::Up) ,
				KEY_DOWN => snake.change_direction(Direction::Down) ,
				_ => {},
			};
		}
	}
}
