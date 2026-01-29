pub mod bitboards;
pub mod boardstate;
pub mod game;
pub mod movegen;
pub mod moves;
pub mod rende;
pub mod util;
pub mod perft;

use crate::bitboards::bitboard_constants::bitboard_indices::*;

/// Represents one of the two playable colors (white or black).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

impl Color {
    /// Helper method that maps each enum variant to its corresponding
    /// bitboard index constant.
    pub fn to_index(&self) -> usize {
        match self {
            Color::White => WHITE,
            Color::Black => BLACK,
        }
    }

    /// Helper method that returns the enemy color.
    pub fn enemy(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

/// Represents the possible piece types.
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
    /// Helper method that maps each enum variant to its corresponding
    /// bitboard index constant.
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
