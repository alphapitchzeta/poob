use crate::bitboard_types::*;

/// Constants for initializing and interacting with bitboards
pub mod bitboard_constants {
    /// Constants of the starting positions for each piece and color.
    use crate::bitboards::BitBoard;

    pub mod starting_positions {
        use super::*;

        pub const DEFAULT_PAWNS_WHITE: BitBoard = BitBoard(0b11111111 << 8);
        pub const DEFAULT_PAWNS_BLACK: BitBoard = BitBoard(0b11111111 << 48);
        pub const DEFAULT_ROOKS_WHITE: BitBoard = BitBoard(0b10000001);
        pub const DEFAULT_ROOKS_BLACK: BitBoard = BitBoard(0b10000001 << 56);
        pub const DEFAULT_KNIGHTS_WHITE: BitBoard = BitBoard(0b01000010);
        pub const DEFAULT_KNIGHTS_BLACK: BitBoard = BitBoard(0b01000010 << 56);
        pub const DEFAULT_BISHOPS_WHITE: BitBoard = BitBoard(0b00100100);
        pub const DEFAULT_BISHOPS_BLACK: BitBoard = BitBoard(0b00100100 << 56);
        pub const DEFAULT_QUEENS_WHITE: BitBoard = BitBoard(0b00001000);
        pub const DEFAULT_QUEENS_BLACK: BitBoard = BitBoard(0b00001000 << 56);
        pub const DEFAULT_KING_WHITE: BitBoard = BitBoard(0b00010000);
        pub const DEFAULT_KING_BLACK: BitBoard = BitBoard(0b00010000 << 56);
    }

    /// Masks of each rank and file.
    pub mod rank_file {
        use super::*;

        pub const RANK_1: BitBoard = BitBoard(0b11111111);
        pub const RANK_2: BitBoard = BitBoard(0b11111111 << 8);
        pub const RANK_3: BitBoard = BitBoard(0b11111111 << 16);
        pub const RANK_4: BitBoard = BitBoard(0b11111111 << 24);
        pub const RANK_5: BitBoard = BitBoard(0b11111111 << 32);
        pub const RANK_6: BitBoard = BitBoard(0b11111111 << 40);
        pub const RANK_7: BitBoard = BitBoard(0b11111111 << 48);
        pub const RANK_8: BitBoard = BitBoard(0b11111111 << 56);

        pub const FILE_A: BitBoard =
            BitBoard(1 | 1 << 8 | 1 << 16 | 1 << 24 | 1 << 32 | 1 << 40 | 1 << 48 | 1 << 56);
        pub const FILE_B: BitBoard =
            BitBoard(1 << 1 | 1 << 9 | 1 << 17 | 1 << 25 | 1 << 33 | 1 << 41 | 1 << 49 | 1 << 57);
        pub const FILE_C: BitBoard =
            BitBoard(1 << 2 | 1 << 10 | 1 << 18 | 1 << 26 | 1 << 34 | 1 << 42 | 1 << 50 | 1 << 58);
        pub const FILE_D: BitBoard =
            BitBoard(1 << 3 | 1 << 11 | 1 << 19 | 1 << 27 | 1 << 35 | 1 << 43 | 1 << 51 | 1 << 59);
        pub const FILE_E: BitBoard =
            BitBoard(1 << 4 | 1 << 12 | 1 << 20 | 1 << 28 | 1 << 36 | 1 << 44 | 1 << 52 | 1 << 60);
        pub const FILE_F: BitBoard =
            BitBoard(1 << 5 | 1 << 13 | 1 << 21 | 1 << 29 | 1 << 37 | 1 << 45 | 1 << 53 | 1 << 61);
        pub const FILE_G: BitBoard =
            BitBoard(1 << 6 | 1 << 14 | 1 << 22 | 1 << 30 | 1 << 38 | 1 << 46 | 1 << 54 | 1 << 62);
        pub const FILE_H: BitBoard =
            BitBoard(1 << 7 | 1 << 15 | 1 << 23 | 1 << 31 | 1 << 39 | 1 << 47 | 1 << 55 | 1 << 63);
    }

    /// Constants mapping each color and piece to a corresponding index in a `[[u64; 6]; 2]`.
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

    /// Masks of squares relevant for castling.
    pub mod castle_squares {
        use super::*;

