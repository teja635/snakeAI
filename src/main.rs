use std::process;
mod display;

use snake;
use snake::Snake;

fn main() {
	let game: Snake = Snake::new(5, 5, 10, 10).unwrap_or_else(|err| {
		println!("Problem generating game: {}", err);
		process::exit(1);
	});

	let mut board: display::Board = display::Board::generate_board().unwrap_or_else(|err| {
		println!("Problem generating board: {}", err);
		process::exit(1);
	});

	board.initialize_screen();
}
