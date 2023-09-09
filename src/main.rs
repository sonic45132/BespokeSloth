#![allow(dead_code)]

mod evaluation;
mod constants;
mod move_generation;
mod helpers;
mod search;
mod zorbist;


use constants::Pieces;
use constants::TEntry;
//use move_generation::*;
//use rand::Rng;
use search::*;

use helpers::*;

fn main() {
	let starting = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

	let state = parse_fen(starting);


	let best_move = minimax(&state, Pieces::WHITE, 5);
	
	println!("{:?}",best_move);

	let mut t_table: [TEntry; 2^24];

}