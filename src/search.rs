use std::cmp;
use crate::constants::State;
use crate::constants::Pieces;
use crate::constants::Move;
use crate::helpers::*;
use crate::move_generation::*;
use crate::evaluation::score_board;

pub fn minimax(state: &State, side: u8, depth: u32) -> Move {

  let mut best_move = Move {
    start: 0,
    target: 0,
    castle: 0
  };

  if side == Pieces::WHITE {

    let mut max = i32::MIN;
    let moves = generate_moves(state, side);
    for mv in moves {
      let tstate = make_move(state, mv, side);
      let eval = minimax_v(&tstate, Pieces::BLACK, depth-1);
      println!("Move: {} scored: {}",to_alg(&mv), eval);
      if eval > max {
        max = eval;
        best_move = mv
      }
    }
    return best_move;

  } else {
    let mut max = i32::MIN;
    let moves = generate_moves(state, side);
    for mv in moves {
      let tstate = make_move(state, mv, side);
      let eval = minimax_v(&tstate, Pieces::WHITE, depth-1);
      println!("Move: {} scored: {}",to_alg(&mv), eval);
      if eval > max {
        max = eval;
        best_move = mv
      }
    }
    return best_move;
  }
}

fn minimax_v(state: &State, side: u8, depth: u32) -> i32 {

  if depth == 0 {
    let score = score_board(state, side);
    return score;
  }

  if side == Pieces::WHITE {
    let mut max = i32::MIN;
    let moves = generate_moves(state, side);
    for mv in moves {
      let tstate = make_move(state, mv, side);
      let eval = minimax_v(&tstate, Pieces::BLACK, depth-1);
      println!("Move: {} scored: {}",to_alg(&mv), eval);
      max = cmp::max(max,eval);
    }
    return max;
  } else {
    let mut max = i32::MIN;
    let moves = generate_moves(state, side);
    for mv in moves {
      let tstate = make_move(state, mv, side);
      let eval = minimax_v(&tstate, Pieces::BLACK, depth-1);
      println!("Move: {} scored: {}",to_alg(&mv), eval);
      max = cmp::max(max,eval);
    }
    return max;
  }
}