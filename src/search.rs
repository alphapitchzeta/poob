use crate::{eval::scores::*, game::Game, moves::*/*, pst::PIECE_SQUARE_TABLES*/};

const DEFAULT_DEPTH: usize = 6;

impl Game {
    pub fn search(&self) -> Move {
        let mut move_list = self.get_ordered_moves();

        for move_score in MoveListIteratorMut::new(&mut move_list) {
            let mut next_turn = self.clone();
            next_turn.unchecked_make_move(move_score.mv);

            //*move_score += next_turn.negamax(DEFAULT_DEPTH);
            move_score.set_score(-next_turn.alpha_beta(MIN_SCORE, MAX_SCORE, DEFAULT_DEPTH));
        }

        move_list.get_best_move()
    }

    pub fn search_id(&self) -> Move {
        let mut move_list = self.get_ordered_moves();

        //let mailbox = self.get_mailbox();
        //let phase = self.phase();

        for depth in (2..=DEFAULT_DEPTH).step_by(2) {
            for move_score in MoveListIteratorMut::new(&mut move_list) {
                //let (color, piece) = mailbox.piece_at(move_score.mv.initial_square()).unwrap_or((self.side_to_move(), Piece::Pawn));

                let mut next_turn = self.clone();
                next_turn.unchecked_make_move(move_score.mv);

                move_score.set_score(-next_turn.alpha_beta(MIN_SCORE, MAX_SCORE, depth));
                //*move_score += PIECE_SQUARE_TABLES.score_tapered(move_score.mv.target_square(), color, piece, phase);
            }

            move_list.sort();
        }

        move_list.get(0).unwrap().mv
    }

    pub fn negamax(&self, depth: usize) -> i32 {
        if depth == 0 {
            return self.evaluate_position();
        }

        let mut max = MIN_SCORE;

        let move_list = self.get_ordered_moves();

        for move_score in MoveListIterator::new(&move_list) {
            let mut next_turn = self.clone();
            next_turn.unchecked_make_move(move_score.mv);

            let score = -next_turn.negamax(depth - 1);

            if score > max {
                max = score;
            }
        }

        max
    }

    pub fn alpha_beta(&self, mut alpha: i32, beta: i32, depth: usize) -> i32 {
        if depth == 0 {
            //return self.evaluate_position();
            return self.quiescence_search(alpha, beta);
        }

        let mut best_score = MIN_SCORE;
        let move_list = self.get_ordered_moves();

        for move_score in MoveListIterator::new(&move_list) {
            let mut next_turn = self.clone();
            next_turn.unchecked_make_move(move_score.mv);

            let score = -next_turn.alpha_beta(-beta, -alpha, depth - 1);

            if score > best_score {
                best_score = score;

                if score > alpha {
                    alpha = score;
                }
            }

            if score >= beta {
                return best_score;
            }
        }

        best_score
    }

    pub fn quiescence_search(&self, mut alpha: i32, beta: i32) -> i32 {
        let eval = self.evaluate_position();
        let mut best_score = eval;

        if best_score >= beta {
            return best_score;
        }

        if best_score > alpha {
            alpha = best_score;
        }

        let move_list = self.get_ordered_moves();

        for move_score in MoveListIterator::new(&move_list) {
            if move_score.mv.is_quiet() {
                continue;
            }

            let mut next_turn = self.clone();
            next_turn.unchecked_make_move(move_score.mv);

            let score = -next_turn.quiescence_search(-beta, -alpha);

            if score >= beta {
                return score;
            }

            if score > best_score {
                best_score = score;
            }

            if score > alpha {
                alpha = score;
            }
        }

        best_score
    }
}
