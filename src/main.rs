#![allow(dead_code)]

mod evaluation;
mod constants;
mod move_generation;

use std::collections::HashMap;
use constants::Pieces;
use evaluation::score_board;
use move_generation::Move;
use rand::Rng;
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

fn to_alg(play: &Move) -> String {

	let mut alg = String::new();

	let alf = ['A','B','C','D','E','F','G','H'];

	let s_letter = alf[(play.start%8) as usize];
	let s_num = char::from_digit((play.start as u32/8)+1, 10).unwrap_or('0');

	alg.push(s_letter);
	alg.push(s_num);

	let t_letter = alf[(play.target%8) as usize];
	let t_num = char::from_digit((play.target as u32/8)+1, 10).unwrap_or('0');

	alg.push(t_letter);
	alg.push(t_num);

	alg
}

fn from_alg(play: String) -> Move {

	let mut s_index = 0;
	let mut t_index = 0;

	let alf = ['a','b','c','d','e','f','g','h'];

	for (i, x) in play.chars().enumerate() {
		if i < 2 {
			if x.is_numeric() {
				s_index += (x.to_digit(10).unwrap_or(0)-1)*8;
			} else {
				s_index += alf.iter().position(|&r| r == x).unwrap() as u32;
			}
		} else {
			if x.is_numeric() {
				t_index += (x.to_digit(10).unwrap_or(0)-1)*8;
			} else {
				t_index += alf.iter().position(|&r| r == x).unwrap() as u32;
			}
		}
	}

	Move {
		start: s_index as u8,
		target: t_index as u8
	}
}


fn main() {
	let starting = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

	let mut board: [u8; 64] = parse_fen(starting);


	let mut input;

	loop {
		let mut rng = rand::thread_rng();
		input = read_line();
		let w_move = from_alg(input);

		//println!("{:?}", w_move);

		let temp = board[w_move.start as usize];
		board[w_move.start as usize] = Pieces::NONE;
		board[w_move.target as usize] = temp;

		print_board(board);

		let moves = move_generation::generate_moves(board, Pieces::BLACK);

		let picked = &moves[rng.gen_range(0..moves.len())];
	  println!("Picked move: {}", to_alg(picked));
	  println!("{:?}",picked);
	  let temp = board[picked.start as usize];
	  board[picked.start as usize] = Pieces::NONE;
	  board[picked.target as usize] = temp;

	  print_board(board);
	}

	// if input != "uci" { panic!("Wrong start"); }
	// println!("id name BespokeSloth");
	// println!("id author neonstatic");
	// println!("uciok");

	// input = read_line();
	// if input != "isready" { panic!("Not ready"); }
	// println!("readyok");

	// input = read_line();
	// if input != "ucinewgame" { panic!("no new game"); }

	// input = read_line();
	// if !input.contains("position startpos") { panic!("bad position"); }
	// let parts = input.split(" ");

	

	// while input != "quit" {
	// 	input = read_line();
	// 	println!("{0}", input);
	// }

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