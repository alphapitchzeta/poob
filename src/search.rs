use crate::{eval::scores::*, game::Game, moves::*, search::time_ctrl::Timer};

pub const MAX_DEPTH: u16 = 6;
pub const MAX_PLY: u16 = MAX_DEPTH / 2;
pub const MAX_TIME: i32 = 30_000;
const TIME_CHECK_INTERVAL: usize = 1024;

#[derive(Debug, Clone, Copy)]
enum SearchResult {
    Score(i32),
    Aborted,
}

impl Game {
    pub fn search(&self, max_depth: u16, timer: Timer) -> Move {
        let mut move_list = self.get_ordered_moves();
        let mut aborted = false;

        for depth in (2..=max_depth).step_by(2) {
            for move_score in MoveListIteratorMut::new(&mut move_list) {

                let mut next_turn = self.clone();
                next_turn.unchecked_make_move(move_score.mv);

                let score = match next_turn.alpha_beta(MIN_SCORE, MAX_SCORE, depth, timer, 0) {
                    SearchResult::Score(s) => -s,
                    SearchResult::Aborted => {
                        aborted = true;
                        break
                    },
                };

                move_score.set_score(score);

                if aborted {
                    break;
                }
            }

            move_list.sort();
        }

        move_list.get(0).unwrap().mv
    }

    fn alpha_beta(&self, mut alpha: i32, beta: i32, depth: u16, timer: Timer, mut nodes: usize) -> SearchResult {
        if depth == 0 {
            //return self.evaluate_position();
            return self.quiescence_search(alpha, beta, timer, nodes);
        }

        let mut best_score = MIN_SCORE;
        let move_list = self.get_ordered_moves();

        for move_score in MoveListIterator::new(&move_list) {
            let mut next_turn = self.clone();
            next_turn.unchecked_make_move(move_score.mv);
            nodes += 1;

            if nodes % TIME_CHECK_INTERVAL == 0 && timer.out_of_time() {
                return SearchResult::Aborted;
            }

            let score = match next_turn.alpha_beta(-beta, -alpha, depth - 1, timer, nodes) {
                SearchResult::Score(s) => -s,
                _ => return SearchResult::Aborted,
            };

            if score > best_score {
                best_score = score;

                if score > alpha {
                    alpha = score;
                }
            }

            if score >= beta {
                return SearchResult::Score(best_score);
            }
        }

        SearchResult::Score(best_score)
    }

    fn quiescence_search(&self, mut alpha: i32, beta: i32, timer: Timer, mut nodes: usize) -> SearchResult {
        let eval = self.evaluate_position();
        let mut best_score = eval;

        if best_score >= beta {
            return SearchResult::Score(best_score);
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
            nodes += 1;

            if nodes % TIME_CHECK_INTERVAL == 0 && timer.out_of_time() {
                return SearchResult::Aborted;
            }

            let score = match next_turn.quiescence_search(-beta, -alpha, timer, nodes) {
                SearchResult::Score(s) => -s,
                _ => return SearchResult::Aborted,
            };

            if score >= beta {
                return SearchResult::Score(score);
            }

            if score > best_score {
                best_score = score;
            }

            if score > alpha {
                alpha = score;
            }
        }

        SearchResult::Score(best_score)
    }
}

pub mod time_ctrl {
    use std::time::Instant;

    #[derive(Debug, Clone, Copy)]
    pub struct Timer {
        time_cap: i32,
        start_time: Instant,
    }

    impl Timer {
        pub fn new(time_cap: i32, start_time: Instant) -> Self {
            Self {
                time_cap,
                start_time,
            }
        }

        pub fn time_remaining(&self) -> i32 {
            self.time_cap - self.start_time.elapsed().as_millis() as i32
        }

        pub fn out_of_time(&self) -> bool {
            self.time_cap - self.start_time.elapsed().as_millis() as i32 <= 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;
    use time_ctrl::Timer;

    fn test_timer_scoped(timer: Timer) -> Timer {
        thread::sleep(Duration::from_millis(101));
        
        timer
    }

    #[test]
    fn test_timer() {
        use std::time::Instant;

        let timer = Timer::new(100, Instant::now());
        assert_eq!(timer.out_of_time(), false);

        let timer = test_timer_scoped(timer);
        assert_eq!(timer.out_of_time(), true);
    }
}
