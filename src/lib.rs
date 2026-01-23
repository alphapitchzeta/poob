pub mod bitboards;
pub mod boardstate;
pub mod game;
pub mod movegen;
pub mod moves;
pub mod rende;
pub mod util;

use crate::bitboards::bitboard_constants::bitboard_indices::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn to_index(&self) -> usize {
        match self {
            Color::White => WHITE,
            Color::Black => BLACK,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl Piece {
    pub fn to_index(&self) -> usize {
        match self {
            Piece::Pawn => PAWN,
            Piece::Rook => ROOK,
            Piece::Knight => KNIGHT,
            Piece::Bishop => BISHOP,
            Piece::Queen => QUEEN,
            Piece::King => KING,
        }
    }
}
