use crate::constants::Values;
use crate::constants::Pieces;


pub fn score_board(board: [u8;64], side: u8) -> i32 {

  let mut totals: [i32; 2] = [0,0];

  for square in board {
    if square != 0 {
      match square&0b00111 {
        Pieces::PAWN => totals[(square>>4) as usize] += Values::PAWN,
        Pieces::KNIGHT => totals[(square>>4) as usize] += Values::KNIGHT,
        Pieces::BISHOP => totals[(square>>4) as usize] += Values::BISHOP,
        Pieces::ROOK => totals[(square>>4) as usize] += Values::ROOK,
        Pieces::QUEEN => totals[(square>>4) as usize] += Values::QUEEN,
        _ => ()
      }
    }
  }

  let first = side >> 4;
  let score = totals[first as usize] - totals[(1-first) as usize];
  score
}