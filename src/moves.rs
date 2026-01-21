// Constants representing the different arrangements of bit flags
// for the four leftmost bits of a u16 representing a move.
pub mod move_constants {
    pub const QUIET_MOVE: u16 = 0b0000 << 12;
    pub const DOUBLE_PAWN_PUSH: u16 = 0b0001 << 12;
    pub const KING_CASTLE: u16 = 0b0010 << 12;
    pub const QUEEN_CASTLE: u16 = 0b0011 << 12;
    pub const CAPTURE: u16 = 0b0100 << 12;
    pub const EN_PASSANT_CAPTURE: u16 = 0b0101 << 12;
    pub const KNIGHT_PROMOTION: u16 = 0b1000 << 12;
    pub const BISHOP_PROMOTION: u16 = 0b1001 << 12;
    pub const ROOK_PROMOTION: u16 = 0b1010 << 12;
    pub const QUEEN_PROMOTION: u16 = 0b1011 << 12;
    pub const KNIGHT_PROMOTION_CAPTURE: u16 = 0b1100 << 12;
    pub const BISHOP_PROMOTION_CAPTURE: u16 = 0b1101 << 12;
    pub const ROOK_PROMOTION_CAPTURE: u16 = 0b1110 << 12;
    pub const QUEEN_PROMOTION_CAPTURE: u16 = 0b1111 << 12;
}

use crate::util::*;
use move_constants::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Move(u16);

impl Move {
    /// Returns an optional move instance from square indices.
    pub fn from_squares(initial_square: u8, target_square: u8) -> Option<Self> {
        let initial_square = checked_square_u8_to_square_u16(initial_square)?;
        let target_square = checked_square_u8_to_square_u16(target_square)?;

        Some(Self((initial_square << 6) | target_square))
    }

    /// Returns an optional move instance from square string slices.
    pub fn from_squares_str(initial_square: &str, target_square: &str) -> Option<Self> {
        let i_square_index = square_str_to_index(initial_square)?;
        let t_square_index = square_str_to_index(target_square)?;

        let new_move = Move::from_squares(i_square_index, t_square_index)?;

        Some(new_move)
    }

    pub fn get_initial_square(&self) -> u8 {
        ((self.0 >> 6) & 0b111111) as u8
    }

    pub fn get_target_square(&self) -> u8 {
        (self.0 & 0b111111) as u8
    }

    pub fn is_quiet(&self) -> bool {
        self.0 & QUIET_MOVE == QUIET_MOVE
    }

    pub fn is_double_pawn_push(&self) -> bool {
        self.0 & DOUBLE_PAWN_PUSH == DOUBLE_PAWN_PUSH
    }

    pub fn is_king_castle(&self) -> bool {
        self.0 & KING_CASTLE == KING_CASTLE
    }

    pub fn is_queen_castle(&self) -> bool {
        self.0 & QUEEN_CASTLE == QUEEN_CASTLE
    }

    pub fn is_capture(&self) -> bool {
        self.0 & CAPTURE == CAPTURE
    }

    pub fn is_en_passant_capture(&self) -> bool {
        self.0 & EN_PASSANT_CAPTURE == EN_PASSANT_CAPTURE
    }

    pub fn is_knight_promotion(&self) -> bool {
        self.0 & KNIGHT_PROMOTION == KNIGHT_PROMOTION
    }

    pub fn is_bishop_promotion(&self) -> bool {
        self.0 & BISHOP_PROMOTION == BISHOP_PROMOTION
    }

    pub fn is_rook_promotion(&self) -> bool {
        self.0 & ROOK_PROMOTION == ROOK_PROMOTION
    }

    pub fn is_queen_promotion(&self) -> bool {
        self.0 & QUEEN_PROMOTION == QUEEN_PROMOTION
    }

    pub fn is_knight_promotion_capture(&self) -> bool {
        self.0 & KNIGHT_PROMOTION_CAPTURE == KNIGHT_PROMOTION_CAPTURE
    }

    pub fn is_bishop_promotion_capture(&self) -> bool {
        self.0 & BISHOP_PROMOTION_CAPTURE == BISHOP_PROMOTION_CAPTURE
    }

    pub fn is_rook_promotion_capture(&self) -> bool {
        self.0 & ROOK_PROMOTION_CAPTURE == ROOK_PROMOTION_CAPTURE
    }

    pub fn is_queen_promotion_capture(&self) -> bool {
        self.0 & QUEEN_PROMOTION_CAPTURE == QUEEN_PROMOTION_CAPTURE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_squares() {
        let i_square = 8;
        let t_square = 24;

        assert_eq!(Move::from_squares(i_square, t_square), Some(Move(0b00000010_00011000)));
    }
}
