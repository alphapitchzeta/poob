use crate::{Color, Piece};
use crate::bitboard_types::Square;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MailboxPiece {
    color: Color,
    piece: Piece,
}

impl MailboxPiece {
    fn new(color: Color, piece: Piece) -> Self {
        Self { color, piece }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mailbox([Option<MailboxPiece>; 64]);

impl Mailbox {
    pub fn new() -> Self {
        Self([None; 64])
    }

    pub fn piece_at(&self, square: Square) -> Option<(Color, Piece)> {
        let piece = self.0[square as usize]?;

        Some((piece.color, piece.piece))
    }

    pub fn set_piece(&mut self, piece: (Color, Piece), square: Square) {
        self.0[square as usize] = Some(MailboxPiece::new(piece.0, piece.1));
    }

    pub fn clear_square(&mut self, square: Square) {
        self.0[square as usize] = None;
    }

    pub fn square_is_occupied(&self, square: Square) -> bool {
        self.0[square as usize] != None
    }

    pub fn move_piece(&mut self, i_square:Square, t_square:Square) {
        self.0[t_square as usize] = self.0[i_square as usize];
        self.0[i_square as usize] = None;
    }

    pub fn castle_kingside_white(&mut self) {
        self.move_piece(Square::E1, Square::G1);
        self.move_piece(Square::H1, Square::F1);
    }

    pub fn castle_kingside_black(&mut self) {
        self.move_piece(Square::E8, Square::G8);
        self.move_piece(Square::H8, Square::F8);
    }

    pub fn castle_queenside_white(&mut self) {
        self.move_piece(Square::E1, Square::C1);
        self.move_piece(Square::A1, Square::D1);
    }

    pub fn castle_queenside_black(&mut self) {
        self.move_piece(Square::E8, Square::C8);
        self.move_piece(Square::A8, Square::D8);
    }
}
