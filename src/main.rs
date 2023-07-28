#![allow(dead_code)]

mod evaluation;
mod constants;
mod move_generation;

use std::collections::HashMap;
use constants::Pieces;
//use evaluation::score_board;
use move_generation::Move;
//use rand::Rng;
use std::io;



fn parse_fen(input: &str) -> [u8; 64] {
	let hash = HashMap::from([('k',Pieces::KING), ('n', Pieces::KNIGHT), ('p', Pieces::PAWN), 
														('b', Pieces::BISHOP), ('r', Pieces::ROOK), ('q', Pieces::QUEEN)]);

	let mut board: [u8; 64] = [0;64];
	let mut file: usize = 0;
	let mut rank: usize = 7;

	for c in input.chars() {
		if c == '/' {
			file = 0;
			rank -= 1;
		} else {
			if c.is_digit(10) {
				file += c.to_digit(10).unwrap_or(0) as usize;
			} else {
				let lower = c.to_lowercase().last().unwrap_or('p');
				if c.is_uppercase() {
					let piece = hash[&lower];
					let color = Pieces::WHITE;
					board[rank*8+file] = (color|piece) as u8;
				} else {
					let piece = hash[&lower];
					let color = Pieces::BLACK;
					board[rank*8+file] = (color|piece) as u8;
				}
				file += 1;
			}
		}
	}
	return board;
}

fn print_board(board: [u8; 64]) {
	let mut file: usize = 0;
	let mut rank: usize = 7;

	loop {
		if file == 8 {
			print!("\n");
			if rank == 0 {
				print!("\n");
				break;
			}
			rank -=1;
			file = 0;
		}


		let current = board[rank*8+file];
		let letter: char;

		let piece = current & 0b00111;
		let color = current & 0b11000;

		match piece {
			Pieces::PAWN => letter = 'p',
			Pieces::ROOK => letter = 'r',
			Pieces::KING => letter = 'k',
			Pieces::QUEEN => letter = 'q',
			Pieces::BISHOP => letter = 'b',
			Pieces::KNIGHT => letter = 'n',
			Pieces::NONE => letter = '-',
			_ => letter = '-'
		}
		if color == Pieces::WHITE {
			print!("{}", letter.to_uppercase());
		} else {
			print!("{}", letter);
		}

		file += 1;
	}
}

fn read_line() -> String {
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("failed to readline");

	if let Some('\n')=input.chars().next_back() {
		input.pop();
	}
	if let Some('\r')=input.chars().next_back() {
		input.pop();
	}

	input
}

fn to_alg(play: Move) -> String {

	

	String::new()
}

fn from_alg(play: String) -> Move {

	for (i, x) in play.chars().enumerate() {
		if i < 2 {

		}
	}

	Move {
		start: 0,
		target: 0
	}
}


fn main() {
	let starting = "rnbqkbnr/ppqppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

	let mut _board: [u8; 64] = parse_fen(starting);


	let mut input = read_line();

	if input != "uci" { panic!("Wrong start"); }
	println!("id name BespokeSloth");
	println!("id author neonstatic");
	println!("uciok");

	input = read_line();
	if input != "isready" { panic!("Not ready"); }
	println!("readyok");

	input = read_line();
	if input != "ucinewgame" { panic!("no new game"); }

	input = read_line();
	if !input.contains("position startpos") { panic!("bad position"); }
	let parts = input.split(" ");

	

	while input != "quit" {
		input = read_line();
		println!("{0}", input);
	}

	//let mut rng = rand::thread_rng();

	//print_board(board);
	
	// let b_score = score_board(board, Pieces::BLACK);
	// println!("{0}",b_score);

	// let w_score = score_board(board, Pieces::WHITE);
	// println!("{0}",w_score);

	// let mut side: u8;
	// let mut oom = false;

	// for _i in 0..1000 {
	//   if _i%2 ==0 {
	//     side = Pieces::WHITE;
	//   } else {
	//     side = Pieces::BLACK;
	//   }

	//   let moves = move_generation::generate_moves(board, side);
	//   if moves.len() == 0 {
	//     if oom { 
	//       println!("Got to {0} moves before end.", _i);
	//       break; 
	//     }
	//     oom = true;
	//     continue;
	//   } else {
	//     oom = false;
	//   }

	//   let picked = &moves[rng.gen_range(0..moves.len())];
	//   let temp = board[picked.start as usize];
	//   board[picked.start as usize] = Pieces::NONE;
	//   board[picked.target as usize] = temp;
	// }

	// print_board(board);

}