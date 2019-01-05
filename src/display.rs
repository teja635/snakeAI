extern crate ncurses;

use ncurses::*; 
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

	pub fn initialize_screen(&mut self) {
		initscr();
		raw();

		nodelay(stdscr(), true);
		noecho();

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
		getch();
		endwin();
	}

	pub fn run(&mut self, snake: Snake) {
	
	}
}
