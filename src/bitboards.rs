// Constants for initializing and interacting with bitboards
pub mod bitboard_constants {
    pub mod starting_positions {
        pub const DEFAULT_PAWNS_WHITE: u64 = 0b11111111 << 8;
        pub const DEFAULT_PAWNS_BLACK: u64 = 0b11111111 << 48;
        pub const DEFAULT_ROOKS_WHITE: u64 = 0b10000001;
        pub const DEFAULT_ROOKS_BLACK: u64 = 0b10000001 << 56;
        pub const DEFAULT_KNIGHTS_WHITE: u64 = 0b01000010;
        pub const DEFAULT_KNIGHTS_BLACK: u64 = 0b01000010 << 56;
        pub const DEFAULT_BISHOPS_WHITE: u64 = 0b00100100;
        pub const DEFAULT_BISHOPS_BLACK: u64 = 0b00100100 << 56;
        pub const DEFAULT_QUEENS_WHITE: u64 = 0b00001000;
        pub const DEFAULT_QUEENS_BLACK: u64 = 0b00001000 << 56;
        pub const DEFAULT_KING_WHITE: u64 = 0b00010000;
        pub const DEFAULT_KING_BLACK: u64 = 0b00010000 << 56;
    }

    pub mod rank_file {
        pub const RANK_1: u64 = 0b11111111;
        pub const RANK_2: u64 = 0b11111111 << 8;
        pub const RANK_3: u64 = 0b11111111 << 16;
        pub const RANK_4: u64 = 0b11111111 << 24;
        pub const RANK_5: u64 = 0b11111111 << 32;
        pub const RANK_6: u64 = 0b11111111 << 40;
        pub const RANK_7: u64 = 0b11111111 << 48;
        pub const RANK_8: u64 = 0b11111111 << 56;

        pub const FILE_A: u64 =
            1 | 1 << 8 | 1 << 16 | 1 << 24 | 1 << 32 | 1 << 40 | 1 << 48 | 1 << 56;
        pub const FILE_B: u64 =
            1 << 1 | 1 << 9 | 1 << 17 | 1 << 25 | 1 << 33 | 1 << 41 | 1 << 49 | 1 << 57;
        pub const FILE_C: u64 =
            1 << 2 | 1 << 10 | 1 << 18 | 1 << 26 | 1 << 34 | 1 << 42 | 1 << 50 | 1 << 58;
        pub const FILE_D: u64 =
            1 << 3 | 1 << 11 | 1 << 19 | 1 << 27 | 1 << 35 | 1 << 43 | 1 << 51 | 1 << 59;
        pub const FILE_E: u64 =
            1 << 4 | 1 << 12 | 1 << 20 | 1 << 28 | 1 << 36 | 1 << 44 | 1 << 52 | 1 << 60;
        pub const FILE_F: u64 =
            1 << 5 | 1 << 13 | 1 << 21 | 1 << 29 | 1 << 37 | 1 << 45 | 1 << 53 | 1 << 61;
        pub const FILE_G: u64 =
            1 << 6 | 1 << 14 | 1 << 22 | 1 << 30 | 1 << 38 | 1 << 46 | 1 << 54 | 1 << 62;
        pub const FILE_H: u64 =
            1 << 7 | 1 << 15 | 1 << 23 | 1 << 31 | 1 << 39 | 1 << 47 | 1 << 55 | 1 << 63;
    }

    pub mod bitboard_indices {
        pub const WHITE: usize = 0;
        pub const BLACK: usize = 1;

        pub const PAWN: usize = 0;
        pub const KNIGHT: usize = 1;
        pub const BISHOP: usize = 2;
        pub const ROOK: usize = 3;
        pub const QUEEN: usize = 4;
        pub const KING: usize = 5;
    }

    pub mod castle_squares {
        pub const KINGSIDE_WHITE_KING_TARGET_SQUARE: u64 = 0b01000000;
        pub const KINGSIDE_BLACK_KING_TARGET_SQUARE: u64 = 0b01000000 << 56;
        pub const QUEENSIDE_WHITE_KING_TARGET_SQUARE: u64 = 0b00000100;
        pub const QUEENSIDE_BLACK_KING_TARGET_SQUARE: u64 = 0b00000100 << 56;

        pub const KINGSIDE_WHITE_ROOK_TARGET_SQUARE: u64 = 0b00100000;
        pub const KINGSIDE_BLACK_ROOK_TARGET_SQUARE: u64 = 0b00100000 << 56;
        pub const QUEENSIDE_WHITE_ROOK_TARGET_SQUARE: u64 = 0b00001000;
        pub const QUEENSIDE_BLACK_ROOK_TARGET_SQUARE: u64 = 0b00001000 << 56;

