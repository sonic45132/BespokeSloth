use rand::rngs::StdRng;
use rand::prelude::*;
use crate::constants::*;


pub fn generate_keys(seed: u64) -> [u64; 781] {

  let mut keys = [0; 781];

  let mut rng = StdRng::seed_from_u64(seed);

  for n in 0..781 {
    keys[n] = rng.gen();
    println!("{:?}", keys[n]);
  }

  return keys;
}

pub fn generate_hash(state: State) -> u64 {

  let mut hash = 0;

  //Pices on Board
  for i in 0..64 {
    let square = state.board[i as usize];
    if square != 0 {
      let piece = (square&0b111) - 1;
      let side = (square&0b11000) >> 4;
      hash ^= state.zkeys[(i*12)+(piece+(6*side)) as usize];
    }
  }

  //Black to Move
  if state.to_move == Pieces::BLACK { hash ^= state.zkeys[768]; }

  //Castling
  for n in 0..4 {
    if state.castle&(0b1<<n) != 0 {
      hash ^= keys[769+n];
    }
  }

  //TODO: En Passant

  return hash;
}