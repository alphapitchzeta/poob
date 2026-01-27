use crate::bitboards::{bitboard_constants::castle_squares::*, *};
use crate::boardstate::*;
use crate::movegen::*;
use crate::rende::*;
use crate::util::*;
use crate::{Color, Piece};

const PROJECTED_GAME_LENGTH: usize = 40;

#[derive(Debug)]
pub struct Game<'a> {
    board_state: BoardState,
    outcome: Option<Outcome>,
    move_gen: &'a MoveGenerator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
enum Outcome {
    Win(Color),
    Draw,
}

impl<'a> Game<'a> {
    pub fn new(move_gen: &'a MoveGenerator) -> Self {
        Self {
            board_state: BoardState::default(),
            move_gen,
            outcome: None,
        }
    }

    pub fn from_fen(
        fen: &str,
        move_gen: &'a MoveGenerator,
    ) -> Result<Self, BoardStateCreationError> {
        Ok(Self {
            board_state: BoardState::from_fen(fen)?,
            move_gen,
            outcome: None,
        })
    }

    pub fn play_sandbox(&mut self) {
        let mut history = BoardHistory::new();

        while self.outcome == None {
            self.print();

            let Some(mv) = read_move() else {
                continue;
            };

            history.push(self.board_state.clone());

            self.get_mut_position().move_piece(mv);

            self.board_state.turn_count += 1;
        }

        let Some(outcome) = self.outcome else {
            return;
        };

        match outcome {
            Outcome::Win(Color::White) => println!("White wins!"),
            Outcome::Win(Color::Black) => println!("Black wins!"),
            Outcome::Draw => println!("Draw!"),
        }
    }

    pub fn print(&self) {
        print_bitboard(self.get_position(), self.board_state.side_to_move);
        println!("Turn count: {}", self.board_state.turn_count);
    }

    pub fn get_position(&self) -> &BitBoards {
        &self.board_state.position
    }

    pub fn get_mut_position(&mut self) -> &mut BitBoards {
        &mut self.board_state.position
    }
}

