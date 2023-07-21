mod evaluation;
mod constants;

use std::collections::HashMap;
use constants::Pieces;
use evaluation::score_board;



fn parse_fen(input: &str) -> [u8; 64] {
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
  return board;
}

fn print_board(board: [u8; 64]) {
  let mut file: usize = 0;
  let mut rank: usize = 7;

  loop {
    if file == 8 {
      print!("\n");
      if rank == 0 {
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


fn main() {
  let starting = "rnbqkbnr/ppqppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

  let board: [u8; 64] = parse_fen(starting);

  print_board(board);
  
  let b_score = score_board(board, Pieces::BLACK);
  println!("{0}",b_score);

  let w_score = score_board(board, Pieces::WHITE);
  println!("{0}",w_score);
}