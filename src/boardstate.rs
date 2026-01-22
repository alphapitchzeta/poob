use crate::bitboards::bitboard_constants::bitboard_indices::*;
use crate::bitboards::{BitBoardCreationError, BitBoards};
use crate::util::*;
use crate::{Color, Piece};

mod boardstate_constants {
    pub const CAN_CASTLE_KINGSIDE_WHITE: u8 = 0b0010;
    pub const CAN_CASTLE_KINGSIDE_BLACK: u8 = 0b1000;
    pub const CAN_CASTLE_QUEENSIDE_WHITE: u8 = 0b0001;
    pub const CAN_CASTLE_QUEENSIDE_BLACK: u8 = 0b0100;
    pub const DEFAULT_CASTLING_RIGHTS: u8 = 0b1111;
}

use boardstate_constants::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoardStateCreationError {
    BitBoardCreationError,
    BadFenString(FenStringError),
}

impl From<BitBoardCreationError> for BoardStateCreationError {
    fn from(_: BitBoardCreationError) -> Self {
        Self::BitBoardCreationError
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FenStringError {
    MalformedString,
    BadPosition,
    BadColor,
    BadCastling,
    BadEnPassant,
    BadHalfTurnCount,
    BadTurnCount,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BoardState {
    pub position: BitBoards,
    pub side_to_move: Color,
    castling_rights: u8,
    pub en_passant_square: Option<u8>,
    pub fifty_move_rule: u8,
    pub turn_count: u16,
}

impl BoardState {
    pub fn default() -> Self {
        BoardState::new(
            Color::White,
            BitBoards::default(),
            1,
            0,
            DEFAULT_CASTLING_RIGHTS,
            None,
        )
    }

    pub fn new(
        side_to_move: Color,
        position: BitBoards,
        turn_count: u16,
        fifty_move_rule: u8,
        castling_rights: u8,
        en_passant_square: Option<u8>,
    ) -> Self {
        Self {
            side_to_move,
            position,
            turn_count,
            fifty_move_rule,
            castling_rights,
            en_passant_square,
        }
    }

    pub fn from_fen(fen: &str) -> Result<Self, BoardStateCreationError> {
        if fen.split_ascii_whitespace().count() != 6 {
            return Err(BoardStateCreationError::BadFenString(
                FenStringError::MalformedString,
            ));
        }

        let mut chunks = fen.split_ascii_whitespace();

        let Some(position) = chunks.next() else {
            return Err(BoardStateCreationError::BadFenString(
                FenStringError::BadPosition,
            ));
        };

        if position.split('/').count() != 8 {
            return Err(BoardStateCreationError::BadFenString(
                FenStringError::BadPosition,
            ));
        }

        let mut ranks = position.split('/').rev();
        let mut unchecked_bitboards = [[0; 6]; 2];
        let mut bit = 1;

        while let Some(rank) = ranks.next() {
            if rank.chars().count() > 8 {
                return Err(BoardStateCreationError::BadFenString(
                    FenStringError::BadPosition,
                ));
            }

            let mut chars = rank.chars();

            while let Some(c) = chars.next() {
                match c {
                    '1'..='8' => {
                        let shift = match c.to_digit(10) {
                            Some(digit) => digit,
                            None => {
                                return Err(BoardStateCreationError::BadFenString(
                                    FenStringError::BadPosition,
                                ));
                            }
                        };

                        bit <<= shift;
                        continue;
                    }
                    'p' => unchecked_bitboards[BLACK][PAWN] |= bit,
                    'r' => unchecked_bitboards[BLACK][ROOK] |= bit,
                    'n' => unchecked_bitboards[BLACK][KNIGHT] |= bit,
                    'b' => unchecked_bitboards[BLACK][BISHOP] |= bit,
                    'q' => unchecked_bitboards[BLACK][QUEEN] |= bit,
                    'k' => unchecked_bitboards[BLACK][KING] |= bit,
                    'P' => unchecked_bitboards[WHITE][PAWN] |= bit,
                    'R' => unchecked_bitboards[WHITE][ROOK] |= bit,
                    'N' => unchecked_bitboards[WHITE][KNIGHT] |= bit,
                    'B' => unchecked_bitboards[WHITE][BISHOP] |= bit,
                    'Q' => unchecked_bitboards[WHITE][QUEEN] |= bit,
                    'K' => unchecked_bitboards[WHITE][KING] |= bit,
                    _ => {
                        return Err(BoardStateCreationError::BadFenString(
                            FenStringError::BadPosition,
                        ));
                    }
                };

                bit <<= 1;
            }
        }

        let side_to_move = match chunks.next() {
            Some("w") => Color::White,
            Some("b") => Color::Black,
            _ => {
                return Err(BoardStateCreationError::BadFenString(
                    FenStringError::BadColor,
                ));
            }
        };

        let Some(castling) = chunks.next() else {
            return Err(BoardStateCreationError::BadFenString(
                FenStringError::BadCastling,
            ));
        };

        let mut castling_rights = 0;

        if castling != "-" {
            if castling.chars().count() > 4 {
                return Err(BoardStateCreationError::BadFenString(
                    FenStringError::BadCastling,
                ));
            }

            let mut chars = castling.chars();

            while let Some(c) = chars.next() {
                match c {
                    'K' => castling_rights |= CAN_CASTLE_KINGSIDE_WHITE,
                    'Q' => castling_rights |= CAN_CASTLE_QUEENSIDE_WHITE,
                    'k' => castling_rights |= CAN_CASTLE_KINGSIDE_BLACK,
                    'q' => castling_rights |= CAN_CASTLE_QUEENSIDE_BLACK,
                    _ => {
                        return Err(BoardStateCreationError::BadFenString(
                            FenStringError::BadCastling,
                        ));
                    }
                };
            }
        }

        let Some(en_passant) = chunks.next() else {
            return Err(BoardStateCreationError::BadFenString(
                FenStringError::BadEnPassant,
            ));
        };

        let en_passant_square = square_str_to_index(en_passant);

        let Some(half_move_counter) = chunks.next() else {
            return Err(BoardStateCreationError::BadFenString(
                FenStringError::BadHalfTurnCount,
            ));
        };

        let fifty_move_rule = match half_move_counter.parse() {
            Ok(num) => match num {
                0..100 => num,
                _ => {
                    return Err(BoardStateCreationError::BadFenString(
                        FenStringError::BadHalfTurnCount,
                    ));
                }
            },
            _ => {
                return Err(BoardStateCreationError::BadFenString(
                    FenStringError::BadHalfTurnCount,
                ));
            }
        };

        let Some(turn_counter) = chunks.next() else {
            return Err(BoardStateCreationError::BadFenString(
                FenStringError::BadTurnCount,
            ));
        };

        let turn_count = match turn_counter.parse() {
            Ok(num) if num != 0 => num,
            _ => {
                return Err(BoardStateCreationError::BadFenString(
                    FenStringError::BadTurnCount,
                ));
            }
        };

        let position = BitBoards::new(unchecked_bitboards)?;

        Ok(Self::new(
            side_to_move,
            position,
            turn_count,
            fifty_move_rule,
            castling_rights,
            en_passant_square,
        ))
    }

    /// Returns a FEN string of this board state.
    /// # Panics
    /// Currently calls `expect` on `index_to_square_str`. This should not fail,
    /// as the board state representation should always be valid.
    pub fn to_fen(&self) -> String {
        let mut fen: Vec<String> = Vec::with_capacity(6);

        fen.push(self.bitboard_to_fen());

        let side_char = match self.side_to_move {
            Color::White => 'w',
            Color::Black => 'b',
        };

        fen.push(side_char.to_string());

        let castling_rights = if self.castling_rights == 0 {
            '-'.to_string()
        } else {
            let mut castle_str = String::with_capacity(4);

            if self.can_castle_kingside_white() {
                castle_str.push('K');
            }

            if self.can_castle_queenside_white() {
                castle_str.push('Q');
            }

            if self.can_castle_kingside_black() {
                castle_str.push('k');
            }

            if self.can_castle_queenside_black() {
                castle_str.push('q');
            }

            castle_str
        };

        fen.push(castling_rights);

        let en_passant = match self.en_passant_square {
            Some(square) => index_to_square_str(square).expect("Invalid square"),
            None => '-'.to_string(),
        };

        fen.push(en_passant);

        fen.push(self.fifty_move_rule.to_string());
        fen.push(self.turn_count.to_string());

        fen.join(" ")
    }

    /// Returns the piece placement section of a FEN string of this board state.
    pub fn bitboard_to_fen(&self) -> String {
        let mut ranks = vec![String::with_capacity(8); 8];

        for (rank, rank_square) in ranks.iter_mut().rev().zip((0..64).step_by(8)) {
            let mut empty_squares = 0;

            for file in 0..8 {
                let square = rank_square + file;

                if let Some((color, piece)) = self.position.piece_at(square) {
                    if empty_squares > 0 {
                        rank.push_str(&empty_squares.to_string());
                        empty_squares = 0;
                    }

                    let c = match (color, piece) {
                        (Color::White, Piece::Pawn) => 'P',
                        (Color::White, Piece::Rook) => 'R',
                        (Color::White, Piece::Knight) => 'N',
                        (Color::White, Piece::Bishop) => 'B',
                        (Color::White, Piece::Queen) => 'Q',
                        (Color::White, Piece::King) => 'K',
                        (Color::Black, Piece::Pawn) => 'p',
                        (Color::Black, Piece::Rook) => 'r',
                        (Color::Black, Piece::Knight) => 'n',
                        (Color::Black, Piece::Bishop) => 'b',
                        (Color::Black, Piece::Queen) => 'q',
                        (Color::Black, Piece::King) => 'k',
                    };

                    rank.push(c);
                } else {
                    empty_squares += 1;
                }
            }

            if empty_squares > 0 {
                rank.push_str(&empty_squares.to_string());
            }
        }

        ranks.join("/")
    }

    /// Returns an immutable reference to the bitboards of this board state.
    pub fn get_position(&self) -> &BitBoards {
        &self.position
    }

    /// Returns a mutable reference to the bitboards of this board state.
    pub fn get_mut_position(&mut self) -> &mut BitBoards {
        &mut self.position
    }

    /// Returns true if the white kingside castling bitflag is set, and false otherwise.
    pub fn can_castle_kingside_white(&self) -> bool {
        self.castling_rights & CAN_CASTLE_KINGSIDE_WHITE != 0
    }

    /// Returns true if the black kingside castling bitflag is set, and false otherwise.
    pub fn can_castle_kingside_black(&self) -> bool {
        self.castling_rights & CAN_CASTLE_KINGSIDE_BLACK != 0
    }

    /// Returns true if the white queenside castling bitflag is set, and false otherwise.
    pub fn can_castle_queenside_white(&self) -> bool {
        self.castling_rights & CAN_CASTLE_QUEENSIDE_WHITE != 0
    }

    /// Returns true if the black queenside castling bitflag is set, and false otherwise.
    pub fn can_castle_queenside_black(&self) -> bool {
        self.castling_rights & CAN_CASTLE_QUEENSIDE_BLACK != 0
    }
}

#[cfg(test)]
mod tests {
    use crate::bitboards::bitboard_constants::starting_positions::*;

    use super::*;

    #[test]
    fn test_from_fen() {
        let fen = "rnbq1bnr/ppppkppp/8/4p3/4P3/8/PPPPKPPP/RNBQ1BNR w - - 2 5";

        let mut position = [[0; 6]; 2];

        position[WHITE][PAWN] = 0b00010000_00000000_11101111 << 8;
        position[WHITE][ROOK] = DEFAULT_ROOKS_WHITE;
        position[WHITE][KNIGHT] = DEFAULT_KNIGHTS_WHITE;
        position[WHITE][BISHOP] = DEFAULT_BISHOPS_WHITE;
        position[WHITE][QUEEN] = DEFAULT_QUEENS_WHITE;
        position[WHITE][KING] = 0b00010000 << 8;

        position[BLACK][PAWN] = 0b11101111_00000000_00010000 << 32;
        position[BLACK][ROOK] = DEFAULT_ROOKS_BLACK;
        position[BLACK][KNIGHT] = DEFAULT_KNIGHTS_BLACK;
        position[BLACK][BISHOP] = DEFAULT_BISHOPS_BLACK;
        position[BLACK][QUEEN] = DEFAULT_QUEENS_BLACK;
        position[BLACK][KING] = 0b00010000 << 48;

        let board_state = BoardState::new(
            Color::White,
            BitBoards::new(position).unwrap(),
            5,
            2,
            0,
            None,
        );

        assert_eq!(BoardState::from_fen(fen).unwrap(), board_state);
    }

    #[test]
    fn test_bitboard_to_fen() {
        let fen = "rnbq1bnr/ppppkppp/8/4p3/4P3/8/PPPPKPPP/RNBQ1BNR w - - 2 5";

        let board_state = BoardState::from_fen(fen).unwrap();

        assert_eq!(
            board_state.bitboard_to_fen(),
            "rnbq1bnr/ppppkppp/8/4p3/4P3/8/PPPPKPPP/RNBQ1BNR".to_string()
        );
    }

    #[test]
    fn test_to_fen() {
        let fen = "rnbq1bnr/ppppkppp/8/4p3/4P3/8/PPPPKPPP/RNBQ1BNR w - - 2 5";

        let board_state = BoardState::from_fen(fen).unwrap();

        assert_eq!(board_state.to_fen(), fen.to_string());
    }
}
