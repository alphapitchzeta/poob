use crate::{eval::scores::*, game::Game, moves::*};

const DEFAULT_DEPTH: usize = 4;

impl Game {
    pub fn search(&self) -> Move {
        let mut move_list = self.get_ordered_moves();

        for move_score in MoveListIteratorMut::new(&mut move_list) {
            let mut next_turn = self.clone();
            next_turn.unchecked_make_move(move_score.mv);

            //*move_score += next_turn.negamax(DEFAULT_DEPTH);
            *move_score += next_turn.alpha_beta(MIN_SCORE, MAX_SCORE, DEFAULT_DEPTH);
        }

        move_list.get_best_move()
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

            let score = next_turn.negamax(depth - 1) * -1;

            if score > max {
                max = score;
            }
        }

        max
    }

    pub fn alpha_beta(&self, mut alpha: i32, beta: i32, depth: usize) -> i32 {
        if depth == 0 {
            return self.evaluate_position();
        }

        let mut best_score = MIN_SCORE;
        let move_list = self.get_ordered_moves();

        for move_score in MoveListIterator::new(&move_list) {
            let mut next_turn = self.clone();
            next_turn.unchecked_make_move(move_score.mv);

            let score = next_turn.alpha_beta(beta * -1, alpha * -1, depth - 1) * -1;

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
}
