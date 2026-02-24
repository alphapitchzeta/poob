use std::io;
use std::str::FromStr;

use crate::moves::*;
use crate::{boardstate::*, game::*, perft::*};

pub const ID: &str = "Poob 0.1.0 by alphapitchzeta";

pub struct Session {
    game: Game,
}

impl Session {
    pub fn new() -> Self {
        Self { game: Game::new() }
    }

    pub fn set_position(&mut self, position: Position) -> Result<(), CommandExecutionError> {
        let (mut new_game, moves) = match position {
            Position::StartPos(moves_string) => (Game::new(), moves_string),
            Position::Fen((s, moves_string)) => (
                Game::from_fen(&s)
                    .map_err(|e| CommandExecutionError::BoardStateCreationError(e))?,
                moves_string,
            ),
        };

        if let Some(moves_string) = moves {
            let move_results = moves_string
                .split_ascii_whitespace()
                .map(|s| {
                    s.parse::<Move>()
                        .map_err(|e| CommandExecutionError::ParseMoveError(e))
                })
                .collect::<Vec<_>>();

            for move_result in move_results {
                let maybe_legal_move = move_result?;

                let legal_move = new_game
                    .match_move(maybe_legal_move)
                    .ok_or(CommandExecutionError::IllegalMoveError)?;

                new_game.unchecked_make_move(legal_move);
            }
        }

        self.game = new_game;

        Ok(())
    }
}

impl Session {
    pub fn run(&mut self) {
        let mut buf = String::new();

        loop {
            buf.clear();
            io::stdin().read_line(&mut buf).unwrap();

            let command = match buf.trim().parse::<Command>() {
                Ok(c) => c,
                Err(e) => {
                    println!("Error: {e:?}");
                    continue;
                }
            };

            match self.execute(command) {
                Ok(true) => (),
                Ok(false) => break,
                Err(e) => println!("Error: {e:?}"),
            };
        }
    }

    fn execute(&mut self, command: Command) -> Result<bool, CommandExecutionError> {
        match command {
            Command::Exit => return Ok(false),
            Command::Display => {
                self.game.print();
                println!("FEN: {}", self.game.to_fen());
            }
            Command::SetPosition(position) => self.set_position(position)?,
            Command::Move(mv) => {
                let checked_move = self
                    .game
                    .match_move(mv)
                    .ok_or(CommandExecutionError::IllegalMoveError)?;
                self.game.unchecked_make_move(checked_move);
            }
            Command::Perft(depth) => self.perft(depth),
            Command::Uci => {
                println!("{ID}");
                println!("uciok")
            }
            Command::IsReady => println!("readyok"),
            Command::UciNewGame => (),
            Command::Go(_) => self.go(),
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

    fn go(&self) {
        match self.game.search_id() {
           Some(best_move) => println!("bestmove {}", best_move.to_string()),
           None => {
               // no legal move: checkmate or stalemate
               // You can print something UCI-like or just a message:
               println!("bestmove (none)");
               // or: println!("mate"); or distinguish checkmate vs stalemate if you add logic
           }
       }
    }

    #[allow(dead_code)]
    fn go_rand(&self) {
        let moves = self.game.enumerate_moves();

        if moves.is_empty() {
            println!("mate");
            return;
        }

        let best_move_index = fastrand::usize(0..moves.len());

        println!(
            "bestmove {}",
            moves.get_move(best_move_index).unwrap().to_string()
        );
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    SetPosition(Position),
    Perft(usize),
    Display,
    Move(Move),
    Uci,
    UciNewGame,
    IsReady,
    Go(Option<String>),
    Exit,
}

#[derive(Debug, Clone, Copy)]
pub enum ParseCommandError {
    BadCommand,
    NoCommand,
    BadPerftDepth,
    NoPerftDepth,
    NoMove,
    ParseMoveError(ParseMoveError),
    ParsePositionError(ParsePositionError),
}

#[derive(Debug, Clone, Copy)]
pub enum CommandExecutionError {
    BoardStateCreationError(BoardStateCreationError),
    ParseMoveError(ParseMoveError),
    IllegalMoveError,
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
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
                    .parse::<Position>()
                    .map_err(|e| ParseCommandError::ParsePositionError(e))?,
            )),
            "perft" => Ok(Command::Perft(
                chunks
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
                    .parse()
                    .map_err(|_| ParseCommandError::BadPerftDepth)?,
            )),
            "move" => Ok(Command::Move(
                chunks
                    .map(|s| s.to_string())
                    .collect::<String>()
                    .parse()
                    .map_err(|e| ParseCommandError::ParseMoveError(e))?,
            )),
            "uci" => Ok(Command::Uci),
            "ucinewgame" => Ok(Command::UciNewGame),
            "isready" => Ok(Command::IsReady),
            "go" => {
                let args = chunks.map(|s| s.to_string()).collect::<Vec<_>>().join(" ");

                let real_args = if args.is_empty() { None } else { Some(args) };

                Ok(Command::Go(real_args))
            }
            _ => Err(ParseCommandError::BadCommand),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Position {
    StartPos(Option<String>),
    Fen((String, Option<String>)),
}

#[derive(Debug, Clone, Copy)]
pub enum ParsePositionError {
    BadPosition,
    NoPosition,
    NoFen,
}

impl FromStr for Position {
    type Err = ParsePositionError;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        let mut moves = None;
        if let Some((fen_s, moves_s)) = s.split_once(" moves ") {
            s = fen_s;

            moves = Some(
                moves_s
                    .split_ascii_whitespace()
                    .map(|str| str.to_string())
                    .collect::<Vec<String>>()
                    .join(" "),
            );
        }

        let mut fen_chunks = s.split_ascii_whitespace();

        match fen_chunks.next().ok_or(ParsePositionError::NoPosition)? {
            "startpos" | "default" => Ok(Position::StartPos(moves)),
            "fen" => Ok(Position::Fen((
                fen_chunks
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join(" "),
                moves,
            ))),
            _ => Err(ParsePositionError::BadPosition),
        }
    }
}
