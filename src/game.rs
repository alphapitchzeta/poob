use crate::bitboard_types::*;
use crate::bitboards::{
    bitboard_constants::{castle_squares::*, rank_file::*},
    *,
};
use crate::boardstate::*;
use crate::movegen::*;
use crate::moves::*;
use crate::rende::*;
use crate::{Color, Piece};

const PROJECTED_GAME_LENGTH: usize = 40;

/// Struct encapsulating the game logic.
#[derive(Debug, Clone)]
pub struct Game<'a> {
    board_state: BoardState,
    move_gen: &'a MoveGenerator,
}

/// Represents the possible outcomes of a [`Game`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Outcome {
    Win(Color),
    Draw,
}

impl<'a> Game<'a> {
    pub fn new(move_gen: &'a MoveGenerator) -> Self {
        Self {
            board_state: BoardState::default(),
            move_gen,
        }
    }

    pub fn from_fen(
        fen: &str,
        move_gen: &'a MoveGenerator,
    ) -> Result<Self, BoardStateCreationError> {
        Ok(Self {
            board_state: BoardState::from_fen(fen)?,
            move_gen,
        })
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
    pub fn to_fen(&self) -> String {
        self.board_state.to_fen()
    }

    /// Returns a [`BitBoard`] of all squares being attacked by pieces of a given [color](crate::Color).
    /// This includes squares currently occupied by other friendly pieces.
    pub fn get_attacks(&self, checked_color: Color) -> BitBoard {
        let mut attacks = BitBoard(0);

        let color_board = match checked_color {
            Color::White => self.board_state.position.white(),
            Color::Black => self.board_state.position.black(),
        };

        let open_squares = !self.board_state.position.all_boards();

        for square in color_board {
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

    /// Returns `true` if the [white](crate::Color::White) [king](crate::Piece::King) is in check, and `false` otherwise.
    pub fn is_in_check_white(&self, enemy_attacks: BitBoard) -> bool {
        !((self.board_state.position.king_white() & enemy_attacks).is_empty())
    }

    /// Returns `true` if the [black](crate::Color::Black) [king](crate::Piece::King) is in check, and `false` otherwise.
    pub fn is_in_check_black(&self, enemy_attacks: BitBoard) -> bool {
        !((self.board_state.position.king_black() & enemy_attacks).is_empty())
    }

    /// Returns `true` if the provided move would put the [white](crate::Color::White) [king](crate::Piece::King) in check,
    /// and `false` otherwise.
    pub fn would_check_white(&self, mv: Move) -> bool {
        let mut next_turn = self.clone();

        next_turn.board_state.make_move(mv);

        let enemy_attacks = next_turn.get_attacks(Color::Black);

        next_turn.is_in_check_white(enemy_attacks)
    }

    /// Returns `true` if the provided move would put the [black](crate::Color::Black) [king](crate::Piece::King) in check,
    /// and `false` otherwise.
    pub fn would_check_black(&self, mv: Move) -> bool {
        let mut next_turn = self.clone();

        next_turn.board_state.make_move(mv);

        let enemy_attacks = next_turn.get_attacks(Color::White);

        next_turn.is_in_check_black(enemy_attacks)
    }

    /// Returns `true` if [white](crate::Color::White) can castle kingside, and `false` otherwise.
    pub fn can_castle_kingside_white(&self, bitboard: BitBoard, enemy_attacks: BitBoard) -> bool {
        if !self.board_state.has_castling_rights_kingside_white() {
            return false;
        }

        if self.is_in_check_white(enemy_attacks) {
            return false;
        }

        if !((bitboard & KINGSIDE_WHITE_SQUARES).is_empty()) {
            return false;
        }

        if !((enemy_attacks & KINGSIDE_WHITE_SQUARES).is_empty()) {
            return false;
        }

        true
    }

    /// Returns `true` if [black](crate::Color::Black) can castle kingside, and `false` otherwise.
    pub fn can_castle_kingside_black(&self, bitboard: BitBoard, enemy_attacks: BitBoard) -> bool {
        if !self.board_state.has_castling_rights_kingside_black() {
            return false;
        }

        if self.is_in_check_black(enemy_attacks) {
            return false;
        }

        if !((bitboard & KINGSIDE_BLACK_SQUARES).is_empty()) {
            return false;
        }

        if !((enemy_attacks & KINGSIDE_BLACK_SQUARES).is_empty()) {
            return false;
        }

        true
    }

    /// Returns `true` if [white](crate::Color::White) can castle queenside, and `false` otherwise.
    pub fn can_castle_queenside_white(&self, bitboard: BitBoard, enemy_attacks: BitBoard) -> bool {
        if !self.board_state.has_castling_rights_queenside_white() {
            return false;
        }

        if self.is_in_check_white(enemy_attacks) {
            return false;
        }

        if !((bitboard & (QUEENSIDE_WHITE_SQUARES | QUEENSIDE_ROOK_SQUARE_WHITE)).is_empty()) {
            return false;
        }

        if !((enemy_attacks & QUEENSIDE_WHITE_SQUARES).is_empty()) {
            return false;
        }

        true
    }

    /// Returns `true` if [black](crate::Color::Black) can castle queenside, and `false` otherwise.
    pub fn can_castle_queenside_black(&self, bitboard: BitBoard, enemy_attacks: BitBoard) -> bool {
        if !self.board_state.has_castling_rights_queenside_black() {
            return false;
        }

        if self.is_in_check_black(enemy_attacks) {
            return false;
        }

        if !((bitboard & (QUEENSIDE_BLACK_SQUARES | QUEENSIDE_ROOK_SQUARE_BLACK)).is_empty()) {
            return false;
        }

        if !((enemy_attacks & QUEENSIDE_BLACK_SQUARES).is_empty()) {
            return false;
        }

        true
    }

    pub fn enumerate_moves(&self) -> MoveList {
        let mut moves = MoveList::new();

        match self.board_state.side_to_move {
            Color::White => {
                let friendly_pieces = self.board_state.position.white();
                let enemy_pieces = self.board_state.position.black();
                //let enemy_attacks = self.get_attacks(Color::Black);

                for initial_square in friendly_pieces {
                    match self.board_state.position.piece_at(initial_square) {
                        Some((Color::White, Piece::Pawn)) => {
                            self.enumerate_white_pawn_moves(
                                initial_square,
                                friendly_pieces,
                                enemy_pieces,
                                &mut moves,
                            );
                        }
                        Some((Color::White, Piece::Knight)) => self.enumerate_white_knight_moves(
                            initial_square,
                            friendly_pieces,
                            enemy_pieces,
                            &mut moves,
                        ),
                        Some((Color::White, Piece::King)) => self.enumerate_white_king_moves(
                            initial_square,
                            friendly_pieces,
                            enemy_pieces,
                            &mut moves,
                        ),
                        Some((Color::White, Piece::Rook)) => self.enumerate_white_rook_moves(
                            initial_square,
                            friendly_pieces,
                            enemy_pieces,
                            &mut moves,
                        ),
                        Some((Color::White, Piece::Bishop)) => self.enumerate_white_bishop_moves(
                            initial_square,
                            friendly_pieces,
                            enemy_pieces,
                            &mut moves,
                        ),
                        Some((Color::White, Piece::Queen)) => self.enumerate_white_queen_moves(
                            initial_square,
                            friendly_pieces,
                            enemy_pieces,
                            &mut moves,
                        ),
                        _ => continue,
                    };
                }
            }
            Color::Black => {
                let friendly_pieces = self.board_state.position.black();
                let enemy_pieces = self.board_state.position.white();

                for initial_square in friendly_pieces {
                    match self.board_state.position.piece_at(initial_square) {
                        Some((Color::Black, Piece::Pawn)) => {
                            self.enumerate_black_pawn_moves(
                                initial_square,
                                friendly_pieces,
                                enemy_pieces,
                                &mut moves,
                            );
                        }
                        Some((Color::Black, Piece::Knight)) => self.enumerate_black_knight_moves(
                            initial_square,
                            friendly_pieces,
                            enemy_pieces,
                            &mut moves,
                        ),
                        Some((Color::Black, Piece::King)) => self.enumerate_black_king_moves(
                            initial_square,
                            friendly_pieces,
                            enemy_pieces,
                            &mut moves,
                        ),
                        Some((Color::Black, Piece::Rook)) => self.enumerate_black_rook_moves(
                            initial_square,
                            friendly_pieces,
                            enemy_pieces,
                            &mut moves,
                        ),
                        Some((Color::Black, Piece::Bishop)) => self.enumerate_black_bishop_moves(
                            initial_square,
                            friendly_pieces,
                            enemy_pieces,
                            &mut moves,
                        ),
                        Some((Color::Black, Piece::Queen)) => self.enumerate_black_queen_moves(
                            initial_square,
                            friendly_pieces,
                            enemy_pieces,
                            &mut moves,
                        ),
                        _ => continue,
                    }
                }
            }
        };

        moves
    }

    pub fn enumerate_white_pawn_moves(
        &self,
        initial_square: Square,
        friendly_pieces: BitBoard,
        enemy_pieces: BitBoard,
        moves: &mut MoveList,
    ) {
        let mut target_squares =
            self.move_gen.get_white_pawn_moves(initial_square) & !(friendly_pieces | enemy_pieces);

        let en_passant_square = self
            .board_state
            .en_passant_square
            .unwrap_or(Square::A1)
            .to_bitboard();

        target_squares |= self.move_gen.get_white_pawn_attacks(initial_square)
            & (enemy_pieces | en_passant_square);

        for target_square in target_squares {
            let target_square_bit = target_square.to_bitboard();

            let mut mv = Move::unchecked_from_squares(initial_square, target_square);

            match (target_square - initial_square) as u8 {
                16 if !(((enemy_pieces | friendly_pieces) & (target_square_bit >> 8)).is_empty()) => continue,
                16 => mv.set_double_pawn_push(),
                _ => (),
            };

            if !((target_square_bit & enemy_pieces).is_empty()) {
                mv.set_capture();
            }

            if !((target_square_bit & en_passant_square).is_empty()) {
                mv.set_en_passant_capture();

                if self.would_check_white(mv) {
                    continue;
                };

                moves.push(MoveScore::new(mv));
                continue;
            }

            if !((target_square_bit & RANK_8).is_empty()) {
                if self.would_check_white(mv) {
                    continue;
                }

                let mut knight_promotion = mv.clone();
                knight_promotion.add_knight_promotion();
                moves.push(MoveScore::new(knight_promotion));

                let mut rook_promotion = mv.clone();
                rook_promotion.add_rook_promotion();
                moves.push(MoveScore::new(rook_promotion));

                let mut bishop_promotion = mv.clone();
                bishop_promotion.add_bishop_promotion();
                moves.push(MoveScore::new(bishop_promotion));

                mv.add_queen_promotion();
                moves.push(MoveScore::new(mv));

                continue;
            }

            if self.would_check_white(mv) {
                continue;
            }

            moves.push(MoveScore::new(mv));
        }
    }

    pub fn enumerate_black_pawn_moves(
        &self,
        initial_square: Square,
        friendly_pieces: BitBoard,
        enemy_pieces: BitBoard,
        moves: &mut MoveList,
    ) {
        let mut target_squares =
            self.move_gen.get_black_pawn_moves(initial_square) & !(friendly_pieces | enemy_pieces);

        let en_passant_square = self
            .board_state
            .en_passant_square
            .unwrap_or(Square::H8)
            .to_bitboard();

        target_squares |= self.move_gen.get_black_pawn_attacks(initial_square)
            & (enemy_pieces | en_passant_square);

        for target_square in target_squares {
            let target_square_bit = target_square.to_bitboard();

            let mut mv = Move::unchecked_from_squares(initial_square, target_square);

            match (initial_square - target_square) as u8 {
                16 if !((enemy_pieces & (target_square_bit << 8)).is_empty()) => continue,
                16 => mv.set_double_pawn_push(),
                _ => (),
            };

            if !((target_square_bit & enemy_pieces).is_empty()) {
                mv.set_capture();
            }

            if !((target_square_bit & en_passant_square).is_empty()) {
                mv.set_en_passant_capture();

                if self.would_check_black(mv) {
                    continue;
                };

                moves.push(MoveScore::new(mv));
                continue;
            }

            if !((target_square_bit & RANK_1).is_empty()) {
                if self.would_check_black(mv) {
                    continue;
                }

                let mut knight_promotion = mv.clone();
                knight_promotion.add_knight_promotion();
                moves.push(MoveScore::new(knight_promotion));

                let mut rook_promotion = mv.clone();
                rook_promotion.add_rook_promotion();
                moves.push(MoveScore::new(rook_promotion));

                let mut bishop_promotion = mv.clone();
                bishop_promotion.add_bishop_promotion();
                moves.push(MoveScore::new(bishop_promotion));

                mv.add_queen_promotion();
                moves.push(MoveScore::new(mv));

                continue;
            }

            if self.would_check_black(mv) {
                continue;
            }

            moves.push(MoveScore::new(mv));
        }
    }

    pub fn enumerate_white_knight_moves(
        &self,
        initial_square: Square,
        friendly_pieces: BitBoard,
        enemy_pieces: BitBoard,
        moves: &mut MoveList,
    ) {
        let target_squares = self.move_gen.get_knight_attacks(initial_square) & !friendly_pieces;

        for target_square in target_squares {
            let target_square_bit = target_square.to_bitboard();

            let mut mv = Move::unchecked_from_squares(initial_square, target_square);

            if !((target_square_bit & enemy_pieces).is_empty()) {
                mv.set_capture();
            }

            if self.would_check_white(mv) {
                continue;
            }

            moves.push(MoveScore::new(mv));
        }
    }

    pub fn enumerate_black_knight_moves(
        &self,
        initial_square: Square,
        friendly_pieces: BitBoard,
        enemy_pieces: BitBoard,
        moves: &mut MoveList,
    ) {
        let target_squares = self.move_gen.get_knight_attacks(initial_square) & !friendly_pieces;

        for target_square in target_squares {
            let target_square_bit = target_square.to_bitboard();

            let mut mv = Move::unchecked_from_squares(initial_square, target_square);

            if !((target_square_bit & enemy_pieces).is_empty()) {
                mv.set_capture();
            }

            if self.would_check_black(mv) {
                continue;
            }

            moves.push(MoveScore::new(mv));
        }
    }

    pub fn enumerate_white_king_moves(
        &self,
        initial_square: Square,
        friendly_pieces: BitBoard,
        enemy_pieces: BitBoard,
        moves: &mut MoveList,
    ) {
        let target_squares = self.move_gen.get_king_attacks(initial_square) & !friendly_pieces;

        for target_square in target_squares {
            let target_square_bit = target_square.to_bitboard();

            let mut mv = Move::unchecked_from_squares(initial_square, target_square);

            if !((target_square_bit & enemy_pieces).is_empty()) {
                mv.set_capture();
            }

            if self.would_check_white(mv) {
                continue;
            }

            moves.push(MoveScore::new(mv));
        }

        let enemy_attacks = self.get_attacks(Color::Black);

        if self.can_castle_kingside_white(friendly_pieces | enemy_pieces, enemy_attacks) {
            let mut mv = Move::new();
            mv.set_kingside_castle();
            moves.push(MoveScore::new(mv));
        }

        if self.can_castle_queenside_white(friendly_pieces | enemy_pieces, enemy_attacks) {
            let mut mv = Move::new();
            mv.set_queenside_castle();
            moves.push(MoveScore::new(mv));
        }
    }

    pub fn enumerate_black_king_moves(
        &self,
        initial_square: Square,
        friendly_pieces: BitBoard,
        enemy_pieces: BitBoard,
        moves: &mut MoveList,
    ) {
        let target_squares = self.move_gen.get_king_attacks(initial_square) & !friendly_pieces;

        for target_square in target_squares {
            let target_square_bit = target_square.to_bitboard();

            let mut mv = Move::unchecked_from_squares(initial_square, target_square);

            if !((target_square_bit & enemy_pieces).is_empty()) {
                mv.set_capture();
            }

            if self.would_check_black(mv) {
                continue;
            }

            moves.push(MoveScore::new(mv));
        }

        let enemy_attacks = self.get_attacks(Color::White);

        if self.can_castle_kingside_black(friendly_pieces | enemy_pieces, enemy_attacks) {
            let mut mv = Move::new();
            mv.set_kingside_castle();
            moves.push(MoveScore::new(mv));
        }

        if self.can_castle_queenside_black(friendly_pieces | enemy_pieces, enemy_attacks) {
            let mut mv = Move::new();
            mv.set_queenside_castle();
            moves.push(MoveScore::new(mv));
        }
    }

    pub fn enumerate_white_rook_moves(
        &self,
        initial_square: Square,
        friendly_pieces: BitBoard,
        enemy_pieces: BitBoard,
        moves: &mut MoveList,
    ) {
        let target_squares =
            MoveGenerator::get_rook_attacks(initial_square, !(friendly_pieces | enemy_pieces))
                & !friendly_pieces;

        for target_square in target_squares {
            let target_square_bit = target_square.to_bitboard();

            let mut mv = Move::unchecked_from_squares(initial_square, target_square);

            if !((target_square_bit & enemy_pieces).is_empty()) {
                mv.set_capture();
            }

            if self.would_check_white(mv) {
                continue;
            }

            moves.push(MoveScore::new(mv));
        }
    }

    pub fn enumerate_black_rook_moves(
        &self,
        initial_square: Square,
        friendly_pieces: BitBoard,
        enemy_pieces: BitBoard,
        moves: &mut MoveList,
    ) {
        let target_squares =
            MoveGenerator::get_rook_attacks(initial_square, !(friendly_pieces | enemy_pieces))
                & !friendly_pieces;

        for target_square in target_squares {
            let target_square_bit = target_square.to_bitboard();

            let mut mv = Move::unchecked_from_squares(initial_square, target_square);

            if !((target_square_bit & enemy_pieces).is_empty()) {
                mv.set_capture();
            }

            if self.would_check_black(mv) {
                continue;
            }

            moves.push(MoveScore::new(mv));
        }
    }

    pub fn enumerate_white_bishop_moves(
        &self,
        initial_square: Square,
        friendly_pieces: BitBoard,
        enemy_pieces: BitBoard,
        moves: &mut MoveList,
    ) {
        let target_squares =
            MoveGenerator::get_bishop_attacks(initial_square, !(friendly_pieces | enemy_pieces))
                & !friendly_pieces;

        for target_square in target_squares {
            let target_square_bit = target_square.to_bitboard();

            let mut mv = Move::unchecked_from_squares(initial_square, target_square);

            if !((target_square_bit & enemy_pieces).is_empty()) {
                mv.set_capture();
            }

            if self.would_check_white(mv) {
                continue;
            }

            moves.push(MoveScore::new(mv));
        }
    }

    pub fn enumerate_black_bishop_moves(
        &self,
        initial_square: Square,
        friendly_pieces: BitBoard,
        enemy_pieces: BitBoard,
        moves: &mut MoveList,
    ) {
        let target_squares =
            MoveGenerator::get_bishop_attacks(initial_square, !(friendly_pieces | enemy_pieces))
                & !friendly_pieces;

        for target_square in target_squares {
            let target_square_bit = target_square.to_bitboard();

            let mut mv = Move::unchecked_from_squares(initial_square, target_square);

            if !((target_square_bit & enemy_pieces).is_empty()) {
                mv.set_capture();
            }

            if self.would_check_black(mv) {
                continue;
            }

            moves.push(MoveScore::new(mv));
        }
    }

    pub fn enumerate_white_queen_moves(
        &self,
        initial_square: Square,
        friendly_pieces: BitBoard,
        enemy_pieces: BitBoard,
        moves: &mut MoveList,
    ) {
        self.enumerate_white_rook_moves(initial_square, friendly_pieces, enemy_pieces, moves);
        self.enumerate_white_bishop_moves(initial_square, friendly_pieces, enemy_pieces, moves);
    }

    pub fn enumerate_black_queen_moves(
        &self,
        initial_square: Square,
        friendly_pieces: BitBoard,
        enemy_pieces: BitBoard,
        moves: &mut MoveList,
    ) {
        self.enumerate_black_rook_moves(initial_square, friendly_pieces, enemy_pieces, moves);
        self.enumerate_black_bishop_moves(initial_square, friendly_pieces, enemy_pieces, moves);
    }

    pub fn unchecked_make_move(&mut self, mv: Move) {
        self.board_state.make_move(mv);
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

        assert_eq!(enemy_attacks.popcount(), 22);
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
