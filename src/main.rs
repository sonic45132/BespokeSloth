#![allow(dead_code)]

mod evaluation;
mod constants;
mod move_generation;
mod helpers;
mod search;
mod zorbist;
mod uci;
mod engine;


//use constants::Pieces;
//use constants::TEntry;
//use rand::Rng;
use uci::*;
extern crate vampirc_uci;
//use search::*;
//use zorbist::*;

//use helpers::*;

//Main function for use in UCI mode
fn main_uci() {
	let uci_rx = setup_uci_parser();
	uci_control(uci_rx);
	std::process::exit(0);
}

//Main function for use in testing
fn main_alt() {
	let starting = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
	let _state = helpers::parse_fen(starting);

	// let mut side = Pieces::WHITE;

	// if read_line() != "white" {
	// 	side = Pieces::BLACK;
	// }

	// if side == Pieces::WHITE {
	// 	let best_move = negamax(&state, side, 8);
	// 	println!("{:?}",to_alg(&best_move));
	// 	state = make_move(&state, best_move, state.to_move);
	// }

	// loop {
	// 	let other_move = from_alg(read_line());
	// 	state = make_move(&state, other_move, state.to_move);

	// 	let best_move = negamax(&state, side, 8);
	// 	println!("{:?}",to_alg(&best_move));
	// 	state = make_move(&state, best_move, state.to_move);
	// }

	//let mut t_table: [TEntry; 2^24];
}

fn main() {
	// let zkeys = generate_keys(0xDEADBEEF);

	//Determine which engine mode to use
	if helpers::read_line() == "uci" {
	 	main_uci();
	} else {
		main_alt();
	}
}