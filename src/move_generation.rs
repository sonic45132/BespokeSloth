use crate::constants::Pieces;

#[derive(Debug)]
pub struct Move {
  pub start: u8,
  pub target: u8
}


pub fn generate_moves(board: [u8;64], side: u8) -> Vec<Move> {

  let mut moves: Vec<Move> = Vec::new();

  for i in 0..64 {
    let square = board[i];
    if square&side ==0 { continue; }

    match square&0b00111 {
      Pieces::PAWN => moves.extend(pawn_moves(board, side, i as u8)),
      _ => ()
    }

  }

  moves
}

fn pawn_moves(board: [u8;64], side: u8, loc: u8) -> Vec<Move> {

  if loc > 64 {
    return Vec::new();
  }

  let mut moves: Vec<Move> = Vec::new();

  let mut offset: i32 = 8;
  if side == Pieces::BLACK { offset = -8; }

  let index = (loc as i32+offset) as usize;

  if index > 63 {
    return moves;
  }

  //Generate normal pawn move 1 space
  if board[index] == 0 {
    moves.push(Move {
      start: loc as u8,
      target: (loc as i32+offset) as u8
    });

    //Generate starting pawn move of 2 spaces if on correct rank
    let index2 = (loc as i32+(offset*2)) as usize;
    if side == Pieces::WHITE {
      if loc/8 == 1 && board[index2] == 0 {
        moves.push(Move {
          start: loc as u8,
          target: (loc as i32+(offset*2)) as u8
        });
      }
    } else {
      if loc/7 == 1 && board[index2] == 0 {
        moves.push(Move {
          start: loc as u8,
          target: (loc as i32+(offset*2)) as u8
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
          target: (index-1) as u8
        });
      }
    }
  }

  if index/8 == (index+1)/8 && (index+1) < 64{
    if board[index+1] != 0 {
      if board[index+1]&side == 0 {
        moves.push(Move {
          start: loc as u8,
          target: (index+1) as u8
        });
      }
    }
  }



  moves
}