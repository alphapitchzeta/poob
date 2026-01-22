use crate::bitboards::*;
use crate::boardstate::*;
use crate::rende::print_bitboard;
//use crate::util::*;
use crate::Color;
use crate::util::read_move;

const PROJECTED_GAME_LENGTH: usize = 40;

#[derive(Debug)]
pub struct Game {
    pub board_state: BoardState,
    outcome: Option<Outcome>,
    history: BoardHistory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Win(Color),
    Draw,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board_state: BoardState::default(),
            outcome: None,
            history: BoardHistory::new(),
        }
    }

    pub fn play_sandbox(&mut self) {
        while self.outcome == None {
            self.print();

            let Some(mv) = read_move() else {
                continue;
            };

            self.history.push(self.board_state.clone());

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

    pub fn get_turn(&self, index: usize) -> Option<&BoardState> {
        if index == 0 {
            return None;
        }

        self.history.get(index - 1)
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