        pub const KINGSIDE_WHITE_KING_TARGET_SQUARE: BitBoard = BitBoard(0b01000000);
        pub const KINGSIDE_BLACK_KING_TARGET_SQUARE: BitBoard = BitBoard(0b01000000 << 56);
        pub const QUEENSIDE_WHITE_KING_TARGET_SQUARE: BitBoard = BitBoard(0b00000100);
        pub const QUEENSIDE_BLACK_KING_TARGET_SQUARE: BitBoard = BitBoard(0b00000100 << 56);

        pub const KINGSIDE_WHITE_ROOK_TARGET_SQUARE: BitBoard = BitBoard(0b00100000);
        pub const KINGSIDE_BLACK_ROOK_TARGET_SQUARE: BitBoard = BitBoard(0b00100000 << 56);
        pub const QUEENSIDE_WHITE_ROOK_TARGET_SQUARE: BitBoard = BitBoard(0b00001000);
        pub const QUEENSIDE_BLACK_ROOK_TARGET_SQUARE: BitBoard = BitBoard(0b00001000 << 56);

        pub const KINGSIDE_WHITE_SQUARES: BitBoard = BitBoard(0b01100000);
        pub const KINGSIDE_BLACK_SQUARES: BitBoard = BitBoard(0b01100000 << 56);
        pub const QUEENSIDE_WHITE_SQUARES: BitBoard = BitBoard(0b00001100);
        pub const QUEENSIDE_BLACK_SQUARES: BitBoard = BitBoard(0b00001100 << 56);

        pub const QUEENSIDE_ROOK_SQUARE_WHITE: BitBoard = BitBoard(0b00000010);
        pub const QUEENSIDE_ROOK_SQUARE_BLACK: BitBoard = BitBoard(0b00000010 << 56);
    }

    /// Various useful masks.
    pub mod masks {
        use super::*;

        pub const NOT_KINGSIDE_WHITE_ROOK_START_SQUARE: BitBoard = BitBoard(!0b10000000);
        pub const NOT_KINGSIDE_BLACK_ROOK_START_SQUARE: BitBoard = BitBoard(!0b10000000 << 56);
        pub const NOT_QUEENSIDE_WHITE_ROOK_START_SQUARE: BitBoard = BitBoard(!1);
        pub const NOT_QUEENSIDE_BLACK_ROOK_START_SQUARE: BitBoard = BitBoard(!(1 << 56));
    }
}

use crate::moves::Move;
use crate::{Color, Piece};
use bitboard_constants::{bitboard_indices::*, castle_squares::*, masks::*, starting_positions::*};

/// Error variants when constructing a new bitboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitBoardCreationError {
    PieceOverlap,
    BadKingCount,
}

/// Error variants when converting a square to a
/// bitboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitBoardConversionError {
    BadSquare,
    BadBitboard,
}

/// Error variants when making a [`Move`] is invalid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitBoardMoveError {
    NoInitialSquarePiece,
    TargetSquareFriendly,
}

pub trait From<BitBoardCreationError> {
    fn from(err: BitBoardCreationError) -> Self;
}

/// Struct containing bitboards for every [`Piece`] of
/// every [`Color`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BitBoards {
    boards: [[BitBoard; 6]; 2],
}

