use std::str::FromStr;
use std::io::{self, Write};

use crate::moves::MoveListIterator;
use crate::{boardstate::*, game::*, movegen::MoveGenerator, perft::*};

pub struct Session<'a> {
    game: Game<'a>,
    move_gen: &'a MoveGenerator,
}

impl<'a> Session<'a> {
    pub fn new(move_gen: &'a MoveGenerator) -> Self {
        Self {
            game: Game::new(move_gen),
            move_gen,
        }
    }

    pub fn set_position(&mut self, position: Position) -> Result<(), BoardStateCreationError> {
        match position {
            Position::StartPos => self.game = Game::new(self.move_gen),
            Position::Fen(s) => self.game = Game::from_fen(&s, self.move_gen)?,
        };

        Ok(())
    }
}

impl Session<'_> {
    pub fn run(&mut self) {
        let mut buf = String::new();

        loop {
            buf.clear();

            print!("> ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut buf).unwrap();

            let command = match buf.trim().parse::<Command>() {
                Ok(c) => c,
                Err(e) => {
                    println!("Error: {e:?}");
                    continue;
                },
            };

            match self.execute(command) {
                Ok(true) => (),
                Ok(false) => break,
                Err(e) => println!("Error: {:?}", e),
            };
        }
    }

    fn execute(&mut self, command: Command) -> Result<bool, BoardStateCreationError> {
        match command {
            Command::Exit => return Ok(false),
            Command::Display => self.game.print(),
            Command::SetPosition(position) => self.set_position(position)?,
            Command::Perft(depth) => self.perft(depth),
        };

        Ok(true)
    }

    fn perft(&self, depth: usize) {
        let mut moves = self.game.enumerate_moves();
        moves.sort_alphanumeric();
        let moves_iter = MoveListIterator::new(&moves);

        let mut nodes = 0;

        for move_score in moves_iter {
            let mv = move_score.mv;

            let mut game = self.game.clone();
            game.unchecked_make_move(mv);

            let searched = perft(depth - 1, game);
            println!("{}: {}", mv.to_string(), searched);
            nodes += searched;
        }

        println!("Total nodes: {}", nodes);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    SetPosition(Position),
    Perft(usize),
    Display,
    Exit,
}

#[derive(Debug, Clone, Copy)]
pub enum ParseCommandError {
    BadCommand,
    NoCommand,
    BadPerftDepth,
    NoPerftDepth,
    ParsePositionError(ParsePositionError),
}

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chunks = s.split_ascii_whitespace();

        match chunks.next().ok_or(ParseCommandError::NoCommand)? {
            "exit" | "quit" => Ok(Command::Exit),
            "d" | "display" => Ok(Command::Display),
            "position" => Ok(Command::SetPosition(
                chunks
                    .collect::<String>()
                    .parse::<Position>()
                    .map_err(|e| ParseCommandError::ParsePositionError(e))?,
            )),
            "perft" => Ok(Command::Perft(
                chunks
                .collect::<String>()
                .parse()
                .map_err(|_| ParseCommandError::BadPerftDepth)?,
            )),
            _ => Err(ParseCommandError::BadCommand),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Position {
    StartPos,
    Fen(String),
}

#[derive(Debug, Clone, Copy)]
pub enum ParsePositionError {
    BadPosition,
    NoPosition,
    NoFen,
}

impl FromStr for Position {
    type Err = ParsePositionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chunks = s.split_ascii_whitespace();

        match chunks.next().ok_or(ParsePositionError::NoPosition)? {
            "startpos" | "default" => Ok(Position::StartPos),
            "fen" => Ok(Position::Fen(chunks.collect())),
            _ => Err(ParsePositionError::BadPosition),
        }
    }
}
