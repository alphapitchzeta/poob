use crate::game::Game;
use crate::moves::*;
use crate::pst::PIECE_SQUARE_TABLES;
use crate::{Color, Piece};

use material_values::*;

impl Game {
    pub fn eval_net_material(&self) -> i32 {
        self.eval_material(self.side_to_move()) - self.eval_material(self.side_to_move().enemy())
    }

    pub fn eval_material(&self, eval_color: Color) -> i32 {
        let position = self.get_position();

        let eval_color_pieces = match eval_color {
            Color::White => position.white(),
            Color::Black => position.black(),
        };

        let mut sum = 0;

        sum += (eval_color_pieces & position.pawns()).popcount() * PAWN_VALUE;
        sum += (eval_color_pieces & position.knights()).popcount() * KNIGHT_VALUE;
        sum += (eval_color_pieces & position.bishops()).popcount() * BISHOP_VALUE;
        sum += (eval_color_pieces & position.rooks()).popcount() * ROOK_VALUE;
        sum += (eval_color_pieces & position.queens()).popcount() * QUEEN_VALUE;
        sum += (eval_color_pieces & position.kings()).popcount() * KING_VALUE; // May be redundant

        sum
    }

    /// Method for calculating the current [phase](https://www.chessprogramming.org/Game_Phases)
    /// of the game for [tapered eval](https://www.chessprogramming.org/Tapered_Eval). Current
    /// implementation is adapted from
    /// [a guide by Jonatan Pettersson](https://mediocrechess.blogspot.com/2011/10/guide-tapered-eval.html).
    /// A value closer to `24` indicates the early to mid-game, while a value closer to `0`
    /// indicates the late game.
    pub fn phase(&self) -> i32 {
        let knight_phase = 1;
        let bishop_phase = 1;
        let rook_phase = 2;
        let queen_phase = 4;

        //let total_phase = knight_phase * 4 + bishop_phase * 4 + rook_phase * 4 + queen_phase * 2;

        let mut phase = 0;
        let position = self.get_position();

        phase += position.knights().popcount() * knight_phase;
        phase += position.bishops().popcount() * bishop_phase;
        phase += position.rooks().popcount() * rook_phase;
        phase += position.queens().popcount() * queen_phase;

        //phase = (phase * 256 + (total_phase / 2)) / total_phase;

        phase.clamp(0, 24)
    }

    pub fn get_ordered_moves(&self) -> MoveList {
        let mut move_list = self.enumerate_moves();
        let mailbox = self.get_mailbox();

        for move_score in MoveListIteratorMut::new(&mut move_list) {
            let target_square = move_score.mv.target_square();

            let (color, piece) = mailbox
                .piece_at(move_score.mv.initial_square())
                .unwrap_or((self.side_to_move(), Piece::Pawn));

            // Give the move a bonus or penalty based on the score from the corresponding piece-square table
            *move_score +=
                PIECE_SQUARE_TABLES.score_tapered(target_square, color, piece, self.phase());

            if move_score.mv.is_capture() {
                let (_, captured_piece) = mailbox
                    .piece_at(target_square)
                    .unwrap_or((self.side_to_move().enemy(), Piece::Pawn));

                // Add the difference between the captured piece's value and the moved piece's value
                *move_score += captured_piece.value() - piece.value();
            }
        }

        move_list.sort();

        move_list
    }

    pub fn evaluate_position(&self) -> i32 {
        self.eval_net_material()
    }
}

/// Constants for measuring the value of each [piece](Piece). Measured using the
/// [centipawn](https://www.chessprogramming.org/Centipawns) scale. Current values
/// are based on the ones defined by Larry Kaufman as listed on the
/// [Chess Programming Wiki](https://www.chessprogramming.org/Point_Value).
pub mod material_values {
    pub const PAWN_VALUE: i32 = 100;
    pub const KNIGHT_VALUE: i32 = 350;
    pub const BISHOP_VALUE: i32 = 350;
    pub const ROOK_VALUE: i32 = 525;
    pub const QUEEN_VALUE: i32 = 1000;
    pub const KING_VALUE: i32 = 10000;
}

pub mod scores {
    pub const MAX_SCORE: i32 = i16::MAX as i32;
    pub const MIN_SCORE: i32 = i16::MIN as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_evaluation() {
        let mut new_game = Game::new();
        let mut phase = new_game.phase();
        assert_eq!(phase, 24);

        let fen = "k7/8/8/8/8/8/8/K7 w - - 0 1";
        new_game = Game::from_fen(fen).unwrap();
        phase = new_game.phase();
        assert_eq!(phase, 0);
    }
}