impl BitBoards {
    /// Returns a collection of bitboards in the default starting position.
    pub fn default() -> Self {
        let mut default_boards = [[BitBoard(0); 6], [BitBoard(0); 6]];

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

    pub fn new(boards: [[BitBoard; 6]; 2]) -> Result<Self, BitBoardCreationError> {
        if boards[WHITE][KING].popcount() != 1 || boards[BLACK][KING].popcount() != 1 {
            return Err(BitBoardCreationError::BadKingCount);
        }

        let mut all_boards = BitBoard(0);
        let mut all_pieces = 0;

        for &board in boards.iter().flatten() {
            all_boards |= board;
            all_pieces += board.popcount();
        }

        if all_boards.popcount() != all_pieces {
            return Err(BitBoardCreationError::PieceOverlap);
        }

        Ok(Self { boards })
    }

    /// Returns the bitboard representing all pieces in the current position.
    pub fn all_boards(&self) -> BitBoard {
        self.boards
            .iter()
            .flatten()
            .fold(BitBoard(0), |acc, &board| acc | board)
    }

    /// Returns the bitboard representing the position of the [black](Color::Black) and [white](Color::White) [pawns](Piece::Pawn).
    pub fn pawns(&self) -> BitBoard {
        self.boards[WHITE][PAWN] | self.boards[BLACK][PAWN]
    }

    /// Returns the bitboard representing the position of the [white](Color::White) [pawns](Piece::Pawn).
    pub fn pawns_white(&self) -> BitBoard {
        self.boards[WHITE][PAWN]
    }

    /// Returns the bitboard representing the position of the [black](Color::Black) [pawns](Piece::Pawn).
    pub fn pawns_black(&self) -> BitBoard {
        self.boards[BLACK][PAWN]
    }

    /// Returns the bitboard representing the position of the [black](Color::Black) and [white](Color::White) [rooks](Piece::Rook).
    pub fn rooks(&self) -> BitBoard {
        self.boards[WHITE][ROOK] | self.boards[BLACK][ROOK]
    }

    /// Returns the bitboard representing the position of the [white](Color::White) [rooks](Piece::Rook).
    pub fn rooks_white(&self) -> BitBoard {
        self.boards[WHITE][ROOK]
    }

    /// Returns the bitboard representing the position of the [black](Color::Black) [rooks](Piece::Rook).
    pub fn rooks_black(&self) -> BitBoard {
        self.boards[BLACK][ROOK]
    }

    /// Returns the bitboard representing the position of the [black](Color::Black) and [white](Color::White) [knights](Piece::Knight).
    pub fn knights(&self) -> BitBoard {
        self.boards[WHITE][KNIGHT] | self.boards[BLACK][KNIGHT]
    }

    /// Returns the bitboard representing the position of the [white](Color::White) [knights](Piece::Knight).
    pub fn knights_white(&self) -> BitBoard {
        self.boards[WHITE][KNIGHT]
    }

    /// Returns the bitboard representing the position of the [black](Color::Black) [knights](Piece::Knight).
    pub fn knights_black(&self) -> BitBoard {
        self.boards[BLACK][KNIGHT]
    }

    /// Returns the bitboard representing the position of the [black](Color::Black) and [white](Color::White) [bishops](Piece::Bishop).
    pub fn bishops(&self) -> BitBoard {
        self.boards[WHITE][BISHOP] | self.boards[BLACK][BISHOP]
    }

    /// Returns the bitboard representing the position of the [white](Color::White) [bishops](Piece::Bishop).
    pub fn bishops_white(&self) -> BitBoard {
        self.boards[WHITE][BISHOP]
    }

    /// Returns the bitboard representing the position of the [black](Color::Black) [bishops](Piece::Bishop).
    pub fn bishops_black(&self) -> BitBoard {
        self.boards[BLACK][BISHOP]
    }

    /// Returns the bitboard representing the position of the [black](Color::Black) and [white](Color::White) [queens](Piece::Queen).
    pub fn queens(&self) -> BitBoard {
        self.boards[WHITE][QUEEN] | self.boards[BLACK][QUEEN]
    }

    /// Returns the bitboard representing the position of the [white](Color::White) [queens](Piece::Queen).
    pub fn queens_white(&self) -> BitBoard {
        self.boards[WHITE][QUEEN]
    }

    /// Returns the bitboard representing the position of the [black](Color::Black) [queens](Piece::Queen).
    pub fn queens_black(&self) -> BitBoard {
        self.boards[BLACK][QUEEN]
    }

    /// Returns the bitboard representing the position of the [black](Color::Black) and [white](Color::White) kings.
    pub fn kings(&self) -> BitBoard {
        self.boards[WHITE][KING] | self.boards[BLACK][KING]
    }

    /// Returns the bitboard representing the position of the [white](Color::White) king.
    pub fn king_white(&self) -> BitBoard {
        self.boards[WHITE][KING]
    }

    /// Returns the bitboard representing the position of the [black](Color::Black) king.
    pub fn king_black(&self) -> BitBoard {
        self.boards[BLACK][KING]
    }

    /// Returns the bitboard representing the position of the [white](Color::White) pieces.
    pub fn white(&self) -> BitBoard {
        self.boards[WHITE]
            .iter()
            .fold(BitBoard(0), |acc, e| acc | *e)
    }

    /// Returns the bitboard representing the position of the [black](Color::Black) pieces.
    pub fn black(&self) -> BitBoard {
        self.boards[BLACK]
            .iter()
            .fold(BitBoard(0), |acc, e| acc | *e)
    }

    /// Returns the total piece count.
    pub fn total_pieces(&self) -> u8 {
        self.all_boards().popcount()
    }

    /// "Clears" the square from all bitboards, setting the bit at that
    /// position to `0`.
    pub fn clear_square(&mut self, square: Square) {
        for board in self.boards.iter_mut().flatten() {
            board.clear(square);
        }
    }

    /// Returns `Some((Color, Piece))` of the piece on a given square, or [`None`]
    /// if that square is unoccupied.
    pub fn piece_at(&self, square: Square) -> Option<(Color, Piece)> {
        let bitboard = square.to_bitboard();

        if !((self.boards[WHITE][PAWN] & bitboard).is_empty()) {
            return Some((Color::White, Piece::Pawn));
        }

        if !(self.boards[WHITE][KNIGHT] & bitboard).is_empty() {
            return Some((Color::White, Piece::Knight));
        }

        if !((self.boards[WHITE][BISHOP] & bitboard).is_empty()) {
            return Some((Color::White, Piece::Bishop));
        }

        if !((self.boards[WHITE][ROOK] & bitboard).is_empty()) {
            return Some((Color::White, Piece::Rook));
        }

        if !((self.boards[WHITE][QUEEN] & bitboard).is_empty()) {
            return Some((Color::White, Piece::Queen));
        }

        if !((self.boards[WHITE][KING] & bitboard).is_empty()) {
            return Some((Color::White, Piece::King));
        }

        if !((self.boards[BLACK][PAWN] & bitboard).is_empty()) {
            return Some((Color::Black, Piece::Pawn));
        }

        if !((self.boards[BLACK][KNIGHT] & bitboard).is_empty()) {
            return Some((Color::Black, Piece::Knight));
        }

        if !((self.boards[BLACK][BISHOP] & bitboard).is_empty()) {
            return Some((Color::Black, Piece::Bishop));
        }

        if !((self.boards[BLACK][ROOK] & bitboard).is_empty()) {
            return Some((Color::Black, Piece::Rook));
        }

        if !((self.boards[BLACK][QUEEN] & bitboard).is_empty()) {
            return Some((Color::Black, Piece::Queen));
        }

        if !((self.boards[BLACK][KING] & bitboard).is_empty()) {
            return Some((Color::Black, Piece::King));
        }

        None
    }

    /// Updates the bitboards of the piece type and color of the initial square specified in the move,
    /// "moving" it to the target square and replacing any piece present there.
    pub fn move_piece(&mut self, move_attempt: Move) {
        let (initial_square, target_square) =
            (move_attempt.initial_square(), move_attempt.target_square());

        let Some((initial_color, initial_piece)) = self.piece_at(initial_square) else {
            return;
        };

        let initial_bitboard = initial_square.to_bitboard();
        let target_bitboard = target_square.to_bitboard();

        if let Some((target_color, target_piece)) = self.piece_at(target_square) {
            self.boards[target_color.to_index()][target_piece.to_index()] ^= target_bitboard;
        }

        self.boards[initial_color.to_index()][initial_piece.to_index()] ^=
            initial_bitboard | target_bitboard;
    }

    /// Sets the [white](Color::White) [king](Piece::King) and [white](Color::White) kingside [rook](Piece::Rook) to their castle target squares.
    /// Currently this is completely unchecked, and may result in overlapping
    /// bitboards.
    pub fn castle_kingside_white(&mut self) {
        self.boards[WHITE][KING] = KINGSIDE_WHITE_KING_TARGET_SQUARE;

        self.boards[WHITE][ROOK] &= NOT_KINGSIDE_WHITE_ROOK_START_SQUARE;
        self.boards[WHITE][ROOK] |= KINGSIDE_WHITE_ROOK_TARGET_SQUARE;
    }

    /// Sets the [black](Color::Black) [king](Piece::King) and [black](Color::Black) kingside [rook](Piece::Rook) to their castle target squares.
    /// Currently this is completely unchecked, and may result in overlapping
    /// bitboards.
    pub fn castle_kingside_black(&mut self) {
        self.boards[BLACK][KING] = KINGSIDE_BLACK_KING_TARGET_SQUARE;

        self.boards[BLACK][ROOK] &= NOT_KINGSIDE_BLACK_ROOK_START_SQUARE;
        self.boards[BLACK][ROOK] |= KINGSIDE_BLACK_ROOK_TARGET_SQUARE;
    }

    /// Sets the [white](Color::White) [king](Piece::King) and [white](Color::White) queenside [rook](Piece::Rook) to their castle target squares.
    /// Currently this is completely unchecked, and may result in overlapping
    /// bitboards.
    pub fn castle_queenside_white(&mut self) {
        self.boards[WHITE][KING] = QUEENSIDE_WHITE_KING_TARGET_SQUARE;

        self.boards[WHITE][ROOK] &= NOT_QUEENSIDE_WHITE_ROOK_START_SQUARE;
        self.boards[WHITE][ROOK] |= QUEENSIDE_WHITE_ROOK_TARGET_SQUARE;
    }

    /// Sets the [black](Color::Black) [king](Piece::King) and [black](Color::Black) queenside [rook](Piece::Rook) to their castle target squares.
    /// Currently this is completely unchecked, and may result in overlapping
    /// bitboards.
    pub fn castle_queenside_black(&mut self) {
        self.boards[BLACK][KING] = QUEENSIDE_BLACK_KING_TARGET_SQUARE;

        self.boards[BLACK][ROOK] &= NOT_QUEENSIDE_BLACK_ROOK_START_SQUARE;
        self.boards[BLACK][ROOK] |= QUEENSIDE_BLACK_ROOK_TARGET_SQUARE;
    }

    /// "Moves" the [white](Color::White) [pawn](Piece::Pawn) in the initial square to the target square, "capturing"
    /// any [black](Color::Black) [pawn](Piece::Pawn) behind it.
    pub fn en_passant_white(&mut self, mv: Move) {
        let (initial_square, target_square) = (mv.initial_square(), mv.target_square());

        let initial_bitboard = initial_square.to_bitboard();
        let target_bitboard = target_square.to_bitboard();

        self.clear_square(target_square - Square::new(8));

        self.boards[WHITE][PAWN] ^= initial_bitboard | target_bitboard;
    }

    // TODO: ^v TEST BOTH OF THESE

    /// "Moves" the [black](Color::Black) [pawn](Piece::Pawn) in the initial square to the target square, "capturing"
    /// any [white](Color::White) [pawn](Piece::Pawn) behind it.
    pub fn en_passant_black(&mut self, mv: Move) {
        let (initial_square, target_square) = (mv.initial_square(), mv.target_square());

        let initial_bitboard = initial_square.to_bitboard();
        let target_bitboard = target_square.to_bitboard();

        self.clear_square(target_square + Square::new(8));

        self.boards[BLACK][PAWN] ^= initial_bitboard | target_bitboard;
    }

    /// Performs a promotion move for [white](Color::White). Removes the [pawn](Piece::Pawn) from the initial square
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

        let (initial_square, target_square) = (mv.initial_square(), mv.target_square());
        self.clear_square(target_square);

        let initial_bitboard = initial_square.to_bitboard();
        let target_bitboard = target_square.to_bitboard();

        self.boards[WHITE][PAWN] ^= initial_bitboard;
        self.boards[WHITE][promote_to] ^= target_bitboard;
    }

