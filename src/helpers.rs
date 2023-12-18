use std::io;
use crate::constants::State;
use std::collections::HashMap;
use crate::constants::Pieces;
use crate::constants::Move;

//TODO: Implement parsing of castling state
pub fn parse_fen(input: &str) -> State {

  let mut state = State{
    board: [0;64],
    to_move: Pieces::WHITE,
    moves_made: 0,
    castle: 0b1111,
    piece_bbs: [0;6],
    side_bbs: [0;2]
  };
  
  let parts: Vec<&str> = input.split(' ').collect();

  if parts.len() != 6 {
    panic!("Invalid FEN String!");
  }

  state.board = parse_pieces(parts[0]);
  let arrs = buildbbs(&state.board);

  state.piece_bbs = arrs.0;
  state.side_bbs = arrs.1;

  if parts[1] == "b" {
    state.to_move = Pieces::BLACK;
  }

  if parts[2] == "-" {
    state.castle = 0;
  }

  state.moves_made = parts[5].parse::<u32>().unwrap();
  
  return state;
}

pub fn print_board(board: [u8; 64]) {
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

pub fn read_line() -> String {
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

pub fn to_alg(play: &Move) -> String {

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

pub fn from_alg(play: String) -> Move {

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
    target: t_index as u8,
    castle: 0
  }
}

fn parse_pieces(input: &str) -> [u8; 64] {

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
  board
}

//TODO more castling
pub fn make_move(state: &State, mv: Move, side: u8) -> State{

  let mut nstate = *state;

  if mv.castle == 0 {
    let temp = nstate.board[mv.start as usize];

    //Set castling ability
    let mut filter = 0b0011;
    if side == Pieces::BLACK { filter = filter << 2; }

    if nstate.castle&filter != 0 {
      if temp&Pieces::KING != 0 {
        if side == Pieces::WHITE {
          nstate.castle = nstate.castle&0b1100;
        } else {
          nstate.castle = nstate.castle&0b0011;
        }
      }

      if temp&Pieces::ROOK != 0 {
        if side == Pieces::WHITE {
          if mv.start == 0 { nstate.castle = nstate.castle&0b1101; }
          if mv.start == 7 { nstate.castle = nstate.castle&0b1110; }
        } else {
          if mv.start == 56 { nstate.castle = nstate.castle&0b0111; }
          if mv.start == 64 { nstate.castle = nstate.castle&0b1011; }
        }
      }
    }

    //Update bitboards
    nstate.piece_bbs[((temp&0b111) - 1) as usize] ^= (1 << mv.start);
    nstate.piece_bbs[((temp&0b111) - 1) as usize] |= (1 << mv.target);

    if(nstate.board[mv.target as usize] != 0) {
      let e_type = nstate.board[mv.target as usize] & 0b111;
      nstate.piece_bbs[(e_type - 1) as usize] ^= (1 << mv.target);
      nstate.side_bbs[1-(side >> 4) as usize] ^= (1 << mv.target);
    }

    //Update Board
    nstate.board[mv.start as usize] = Pieces::NONE;
    nstate.board[mv.target as usize] = temp;

    if side == Pieces::WHITE {
      nstate.side_bbs[0] ^= (1 << mv.start);
      nstate.side_bbs[0] |= (1 << mv.target);
      nstate.to_move = Pieces::BLACK;
    } else {
      nstate.side_bbs[1] ^= (1 << mv.start);
      nstate.side_bbs[1] |= (1 << mv.target);
      nstate.to_move = Pieces::WHITE;
      nstate.moves_made += 1;
    }

  }

  nstate
}

pub fn isInCheck(board: &[u8;64], side: u8, loc: u8) -> (bool,u8) {

  for i in 1..8 {
    
  }

  (false,0)
}

fn buildbbs(board: &[u8;64]) -> ([u64; 6], [u64; 2]) {
  let mut piece_bbs:[u64;6] = [0;6];
  let mut side_bbs:[u64;2] = [0; 2];

  for i in 0..64 {
    let cur_bit:u64 = 1 << (i as u64);
    let side:usize = ((board[i] & Pieces::BLACK) >> 4) as usize;
    match board[i]&0b00111 {
      Pieces::PAWN => {
        let bbi:usize = (Pieces::PAWN - 1) as usize;
        piece_bbs[bbi] = piece_bbs[bbi] | cur_bit;
        side_bbs[side] = side_bbs[side] | cur_bit;
      }
      Pieces::KING => {
        let bbi:usize = (Pieces::KING - 1) as usize;
        piece_bbs[bbi] = piece_bbs[bbi] | cur_bit;
        side_bbs[side] = side_bbs[side] | cur_bit;
      }
      Pieces::KNIGHT => {
        let bbi:usize = (Pieces::KNIGHT - 1) as usize;
        piece_bbs[bbi] = piece_bbs[bbi] | cur_bit;
        side_bbs[side] = side_bbs[side] | cur_bit;
      }
      Pieces::ROOK => {
        let bbi:usize = (Pieces::ROOK - 1) as usize;
        piece_bbs[bbi] = piece_bbs[bbi] | cur_bit;
        side_bbs[side] = side_bbs[side] | cur_bit;
      }
      Pieces::BISHOP => {
        let bbi:usize = (Pieces::BISHOP - 1) as usize;
        piece_bbs[bbi] = piece_bbs[bbi] | cur_bit;
        side_bbs[side] = side_bbs[side] | cur_bit;      }
      Pieces::QUEEN => {
        let bbi:usize = (Pieces::QUEEN - 1) as usize;
        piece_bbs[bbi] = piece_bbs[bbi] | cur_bit;
        side_bbs[side] = side_bbs[side] | cur_bit;
      }
      _ => ()
    }
  }

  return(piece_bbs, side_bbs);
}

fn setupbbs(state: &mut State) {
  state.piece_bbs[0] = 0x1000000000000010;
  state.piece_bbs[1] = 0x00FF00000000FF00;
  state.piece_bbs[2] = 0x4200000000000042;
  state.piece_bbs[3] = 0x2400000000000024;
  state.piece_bbs[4] = 0x8100000000000081;
  state.piece_bbs[5] = 0x0800000000000008;

  state.side_bbs[0] = 0x000000000000FFFF;
  state.side_bbs[1] = 0xFFFF000000000000;
}