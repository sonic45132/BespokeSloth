#[non_exhaustive]
pub struct Pieces;

impl Pieces {
    pub const NONE: u8 = 0;
    pub const KING: u8 = 1;
    pub const PAWN: u8 = 2;
    pub const KNIGHT: u8 = 3;
    pub const BISHOP: u8 = 4;
    pub const ROOK: u8 = 5;
    pub const QUEEN: u8 = 6;
    pub const WHITE: u8 = 8;
    pub const BLACK: u8 = 16;
}

#[non_exhaustive]
pub struct Values;

impl Values {
  pub const PAWN: i32 = 10;
  pub const KNIGHT: i32 = 30;
  pub const BISHOP: i32 = 30;
  pub const ROOK: i32 = 50;
  pub const QUEEN: i32 = 90;
}

#[derive(Debug,Clone,Copy)]
pub struct Move {
  pub start: u8,
  pub target: u8,
  pub castle: u8
}

#[derive(Debug,Clone,Copy)]
pub struct State {
  pub board: [u8; 64],
  pub to_move: u8,
  pub moves_made: u32,
  pub castle: u8,
}