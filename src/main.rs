#![allow(dead_code)]

mod evaluation;
mod constants;
mod move_generation;
mod helpers;

use constants::Pieces;
use evaluation::score_board;
use move_generation::Move;
use rand::Rng;

use helpers::*;

pub struct State {
	pub board: [u8; 64],
	pub to_move: u8,
	pub moves_made: u32,
	pub white_castle: u8,
	pub black_castle: u8
}

fn main() {
	let starting = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

	let mut state = State {
		board: parse_fen(starting),
		to_move: Pieces::WHITE,
		moves_made: 0,
		white_castle: 0b11,
		black_castle: 0b11
	};


	let mut input;

	loop {
		let mut rng = rand::thread_rng();
		input = read_line();
		let w_move = from_alg(input);

		//println!("{:?}", w_move);

		let temp = state.board[w_move.start as usize];
		state.board[w_move.start as usize] = Pieces::NONE;
		state.board[w_move.target as usize] = temp;

		print_board(state.board);

		let moves = move_generation::generate_moves(&state, Pieces::BLACK);

		let picked = &moves[rng.gen_range(0..moves.len())];
	  println!("Picked move: {}", to_alg(picked));
	  println!("{:?}",picked);
	  let temp = state.board[picked.start as usize];
	  state.board[picked.start as usize] = Pieces::NONE;
	  state.board[picked.target as usize] = temp;

	  print_board(state.board);
	}

	// let mut rng = rand::thread_rng();

	// print_board(board);
	
	// let b_score = score_board(board, Pieces::BLACK);
	// println!("{0}",b_score);

	// let w_score = score_board(board, Pieces::WHITE);
	// println!("{0}",w_score);

	// let mut side: u8;
	// let mut oom = false;

	// for i in 0..30 {
	//   if i%2 ==0 {
	//     side = Pieces::WHITE;
	//     println!("Side to go: White");
	//   } else {
	//     side = Pieces::BLACK;
	//     println!("Side to go: Black");
	//   }

	//   let moves = move_generation::generate_moves(board, side);
	//   //println!("{:?}",moves);
	//   if moves.len() == 0 {
	//     if oom { 
	//       println!("Got to {0} moves before end.", i);
	//       break; 
	//     }
	//     println!("No moves");
	//     oom = true;
	//     continue;
	//   } else {
	//     oom = false;
	//   }

	//   let picked = &moves[rng.gen_range(0..moves.len())];
	//   print!("Picked move: ");
	//   println!("{:?}",picked);
	//   let temp = board[picked.start as usize];
	//   board[picked.start as usize] = Pieces::NONE;
	//   board[picked.target as usize] = temp;

	//   print_board(board);
	// }

}