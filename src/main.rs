#![allow(dead_code)]

mod evaluation;
mod constants;
mod move_generation;
mod helpers;
mod search;
mod zorbist;
mod uci;


//use constants::Pieces;
//use constants::TEntry;
//use rand::Rng;
use uci::*;
//use search::*;
//use zorbist::*;

//use helpers::*;

fn main() {

	setup_uci();
	std::process::exit(1);

	// let zkeys = generate_keys(0xDEADBEEF);

	// if read_line() != "start" {
	// 	std::process::exit(1);
	// }

	// let starting = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

	// let mut state = parse_fen(starting);

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