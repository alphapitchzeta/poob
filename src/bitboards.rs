// Constants for initializing and interacting with bitboards
pub mod bitboard_constants {
    pub mod starting_positions {
        pub const DEFAULT_PAWNS_WHITE: u64 =
            0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000;
        pub const DEFAULT_PAWNS_BLACK: u64 =
            0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000;
        pub const DEFAULT_ROOKS_WHITE: u64 =
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000001;
        pub const DEFAULT_ROOKS_BLACK: u64 =
            0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        pub const DEFAULT_KNIGHTS_WHITE: u64 =
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000010;
        pub const DEFAULT_KNIGHTS_BLACK: u64 =
            0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        pub const DEFAULT_BISHOPS_WHITE: u64 =
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100100;
        pub const DEFAULT_BISHOPS_BLACK: u64 =
            0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        pub const DEFAULT_QUEENS_WHITE: u64 =
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000;
        pub const DEFAULT_QUEENS_BLACK: u64 =
            0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
        pub const DEFAULT_KING_WHITE: u64 =
            0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000;
        pub const DEFAULT_KING_BLACK: u64 =
            0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000;
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
}

use bitboard_constants::{bitboard_indices::*, starting_positions::*};

use crate::{Color, Piece};

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

pub trait From<BitBoardCreationError> {
    fn from(err: BitBoardCreationError) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BitBoards {
    boards: [[u64; 6]; 2],
}

impl BitBoards {
    /// Returns a collection of bitboards in the default starting position.
    ///
    /// # Panics
    /// Calls `expect` on the `Result` returned by `new`. This should theoretically
    /// never panic, but who knows.
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

        Self::new(default_boards).expect("Invalid board configuration")
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

    pub fn square_to_bitboard(square: u8) -> Result<u64, BitBoardConversionError> {
        if square > 63 {
            return Err(BitBoardConversionError::BadSquare);
        }

        Ok(1 << square)
    }

    pub fn bitboard_to_square(bitboard: u64) -> Result<u8, BitBoardConversionError> {
        if bitboard.count_ones() != 1 {
            return Err(BitBoardConversionError::BadBitboard);
        }

        Ok(bitboard.trailing_zeros() as u8)
    }

    pub fn piece_at(&self, square: u8) -> Option<(Color, Piece)> {
        let Ok(bitboard) = BitBoards::square_to_bitboard(square) else {
            return None;
        };

        if self.boards[WHITE][PAWN] & bitboard == 1 {
            return Some((Color::White, Piece::Pawn));
        }

        if self.boards[WHITE][KNIGHT] & bitboard == 1 {
            return Some((Color::White, Piece::Knight));
        }

        if self.boards[WHITE][BISHOP] & bitboard == 1 {
            return Some((Color::White, Piece::Bishop));
        }

        if self.boards[WHITE][ROOK] & bitboard == 1 {
            return Some((Color::White, Piece::Rook));
        }

        if self.boards[WHITE][QUEEN] & bitboard == 1 {
            return Some((Color::White, Piece::Queen));
        }

        if self.boards[WHITE][KING] & bitboard == 1 {
            return Some((Color::White, Piece::King));
        }

        if self.boards[BLACK][PAWN] & bitboard == 1 {
            return Some((Color::Black, Piece::Pawn));
        }

        if self.boards[BLACK][KNIGHT] & bitboard == 1 {
            return Some((Color::Black, Piece::Knight));
        }

        if self.boards[BLACK][BISHOP] & bitboard == 1 {
            return Some((Color::Black, Piece::Bishop));
        }

        if self.boards[BLACK][ROOK] & bitboard == 1 {
            return Some((Color::Black, Piece::Rook));
        }

        if self.boards[BLACK][QUEEN] & bitboard == 1 {
            return Some((Color::Black, Piece::Queen));
        }

        if self.boards[BLACK][KING] & bitboard == 1 {
            return Some((Color::Black, Piece::King));
        }

        None
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
}
