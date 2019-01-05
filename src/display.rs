extern crate ncurses;

use ncurses::*; 
use std::{thread, time};

use snake;
use snake::Snake; 

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

impl Board {
	pub fn generate_board() -> Result<Board, &'static str> {
		let mut board = [Piece::Null; WIDTH*HEIGHT];
		for i in 0..WIDTH {
			board[i] = Piece::Wall;
			board[WIDTH*HEIGHT - 1 - i] = Piece::Wall;
		}
		for i in 0..HEIGHT {
			board[WIDTH * i] = Piece::Wall;
			if (i > 0) { board[(WIDTH * i) - 1] = Piece::Wall; }
		}
		Ok(Board {board: board})
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

		nodelay(stdscr(), true);
		noecho();

		self.print_board();
	}

	fn clear_screen(&mut self) {
		for i in 1..(HEIGHT - 1) {
			for k in 1..(WIDTH - 1) {
				self.board[i*16 + k] = Piece::Null;
			}
		}
	}

	pub fn run(&mut self, mut snake: Snake) {
		while !snake.game_over {
			thread::sleep(time::Duration::from_millis(50));
			
			clear();
			snake.mov();
			for piece in snake.get_snake() {
				self.board[(piece.y as usize) * (HEIGHT - 2) + (piece.x as usize)] = Piece::Snake
			}
			self.print_board();
		}
	}
}
