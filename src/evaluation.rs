use crate::constants::*;

pub fn score_board(state: &State, side: u8) -> i32 {

	let mut totals: [i32; 2] = [0,0];

	for square in state.board {
		if square != 0 {
			let tside = (square>>4) as usize;
			match square&0b00111 {
				Pieces::PAWN => totals[tside] += Values::PAWN,
				Pieces::KNIGHT => totals[tside] += Values::KNIGHT,
				Pieces::BISHOP => totals[tside] += Values::BISHOP,
				Pieces::ROOK => totals[tside] += Values::ROOK,
				Pieces::QUEEN => totals[tside] += Values::QUEEN,
				_ => ()
			}
		}
	}

	let first = side >> 4;
	let score = totals[first as usize] - totals[(1-first) as usize];

	score

}