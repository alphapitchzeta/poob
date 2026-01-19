use crate::bitboards::bitboard_constants::bitboard_indices::*;
use crate::bitboards::{BitBoardCreationError, BitBoards};

mod boardstate_constants {
    pub const PROJECTED_GAME_LENGTH: usize = 40;
    pub const CAN_CASTLE_KINGSIDE_WHITE: u8 = 0b0010;
    pub const CAN_CASTLE_KINGSIDE_BLACK: u8 = 0b1000;
    pub const CAN_CASTLE_QUEENSIDE_WHITE: u8 = 0b0001;
    pub const CAN_CASTLE_QUEENSIDE_BLACK: u8 = 0b0100;
    pub const DEFAULT_CASTLING_RIGHTS: u8 = 0b1111;
}

use boardstate_constants::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

#[derive(Debug)]
pub struct CurrentBoardState {
    pub board_state: BoardState,
    history: BoardHistory,
}

impl CurrentBoardState {
    pub fn get_position(&self) -> &BitBoards {
        &self.board_state.position
    }

    pub fn get_mut_position(&mut self) -> &mut BitBoards {
        &mut self.board_state.position
    }

    pub fn get_turn(&self, index: usize) -> Option<&BoardState> {
        self.history.get(index)
    }
}

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

#[derive(Debug, Clone)]
pub struct BoardState {
    position: BitBoards,
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

        if position.split("/").count() != 8 {
            return Err(BoardStateCreationError::BadFenString(
                FenStringError::BadPosition,
            ));
        }

        let mut ranks = position.split("/");
        let mut unchecked_bitboards = [[0; 6]; 2];
        let mut bit = 1 << 63;

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

                        bit >>= shift;
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

                bit >>= 1;
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

        if en_passant.chars().count() > 2 {
            return Err(BoardStateCreationError::BadFenString(
                FenStringError::BadEnPassant,
            ));
        }

        let mut en_passant_square = 0;

        {
            let mut chars = en_passant.chars();

            let Some(file) = chars.next() else {
                return Err(BoardStateCreationError::BadFenString(
                    FenStringError::BadEnPassant,
                ));
            };

            match file {
                '-' => en_passant_square = 0,
                'a'..='h' => en_passant_square |= (file as u8 - ('a' as u8 - 1)) << 4,
                _ => {
                    return Err(BoardStateCreationError::BadFenString(
                        FenStringError::BadEnPassant,
                    ));
                }
            };

            let rank = chars.next();

            match rank {
                Some('1'..='8') => {
                    let Some(digit) = rank.unwrap_or('1').to_digit(10) else {
                        return Err(BoardStateCreationError::BadFenString(
                            FenStringError::BadEnPassant,
                        ));
                    };

                    en_passant_square |= digit as u8;
                }
                None if en_passant_square == 0 => (),
                _ => {
                    return Err(BoardStateCreationError::BadFenString(
                        FenStringError::BadEnPassant,
                    ));
                }
            }
        }

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

        let en_passant_square = if en_passant_square == 0 {
            None
        } else {
            Some(en_passant_square)
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

    pub fn get_position(&self) -> &BitBoards {
        &self.position
    }

    pub fn from(current: &CurrentBoardState) -> Self {
        current.board_state.clone()
    }

    pub fn can_castle_kingside_white(&self) -> bool {
        self.castling_rights & CAN_CASTLE_KINGSIDE_WHITE != 0
    }

    pub fn can_castle_kingside_black(&self) -> bool {
        self.castling_rights & CAN_CASTLE_KINGSIDE_BLACK != 0
    }

    pub fn can_castle_queenside_white(&self) -> bool {
        self.castling_rights & CAN_CASTLE_QUEENSIDE_WHITE != 0
    }

    pub fn can_castle_queenside_black(&self) -> bool {
        self.castling_rights & CAN_CASTLE_QUEENSIDE_BLACK != 0
    }
}

#[derive(Debug)]
pub struct BoardHistory {
    vec: Vec<BoardState>,
}

impl BoardHistory {
    pub fn new() -> Self {
        Self {
            vec: Vec::with_capacity(PROJECTED_GAME_LENGTH),
        }
    }

    pub fn get(&self, index: usize) -> Option<&BoardState> {
        if index == 0 {
            return None;
        }

        self.vec.get(index - 1)
    }

    // TODO: Consider adding validation logic so it is guaranteed turns
    // are all sequential. This logic may be applied elsewhere instead.
    pub fn push(&mut self, board_state: BoardState) {
        self.vec.push(board_state);
    }

    pub fn pop(&mut self) -> Option<BoardState> {
        self.vec.pop()
    }
}