impl Game<'_> {
    /// Returns a `u64` bitboard of all squares being attacked by pieces of a given color.
    /// This includes squares currently occupied by other friendly pieces.
    pub fn get_attacks(&self, checked_color: Color) -> u64 {
        let mut attacks = 0;
        let open_squares = !self.board_state.position.all_boards();

        for square in 0..64 {
            let piece = match self.board_state.position.piece_at(square) {
                Some((color, piece)) if color == checked_color => piece,
                _ => continue,
            };

            attacks |= match (checked_color, piece) {
                (Color::Black, Piece::Pawn) => self.move_gen.get_black_pawn_attacks(square),
                (Color::White, Piece::Pawn) => self.move_gen.get_white_pawn_attacks(square),
                (_, Piece::Knight) => self.move_gen.get_knight_attacks(square),
                (_, Piece::King) => self.move_gen.get_king_attacks(square),
                (_, Piece::Rook) => MoveGenerator::get_rook_attacks(square, open_squares),
                (_, Piece::Bishop) => MoveGenerator::get_bishop_attacks(square, open_squares),
                (_, Piece::Queen) => MoveGenerator::get_queen_attacks(square, open_squares),
            };
        }

        attacks
    }

    /// Returns `true` if the white king is in check, and `false` otherwise.
    pub fn is_in_check_white(&self, enemy_attacks: u64) -> bool {
        self.board_state.position.king_white() & enemy_attacks != 0
    }

    /// Returns `true` if the black king is in check, and `false` otherwise.
    pub fn is_in_check_black(&self, enemy_attacks: u64) -> bool {
        self.board_state.position.king_black() & enemy_attacks != 0
    }

    /// Returns `true` if white can castle kingside, and `false` otherwise.
    pub fn can_castle_kingside_white(&self, bitboard: u64, enemy_attacks: u64) -> bool {
        if !self.board_state.has_castling_rights_kingside_white() {
            return false;
        }

        if self.is_in_check_white(enemy_attacks) {
            return false;
        }

        if bitboard & KINGSIDE_WHITE_SQUARES != 0 {
            return false;
        }

        if enemy_attacks & KINGSIDE_WHITE_SQUARES != 0 {
            return false;
        }

        true
    }

    /// Returns `true` if black can castle kingside, and `false` otherwise.
    pub fn can_castle_kingside_black(&self, bitboard: u64, enemy_attacks: u64) -> bool {
        if !self.board_state.has_castling_rights_kingside_black() {
            return false;
        }

        if self.is_in_check_black(enemy_attacks) {
            return false;
        }

        if bitboard & KINGSIDE_BLACK_SQUARES != 0 {
            return false;
        }

        if enemy_attacks & KINGSIDE_BLACK_SQUARES != 0 {
            return false;
        }

        true
    }

    /// Returns `true` if white can castle queenside, and `false` otherwise.
    pub fn can_castle_queenside_white(&self, bitboard: u64, enemy_attacks: u64) -> bool {
        if !self.board_state.has_castling_rights_queenside_white() {
            return false;
        }

        if self.is_in_check_white(enemy_attacks) {
            return false;
        }

        if bitboard & (QUEENSIDE_WHITE_SQUARES | QUEENSIDE_ROOK_SQUARE_WHITE) != 0 {
            return false;
        }

        if enemy_attacks & QUEENSIDE_WHITE_SQUARES != 0 {
            return false;
        }

        true
    }

    /// Returns `true` if black can castle queenside, and `false` otherwise.
    pub fn can_castle_queenside_black(&self, bitboard: u64, enemy_attacks: u64) -> bool {
        if !self.board_state.has_castling_rights_queenside_black() {
            return false;
        }

        if self.is_in_check_black(enemy_attacks) {
            return false;
        }

        if bitboard & (QUEENSIDE_BLACK_SQUARES | QUEENSIDE_ROOK_SQUARE_BLACK) != 0 {
            return false;
        }

        if enemy_attacks & QUEENSIDE_BLACK_SQUARES != 0 {
            return false;
        }

        true
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
        self.vec.get(index)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_enemy_attacks() {
        let move_gen = MoveGenerator::new();
        let game = Game::new(&move_gen);
        let enemy_attacks = game.get_attacks(game.board_state.side_to_move.enemy());

        assert_eq!(enemy_attacks.count_ones(), 22);
    }

    // TODO: Add more test cases
    #[test]
    fn test_is_in_check() {
        let move_gen = MoveGenerator::new();

        {
            let game = Game::new(&move_gen);
            let enemy_attacks = game.get_attacks(game.board_state.side_to_move.enemy());

            assert_eq!(game.is_in_check_white(enemy_attacks), false);
        }

        {
            let fen = "rnbqkbnr/pppp1ppp/8/8/8/8/PPPPQPPP/RNB1KBNR b KQkq - 0 1";
            let game = Game::from_fen(fen, &move_gen).expect("Invalid FEN");
            let enemy_attacks = game.get_attacks(game.board_state.side_to_move.enemy());

            assert_eq!(game.is_in_check_black(enemy_attacks), true);
        }
    }

    // TODO: Add more test cases
    #[test]
    fn test_can_castle() {
        let move_gen = MoveGenerator::new();

        {
            let fen = "rnbq1bnr/pppp1ppp/4k3/8/8/4K3/PPPP1PPP/RNBQ1BNR w - - 0 1";
            let game = Game::from_fen(fen, &move_gen).expect("Invalid FEN");

            let bitboard = game.board_state.position.all_boards();
            let enemy_attacks = game.get_attacks(game.board_state.side_to_move.enemy());

            assert_eq!(
                game.can_castle_kingside_white(bitboard, enemy_attacks),
                false
            );
            assert_eq!(
                game.can_castle_kingside_black(bitboard, enemy_attacks),
                false
            );
            assert_eq!(
                game.can_castle_queenside_white(bitboard, enemy_attacks),
                false
            );
            assert_eq!(
                game.can_castle_queenside_black(bitboard, enemy_attacks),
                false
            );
        }

        {
            let game = Game::new(&move_gen);

            let bitboard = game.board_state.position.all_boards();
            let enemy_attacks = game.get_attacks(game.board_state.side_to_move.enemy());

            assert_eq!(
                game.can_castle_kingside_white(bitboard, enemy_attacks),
                false
            );
            assert_eq!(
                game.can_castle_kingside_black(bitboard, enemy_attacks),
                false
            );
            assert_eq!(
                game.can_castle_queenside_white(bitboard, enemy_attacks),
                false
            );
            assert_eq!(
                game.can_castle_queenside_black(bitboard, enemy_attacks),
                false
            );
        }

        {
            let fen = "r3k2r/pppq1ppp/2npbn2/1B2p3/1b2P3/2NPBN2/PPPQ1PPP/R3K2R w KQkq - 0 1";
            let game = Game::from_fen(fen, &move_gen).expect("Invalid FEN");

            let bitboard = game.board_state.position.all_boards();
            let enemy_attacks = game.get_attacks(game.board_state.side_to_move.enemy());
            let friendly_attacks = game.get_attacks(game.board_state.side_to_move);

            assert_eq!(
                game.can_castle_kingside_white(bitboard, enemy_attacks),
                true
            );
            assert_eq!(
                game.can_castle_kingside_black(bitboard, friendly_attacks),
                true
            );
            assert_eq!(
                game.can_castle_queenside_white(bitboard, enemy_attacks),
                true
            );
            assert_eq!(
                game.can_castle_queenside_black(bitboard, friendly_attacks),
                true
            );
        }
    }
}
