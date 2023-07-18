use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq)]
enum Pieces {
    None,
    King,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen
}
#[derive(Copy, Clone, PartialEq)]
enum Colors {
  White,
  Black
}

#[derive(Copy, Clone)]
struct Square{
  piece: Pieces,
  color: Colors
}


fn parse_fen(input: &str) -> [Square; 64] {
  let hash = HashMap::from([('k',Pieces::King), ('n', Pieces::Knight), ('p', Pieces::Pawn), 
                            ('b', Pieces::Bishop), ('r', Pieces::Rook), ('q', Pieces::Queen),
                            ('K',Pieces::King), ('N', Pieces::Knight), ('P', Pieces::Pawn), 
                            ('B', Pieces::Bishop), ('R', Pieces::Rook), ('Q', Pieces::Queen)]);

  let mut board: [Square; 64] = [Square{piece: Pieces::None, color: Colors::Black};64];

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
        if c.is_uppercase() {
          let piece = hash[&c];
          let color = Colors::White;
          board[rank*8+file] = Square {piece, color};
        } else {
          let piece = hash[&c];
          let color = Colors::Black;
          board[rank*8+file] = Square {piece, color};
        }
        file += 1;
      }
    }
  }
  return board;
}

fn print_board(board: [Square; 64]) {
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


    let current: Square = board[rank*8+file];
    let letter: char;

    match current.piece {
      Pieces::Pawn => letter = 'p',
      Pieces::Rook => letter = 'r',
      Pieces::King => letter = 'k',
      Pieces::Queen => letter = 'q',
      Pieces::Bishop => letter = 'b',
      Pieces::Knight => letter = 'n',
      Pieces::None => letter = '-'
    }
    if current.color == Colors::White {
      print!("{}", letter.to_uppercase());
    } else {
      print!("{}", letter);
    }

    file += 1;
  }
}


fn main() {
  let starting = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

  let mut board: [Square; 64] = parse_fen(starting);

  print_board(board);

}