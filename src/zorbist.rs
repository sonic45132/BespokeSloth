use rand::rngs::StdRng;
use rand::prelude::*;
use crate::constants::*;


pub fn generate_keys(seed: u64) -> [u64; 781] {

  let mut keys = [0; 781];

  let mut rng = StdRng::seed_from_u64(seed);

  for n in 0..781 {
    keys[n] = rng.gen();
  }

  return keys;
}

pub fn generate_hash(state: State, keys: [u64; 781]) -> u64 {

  let mut hash = 0;

  for i in 0..64 {
    let square = state.board[i as usize];
    if square != 0 {
      let piece = (square&0b111) - 1;
      let side = (square&0b11000) >> 3;
      hash = hash^keys[(i+(piece*side)) as usize];
    }
  }

  if state.to_move == Pieces::BLACK { hash = hash ^ keys[768]; }


  for n in 0..4 {
    if state.castle&(0b1<<n) != 0 {
      hash = hash^keys[769+n];
    }
  }

  return hash;
}