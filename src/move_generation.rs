use crate::constants::Pieces;
use crate::constants::Move;
use crate::constants::State;

pub fn generate_moves(state: &State, side: u8) -> Vec<Move> {

	let mut moves: Vec<Move> = Vec::new();
	let board = state.board;

	for i in 0..64 {
		let square = board[i];
		if square&side == 0 { continue; }

		match square&0b00111 {
			Pieces::PAWN => moves.extend(pawn_moves(board, side, i as u8)),
			Pieces::KING => moves.extend(king_moves(state, side, i as u8)),
			Pieces::KNIGHT => moves.extend(knight_moves(board, side, i as u8)),
			Pieces::ROOK => moves.extend(slide_moves(board, side, i as u8, Pieces::ROOK)),
			Pieces::BISHOP => moves.extend(slide_moves(board, side, i as u8, Pieces::BISHOP)),
			Pieces::QUEEN => moves.extend(slide_moves(board, side, i as u8, Pieces::QUEEN)),
			_ => ()
		}

	}

	moves
}

//TODO: Look into cleaning this up
fn pawn_moves(board: [u8;64], side: u8, loc: u8) -> Vec<Move> {

	let cur_pos = loc as i32;

	let mut moves: Vec<Move> = Vec::new();

	
	let offset;

	if side == Pieces::WHITE {
		offset = 8;
	} else {
		offset = -8;
	}

	let target = cur_pos+offset;

	if target > 63 || target < 0 {
		return moves;
	}

	let index = target as usize;

	//Generate normal pawn move 1 space
	if board[index] == 0 {
		moves.push(Move {
			start: loc as u8,
			target: (cur_pos+offset) as u8,
			castle: 0
		});

		//Generate starting pawn move of 2 spaces if on correct rank
		let index2 = (cur_pos+(offset*2)) as usize;
		if side == Pieces::WHITE {
			if loc/8 == 1 && board[index2] == 0 {
				moves.push(Move {
					start: loc as u8,
					target: (cur_pos+(offset*2)) as u8,
					castle: 0
				});
			}
		} else {
			if loc/8 == 7 && board[index2] == 0 {
				moves.push(Move {
					start: loc as u8,
					target: (cur_pos+(offset*2)) as u8,
					castle: 0
				});
			}
		}
	}

	//Check for capture moves one left and one right of the square ahead
	if index!= 0 && index/8 == (index-1)/8 {
		if board[index-1] != 0 {
			if board[index-1]&side == 0 {
				moves.push(Move {
					start: loc as u8,
					target: (index-1) as u8,
					castle: 0
				});
			}
		}
	}

	if index/8 == (index+1)/8 && (index+1) < 64{
		if board[index+1] != 0 {
			if board[index+1]&side == 0 {
				moves.push(Move {
					start: loc as u8,
					target: (index+1) as u8,
					castle: 0
				});
			}
		}
	}

	moves
}

fn king_moves(state: &State, side: u8, loc: u8) -> Vec<Move> {

	let mut moves: Vec<Move> = Vec::new();

	let offsets = [-8,-1,1,8];

	for offset in offsets {
		let cur_pos = loc as i32;
		if cur_pos + offset > 63 || cur_pos + offset < 0 { continue; }
		if offset.abs() == 1 && (cur_pos + offset)/8  != cur_pos/8 { continue; }

		let index = (cur_pos+offset) as usize;
		if state.board[index]&side == 0 {
			moves.push(Move {
				start: loc,
				target: index as u8,
				castle: 0
			});
		}

	}

	//Castling - Verify
	let mut filter = 0b0001;
	if side == Pieces::BLACK { filter = filter << 2; }

	if state.castle != 0 {
	  let index = loc as usize;
	  if state.castle & filter != 0 {
      if state.board[index+1] | state.board[index+2] == 0 {
        moves.push(Move {
          start: loc,
          target: loc+2,
          castle: 1
        });
      }
    } else if state.castle & (filter << 1) != 0 {
      if (state.board[index-1] | state.board[index-2] | state.board[index-3]) == 0 {
        moves.push(Move {
          start: loc,
          target: loc-2,
          castle: 2
        });
      }
    }
	}

	moves
}

fn knight_moves(board: [u8;64], side: u8, loc: u8) -> Vec<Move> {

	let mut moves: Vec<Move> = Vec::new();

	let offsets = [-17,-15,-10,-6,6,10,15,17];

	for offset in offsets {
		let cur_pos = loc as i32;
		let target = cur_pos + offset;
		if target > 63 || target < 0 { continue; }


		let t_rank = target/8;
		let c_rank = cur_pos/8;
		let r_diff = (t_rank-c_rank).abs();
		if (offset < -14 || offset > 14) && r_diff != 2 { continue; }
		if (offset > -14 && offset < 14) && r_diff != 1 { continue; }
		
		let index = (cur_pos+offset) as usize;
		if board[index]&side == 0 {
			moves.push(Move {
				start: loc,
				target: index as u8,
				castle: 0
			});
		}

	}

	//println!("{:?}",moves);

	moves
}

fn slide_moves(board: [u8;64], side: u8, loc: u8, piece: u8) -> Vec<Move> {

	let mut moves: Vec<Move> = Vec::new();
	let cur_pos = loc as i32;

	let dirs: Vec<i32>;

	match piece {
		Pieces::QUEEN => dirs = vec![-9,-8,-7,-1,1,7,8,9],
		Pieces::ROOK => dirs = vec![-8,-1,1,8],
		Pieces::BISHOP => dirs = vec![-9,-7,7,9],
		_ => dirs = Vec::new()
	}
	
	for dir in dirs {
		let mut i = 1;
		loop {
			let target = cur_pos + (dir*i);
			if target > 63 || target < 0 { break; }
			let t_rank = target/8;
			let c_rank = (cur_pos + (dir*(i-1)))/8;
			let r_diff = (t_rank-c_rank).abs();
			if (dir < -6 || dir > 6) && r_diff != 1 { break; }
			if (dir > -6 && dir < 6) && r_diff != 0 { break; }

			let index = target as usize;

			if board[index] != 0  && board[index]&side == 0 {
				moves.push(Move {
					start: loc,
					target: target as u8,
					castle: 0
				});
				break;
			} else if board[index]&side != 0 {
				break;
			} else {
				moves.push(Move {
					start: loc,
					target: target as u8,
					castle: 0
				});
			}

			i += 1;

		}

	}

	//println!("{:?}",moves);

	moves
}