    // TODO: ^v TEST BOTH OF THESE

    /// Performs a promotion move for [black](Color::Black). Removes the [pawn](Piece::Pawn) from the initial square
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

        let (initial_square, target_square) = (mv.initial_square(), mv.target_square());
        self.clear_square(target_square);

        let initial_bitboard = initial_square.to_bitboard();
        let target_bitboard = target_square.to_bitboard();

        self.boards[BLACK][PAWN] ^= initial_bitboard;
        self.boards[BLACK][promote_to] ^= target_bitboard;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_board_state(board: BitBoards, expected_position: BitBoard) {
        assert_eq!(board.all_boards(), expected_position);
    }

    #[test]
    fn default_board_state() {
        test_board_state(
            BitBoards::default(),
            BitBoard(0b11111111_11111111_00000000_00000000_00000000_00000000_11111111_11111111),
        );
    }

    #[test]
    fn test_piece_at() {
        let board = BitBoards::default();

        //assert_eq!(board.piece_at(Square::new(69)), None);
        assert_eq!(board.piece_at(Square::new(27)), None);
        assert_eq!(
            board.piece_at(Square::A1),
            Some((Color::White, Piece::Rook))
        );
        assert_eq!(
            board.piece_at(Square::new(60)),
            Some((Color::Black, Piece::King))
        );
    }

    #[test]
    fn test_move_piece() {
        let mut board = BitBoards::default();

        let move_1 = Move::unchecked_from_squares(Square::new(15), Square::new(31));

        board.move_piece(move_1);

        assert_eq!(
            board.all_boards(),
            BitBoard(0b11111111_11111111_00000000_00000000_10000000_00000000_01111111_11111111)
        );

        let move_2 = Move::unchecked_from_squares(Square::new(7), Square::new(31));

        board.move_piece(move_2);

        assert_eq!(
            board.all_boards(),
            BitBoard(0b11111111_11111111_00000000_00000000_10000000_00000000_01111111_01111111)
        );
    }
}