        pub const KINGSIDE_WHITE_SQUARES: u64 = 0b01100000;
        pub const KINGSIDE_BLACK_SQUARES: u64 = 0b01100000 << 56;
        pub const QUEENSIDE_WHITE_SQUARES: u64 = 0b00001100;
        pub const QUEENSIDE_BLACK_SQUARES: u64 = 0b00001100 << 56;

        pub const QUEENSIDE_ROOK_SQUARE_WHITE: u64 = 0b00000010;
        pub const QUEENSIDE_ROOK_SQUARE_BLACK: u64 = 0b00000010 << 56;
    }

    pub mod masks {
        pub const NOT_KINGSIDE_WHITE_ROOK_START_SQUARE: u64 = !0b10000000;
        pub const NOT_KINGSIDE_BLACK_ROOK_START_SQUARE: u64 = !(0b10000000 << 56);
        pub const NOT_QUEENSIDE_WHITE_ROOK_START_SQUARE: u64 = !1;
        pub const NOT_QUEENSIDE_BLACK_ROOK_START_SQUARE: u64 = !(1 << 56);
    }
}

use crate::moves::Move;
use crate::{Color, Piece};
use bitboard_constants::{bitboard_indices::*, castle_squares::*, masks::*, starting_positions::*};

// Error variants when constructing a new bitboard
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitBoardCreationError {
    PieceOverlap,
    BadKingCount,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitBoardConversionError {
    BadSquare,
    BadBitboard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitBoardMoveError {
    NoInitialSquarePiece,
    TargetSquareFriendly,
}

pub trait From<BitBoardCreationError> {
    fn from(err: BitBoardCreationError) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BitBoards {
    boards: [[u64; 6]; 2],
}

impl BitBoards {
    /// Returns a collection of bitboards in the default starting position.
    pub fn default() -> Self {
        let mut default_boards = [[0; 6], [0; 6]];

        default_boards[WHITE][PAWN] = DEFAULT_PAWNS_WHITE;
        default_boards[WHITE][KNIGHT] = DEFAULT_KNIGHTS_WHITE;
        default_boards[WHITE][BISHOP] = DEFAULT_BISHOPS_WHITE;
        default_boards[WHITE][ROOK] = DEFAULT_ROOKS_WHITE;
        default_boards[WHITE][QUEEN] = DEFAULT_QUEENS_WHITE;
        default_boards[WHITE][KING] = DEFAULT_KING_WHITE;

        default_boards[BLACK][PAWN] = DEFAULT_PAWNS_BLACK;
        default_boards[BLACK][KNIGHT] = DEFAULT_KNIGHTS_BLACK;
        default_boards[BLACK][BISHOP] = DEFAULT_BISHOPS_BLACK;
        default_boards[BLACK][ROOK] = DEFAULT_ROOKS_BLACK;
        default_boards[BLACK][QUEEN] = DEFAULT_QUEENS_BLACK;
        default_boards[BLACK][KING] = DEFAULT_KING_BLACK;

        Self {
            boards: default_boards,
        }
    }

    pub fn new(boards: [[u64; 6]; 2]) -> Result<Self, BitBoardCreationError> {
        if boards[WHITE][KING].count_ones() != 1 || boards[BLACK][KING].count_ones() != 1 {
            return Err(BitBoardCreationError::BadKingCount);
        }

        let mut all_boards = 0;
        let mut all_pieces = 0;

        for &board in boards.iter().flatten() {
            all_boards |= board;
            all_pieces += board.count_ones();
        }

        if all_boards.count_ones() != all_pieces {
            return Err(BitBoardCreationError::PieceOverlap);
        }

        Ok(Self { boards })
    }

    pub fn all_boards(&self) -> u64 {
        self.boards
            .iter()
            .flatten()
            .fold(0, |acc, &board| acc | board)
    }

    /// Returns the bitboard representing the position of the black and white pawns.
    pub fn pawns(&self) -> u64 {
        self.boards[WHITE][PAWN] | self.boards[BLACK][PAWN]
    }

    /// Returns the bitboard representing the position of the white pawns.
    pub fn pawns_white(&self) -> u64 {
        self.boards[WHITE][PAWN]
    }

    /// Returns the bitboard representing the position of the black pawns.
    pub fn pawns_black(&self) -> u64 {
        self.boards[BLACK][PAWN]
    }

    /// Returns the bitboard representing the position of the black and white rooks.
    pub fn rooks(&self) -> u64 {
        self.boards[WHITE][ROOK] | self.boards[BLACK][ROOK]
    }

    /// Returns the bitboard representing the position of the white rooks.
    pub fn rooks_white(&self) -> u64 {
        self.boards[WHITE][ROOK]
    }

    /// Returns the bitboard representing the position of the black rooks.
    pub fn rooks_black(&self) -> u64 {
        self.boards[BLACK][ROOK]
    }

    /// Returns the bitboard representing the position of the black and white knights.
    pub fn knights(&self) -> u64 {
        self.boards[WHITE][KNIGHT] | self.boards[BLACK][KNIGHT]
    }

    /// Returns the bitboard representing the position of the white knights.
    pub fn knights_white(&self) -> u64 {
        self.boards[WHITE][KNIGHT]
    }

    /// Returns the bitboard representing the position of the black knights.
    pub fn knights_black(&self) -> u64 {
        self.boards[BLACK][KNIGHT]
    }

    /// Returns the bitboard representing the position of the black and white bishops.
    pub fn bishops(&self) -> u64 {
        self.boards[WHITE][BISHOP] | self.boards[BLACK][BISHOP]
    }

    /// Returns the bitboard representing the position of the white bishops.
    pub fn bishops_white(&self) -> u64 {
        self.boards[WHITE][BISHOP]
    }

    /// Returns the bitboard representing the position of the black bishops.
    pub fn bishops_black(&self) -> u64 {
        self.boards[BLACK][BISHOP]
    }

    /// Returns the bitboard representing the position of the black and white queens.
    pub fn queens(&self) -> u64 {
        self.boards[WHITE][QUEEN] | self.boards[BLACK][QUEEN]
    }

    /// Returns the bitboard representing the position of the white queens.
    pub fn queens_white(&self) -> u64 {
        self.boards[WHITE][QUEEN]
    }

    /// Returns the bitboard representing the position of the black queens.
    pub fn queens_black(&self) -> u64 {
        self.boards[BLACK][QUEEN]
    }

    /// Returns the bitboard representing the position of the black and white kings.
    pub fn kings(&self) -> u64 {
        self.boards[WHITE][KING] | self.boards[BLACK][KING]
    }

    /// Returns the bitboard representing the position of the white king.
    pub fn king_white(&self) -> u64 {
        self.boards[WHITE][KING]
    }

    /// Returns the bitboard representing the position of the black king.
    pub fn king_black(&self) -> u64 {
        self.boards[BLACK][KING]
    }

    /// Returns the bitboard representing the position of the white pieces.
    pub fn white(&self) -> u64 {
        self.boards[WHITE].iter().fold(0, |acc, e| acc | *e)
    }

    /// Returns the bitboard representing the position of the black pieces.
    pub fn black(&self) -> u64 {
        self.boards[BLACK].iter().fold(0, |acc, e| acc | *e)
    }

    pub fn total_pieces(&self) -> u32 {
        self.all_boards().count_ones()
    }

    /// "Clears" the square from all bitboards, setting the bit at that
    /// potiion to 0.
    pub fn clear_square(&mut self, square: u8) {
        let clear_mask = !(1 << square);

        for board in self.boards.iter_mut().flatten() {
            *board &= clear_mask;
        }
    }

    pub fn square_to_bitboard(square: u8) -> Result<u64, BitBoardConversionError> {
        if square > 63 {
            return Err(BitBoardConversionError::BadSquare);
        }

        Ok(1 << square)
    }

    pub fn unchecked_square_to_bitboard(square: u8) -> u64 {
        1 << square
    }

    pub fn bitboard_to_square(bitboard: u64) -> Result<u8, BitBoardConversionError> {
        if bitboard.count_ones() != 1 {
            return Err(BitBoardConversionError::BadBitboard);
        }

        Ok(bitboard.trailing_zeros() as u8)
    }

    pub fn unchecked_bitboard_to_square(bitboard: u64) -> u8 {
        bitboard.trailing_zeros() as u8
    }

    pub fn piece_at(&self, square: u8) -> Option<(Color, Piece)> {
        let Ok(bitboard) = BitBoards::square_to_bitboard(square) else {
            return None;
        };

        if self.boards[WHITE][PAWN] & bitboard != 0 {
            return Some((Color::White, Piece::Pawn));
        }

        if self.boards[WHITE][KNIGHT] & bitboard != 0 {
            return Some((Color::White, Piece::Knight));
        }

        if self.boards[WHITE][BISHOP] & bitboard != 0 {
            return Some((Color::White, Piece::Bishop));
        }

        if self.boards[WHITE][ROOK] & bitboard != 0 {
            return Some((Color::White, Piece::Rook));
        }

        if self.boards[WHITE][QUEEN] & bitboard != 0 {
            return Some((Color::White, Piece::Queen));
        }

        if self.boards[WHITE][KING] & bitboard != 0 {
            return Some((Color::White, Piece::King));
        }

        if self.boards[BLACK][PAWN] & bitboard != 0 {
            return Some((Color::Black, Piece::Pawn));
        }

        if self.boards[BLACK][KNIGHT] & bitboard != 0 {
            return Some((Color::Black, Piece::Knight));
        }

        if self.boards[BLACK][BISHOP] & bitboard != 0 {
            return Some((Color::Black, Piece::Bishop));
        }

        if self.boards[BLACK][ROOK] & bitboard != 0 {
            return Some((Color::Black, Piece::Rook));
        }

        if self.boards[BLACK][QUEEN] & bitboard != 0 {
            return Some((Color::Black, Piece::Queen));
        }

        if self.boards[BLACK][KING] & bitboard != 0 {
            return Some((Color::Black, Piece::King));
        }

        None
    }

    /// Updates the bitboards of the piece type and color of the initial square specified in the move,
    /// "moving" it to the target square and replacing any piece present there.
    pub fn move_piece(&mut self, move_attempt: Move) {
        let (initial_square, target_square) = (
            move_attempt.get_initial_square(),
            move_attempt.get_target_square(),
        );

        let Some((initial_color, initial_piece)) = self.piece_at(initial_square) else {
            return;
        };

        let initial_bitboard = BitBoards::unchecked_square_to_bitboard(initial_square);
        let target_bitboard = BitBoards::unchecked_square_to_bitboard(target_square);

        if let Some((target_color, target_piece)) = self.piece_at(target_square) {
            self.boards[target_color.to_index()][target_piece.to_index()] ^= target_bitboard;
        }

        self.boards[initial_color.to_index()][initial_piece.to_index()] ^=
            initial_bitboard | target_bitboard;
    }

    /// Sets the white king and white kingside rook to their castle target squares.
    /// Currently this is completely unchecked, and may result in overlapping
    /// bitboards.
    pub fn castle_kingside_white(&mut self) {
        self.boards[WHITE][KING] = KINGSIDE_WHITE_KING_TARGET_SQUARE;

        self.boards[WHITE][ROOK] &= NOT_KINGSIDE_WHITE_ROOK_START_SQUARE;
        self.boards[WHITE][ROOK] |= KINGSIDE_WHITE_ROOK_TARGET_SQUARE;
    }

    /// Sets the black king and black kingside rook to their castle target squares.
    /// Currently this is completely unchecked, and may result in overlapping
    /// bitboards.
    pub fn castle_kingside_black(&mut self) {
        self.boards[BLACK][KING] = KINGSIDE_BLACK_KING_TARGET_SQUARE;

        self.boards[BLACK][ROOK] &= NOT_KINGSIDE_BLACK_ROOK_START_SQUARE;
        self.boards[BLACK][ROOK] |= KINGSIDE_BLACK_ROOK_TARGET_SQUARE;
    }

    /// Sets the white king and white queenside rook to their castle target squares.
    /// Currently this is completely unchecked, and may result in overlapping
    /// bitboards.
    pub fn castle_queenside_white(&mut self) {
        self.boards[WHITE][KING] = QUEENSIDE_WHITE_KING_TARGET_SQUARE;

        self.boards[WHITE][ROOK] &= NOT_QUEENSIDE_WHITE_ROOK_START_SQUARE;
        self.boards[WHITE][ROOK] |= QUEENSIDE_WHITE_ROOK_TARGET_SQUARE;
    }

    /// Sets the black king and black queenside rook to their castle target squares.
    /// Currently this is completely unchecked, and may result in overlapping
    /// bitboards.
    pub fn castle_queenside_black(&mut self) {
        self.boards[BLACK][KING] = QUEENSIDE_BLACK_KING_TARGET_SQUARE;

        self.boards[BLACK][ROOK] &= NOT_QUEENSIDE_BLACK_ROOK_START_SQUARE;
        self.boards[BLACK][ROOK] |= QUEENSIDE_BLACK_ROOK_TARGET_SQUARE;
    }

    /// "Moves" the white pawn in the initial square to the target square, "capturing"
    /// any black pawn behind it.
    pub fn en_passant_white(&mut self, mv: Move) {
        let (initial_square, target_square) = (mv.get_initial_square(), mv.get_target_square());

        let initial_bitboard = BitBoards::unchecked_square_to_bitboard(initial_square);
        let target_bitboard = BitBoards::unchecked_square_to_bitboard(target_square);

        self.clear_square(target_square - 8);

        self.boards[WHITE][PAWN] ^= initial_bitboard | target_bitboard;
    }

    // TODO: ^v TEST BOTH OF THESE

    /// "Moves" the black pawn in the initial square to the target square, "capturing"
    /// any white pawn behind it.
    pub fn en_passant_black(&mut self, mv: Move) {
        let (initial_square, target_square) = (mv.get_initial_square(), mv.get_target_square());

        let initial_bitboard = BitBoards::unchecked_square_to_bitboard(initial_square);
        let target_bitboard = BitBoards::unchecked_square_to_bitboard(target_square);

        self.clear_square(target_square + 8);

        self.boards[BLACK][PAWN] ^= initial_bitboard | target_bitboard;
    }

    /// Performs a promotion move for white. Removes the pawn from the initial square
    /// and places the piece specified in the move in the target square.
    pub fn promote_white(&mut self, mv: Move) {
        let promote_to = if mv.is_queen_promotion() {
            QUEEN
        } else if mv.is_knight_promotion() {
            KNIGHT
        } else if mv.is_rook_promotion() {
            ROOK
        } else {
            BISHOP
        };

        let (initial_square, target_square) = (mv.get_initial_square(), mv.get_target_square());
        self.clear_square(target_square);

        let initial_bitboard = BitBoards::unchecked_square_to_bitboard(initial_square);
        let target_bitboard = BitBoards::unchecked_square_to_bitboard(target_square);

        self.boards[WHITE][PAWN] ^= initial_bitboard;
        self.boards[WHITE][promote_to] ^= target_bitboard;
    }

    // TODO: ^v TEST BOTH OF THESE

    /// Performs a promotion move for black. Removes the pawn from the initial square
    /// and places the piece specified in the move in the target square.
    pub fn promote_black(&mut self, mv: Move) {
        let promote_to = if mv.is_queen_promotion() {
            QUEEN
        } else if mv.is_knight_promotion() {
            KNIGHT
        } else if mv.is_rook_promotion() {
            ROOK
        } else {
            BISHOP
        };

        let (initial_square, target_square) = (mv.get_initial_square(), mv.get_target_square());
        self.clear_square(target_square);

        let initial_bitboard = BitBoards::unchecked_square_to_bitboard(initial_square);
        let target_bitboard = BitBoards::unchecked_square_to_bitboard(target_square);

        self.boards[BLACK][PAWN] ^= initial_bitboard;
        self.boards[BLACK][promote_to] ^= target_bitboard;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_board_state(board: BitBoards, expected_position: u64) {
        assert_eq!(board.all_boards(), expected_position);
    }

    #[test]
    fn default_board_state() {
        test_board_state(
            BitBoards::default(),
            0b11111111_11111111_00000000_00000000_00000000_00000000_11111111_11111111,
        );
    }

    #[test]
    fn test_bitboard_conversion() {
        assert_eq!(
            BitBoards::square_to_bitboard(69),
            Err(BitBoardConversionError::BadSquare)
        );
        assert_eq!(BitBoards::square_to_bitboard(53), Ok(1 << 53));

        assert_eq!(
            BitBoards::bitboard_to_square(0b11),
            Err(BitBoardConversionError::BadBitboard)
        );
        assert_eq!(BitBoards::bitboard_to_square(1 << 53), Ok(53));
    }

    #[test]
    fn test_piece_at() {
        let board = BitBoards::default();

        assert_eq!(board.piece_at(69), None);
        assert_eq!(board.piece_at(27), None);
        assert_eq!(board.piece_at(0), Some((Color::White, Piece::Rook)));
        assert_eq!(board.piece_at(60), Some((Color::Black, Piece::King)));
    }

    #[test]
    fn test_move_piece() {
        let mut board = BitBoards::default();

        let move_1 = Move::from_squares(15, 31).unwrap();

        board.move_piece(move_1);

        assert_eq!(
            board.all_boards(),
            0b11111111_11111111_00000000_00000000_10000000_00000000_01111111_11111111
        );

        let move_2 = Move::from_squares(7, 31).unwrap();

        board.move_piece(move_2);

        assert_eq!(
            board.all_boards(),
            0b11111111_11111111_00000000_00000000_10000000_00000000_01111111_01111111
        );
    }
}
