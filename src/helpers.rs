use std::io;
use crate::State;
use std::collections::HashMap;
use crate::constants::Pieces;
use crate::move_generation::Move;

//TODO: Implement parsing of castling state
pub fn parse_fen(input: &str) -> State {

  let mut state = State{
    board: [0;64],
    to_move: Pieces::WHITE,
    moves_made: 0,
    white_castle: 0b11,
    black_castle: 0b11
  };
  
  let parts: Vec<&str> = input.split(' ').collect();

  if parts.len() != 6 {
    panic!("Invalid FEN String!");
  }

  state.board = parse_pieces(parts[0]);

  if parts[1] == "b" {
    state.to_move = Pieces::BLACK;
  }

  if parts[2] == "-" {
    state.white_castle = 0;
    state.black_castle = 0;
  }

  state.moves_made = parts[6].parse::<u32>().unwrap();
  
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
    castle: false
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