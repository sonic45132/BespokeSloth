//use std::cmp;
use rand::Rng;
use crate::constants::State;
use crate::constants::Pieces;
use crate::constants::Move;
use crate::helpers::*;
use crate::move_generation::*;
use crate::evaluation::score_board;

pub fn negamax(state: &State, side: u8, depth: u32) -> Move {

  let mut best_moves: Vec<Move> = Vec::new();

  let n_side;
  if side == Pieces::WHITE {
    n_side = Pieces::BLACK;
  } else {
    n_side = Pieces::WHITE;
  }

  let mut alpha = -2000000000;
  let beta = 2000000000;

  let moves = generate_moves(state, side);
  //println!("{:?}",moves); 
  for mv in moves {
    let tstate = make_move(state, mv, side);
    let score = -negamax_v(&tstate, n_side, depth-1, -beta, -alpha);
    //println!("Move: {} scored: {}",to_alg(&mv), score);
    if score >= beta {
      return mv;
    }
    if score > alpha {
      alpha = score;
      best_moves.clear();
      best_moves.push(mv);
    } else if score == alpha {
      best_moves.push(mv);
    }
  }

  if best_moves.len() != 1 {
    let mut rng = rand::thread_rng();
    return best_moves[rng.gen_range(0..best_moves.len())];
  } else {
    return best_moves[0];
  }
}

fn negamax_v(state: &State, side: u8, depth: u32, alpha: i32, beta: i32) -> i32 {
  if depth == 0 {
    let score = score_board(state, side);
    return score;
  }

  let n_side;
  if side == Pieces::WHITE {
    n_side = Pieces::BLACK;
  } else {
    n_side = Pieces::WHITE;
  }

  let mut t_alpha = alpha;
  let moves = generate_moves(state, side);
  for mv in moves {
    let tstate = make_move(state, mv, side);
    let score = -negamax_v(&tstate, n_side, depth-1, -beta, -t_alpha);
    //println!("Move: {} scored: {}",to_alg(&mv), score);
    if score >= beta {
      return beta;
    } else if score > t_alpha {
      t_alpha = score;
    }
  }
  return t_alpha;
}