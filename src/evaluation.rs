use crate::constants::*;

// pub fn score_board(state: &State, side: u8) -> i32 {

// 	let mut totals: [i32; 2] = [0,0];

// 	for square in state.board {
// 		if square != 0 {
// 			let tside = (square>>4) as usize;
// 			match square&0b00111 {
// 				Pieces::PAWN => totals[tside] += Values::PAWN,
// 				Pieces::KNIGHT => totals[tside] += Values::KNIGHT,
// 				Pieces::BISHOP => totals[tside] += Values::BISHOP,
// 				Pieces::ROOK => totals[tside] += Values::ROOK,
// 				Pieces::QUEEN => totals[tside] += Values::QUEEN,
// 				_ => ()
// 			}
// 		}
// 	}

// 	let first = side >> 4;
// 	let score = totals[first as usize] - totals[(1-first) as usize];

// 	score

// }

pub fn score_board(state: &State, side: u8) -> i32 {

	let mut white_total:i32 = 0;
	let mut black_total:i32 = 0;

	white_total += (state.piece_bbs[1]&state.side_bbs[0]).count_ones() as i32 * Values::PAWN;
	black_total += (state.piece_bbs[1]&state.side_bbs[1]).count_ones() as i32 * Values::PAWN;

	white_total += (state.piece_bbs[2]&state.side_bbs[0]).count_ones() as i32 * Values::KNIGHT;
	black_total += (state.piece_bbs[2]&state.side_bbs[1]).count_ones() as i32 * Values::KNIGHT;

	white_total += (state.piece_bbs[3]&state.side_bbs[0]).count_ones() as i32 * Values::BISHOP;
	black_total += (state.piece_bbs[3]&state.side_bbs[1]).count_ones() as i32 * Values::BISHOP;

	white_total += (state.piece_bbs[4]&state.side_bbs[0]).count_ones() as i32 * Values::ROOK;
	black_total += (state.piece_bbs[4]&state.side_bbs[1]).count_ones() as i32 * Values::ROOK;

	white_total += (state.piece_bbs[5]&state.side_bbs[0]).count_ones() as i32 * Values::QUEEN;
	black_total += (state.piece_bbs[5]&state.side_bbs[1]).count_ones() as i32 * Values::QUEEN;

	if(side >> 4 == 0) {
		return white_total - black_total;
	} else {
		return black_total - white_total;
	} 
}