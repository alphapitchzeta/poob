use std::io;
use std::time::Instant;
use std::str::{FromStr, SplitAsciiWhitespace};

use crate::search::{MAX_PLY, MAX_TIME, time_ctrl::Timer};
use crate::{Color, boardstate::*, game::*, moves::*, perft::*};

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
                    eprintln!("Error: {e:?}");
                    continue;
                }
            };

            match self.execute(command) {
                Ok(true) => (),
                Ok(false) => break,
                Err(e) => eprintln!("Error: {e:?}"),
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
            Command::Go(go_command) => self.go(go_command),
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

    fn go(&self, go_command: GoCommand) {
        let (clock_set, increment_set) = match self.game.side_to_move() {
            Color::White => (go_command.white_time, go_command.white_increment),
            Color::Black => (go_command.black_time, go_command.black_increment),
        };

        let search_time = match clock_set {
            Some(clock) => clock / 20 + increment_set.unwrap_or(0) / 2,
            None => go_command.move_time,
        }.clamp(0, go_command.move_time);

        let search_timer = Timer::new(search_time, Instant::now());

        let best_move = self.game.search(go_command.max_ply * 2, search_timer);

        println!("bestmove {}", best_move.to_string());
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

#[derive(Debug, Clone)]
pub enum Command {
    SetPosition(Position),
    Perft(usize),
    Display,
    Move(Move),
    Uci,
    UciNewGame,
    IsReady,
    Go(GoCommand),
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
    ParseGoError(ParseGoError),
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

                let real_args = args
                    .parse()
                    .map_err(|e| ParseCommandError::ParseGoError(e))?;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseGoError {
    ParseIntError,
    InvalidArgument,
    MissingArgument,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GoCommand {
    pub move_time: i32,
    pub max_ply: u16,
    pub white_time: Option<i32>,
    pub black_time: Option<i32>,
    pub white_increment: Option<i32>,
    pub black_increment: Option<i32>,
}

impl FromStr for GoCommand {
    type Err = ParseGoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chunks = s.split_ascii_whitespace();

        let chunk_1 = chunks.next();

        let mut go_command = GoCommand::default();

        match chunk_1 {
            None | Some("infinite") => return Ok(go_command),
            Some("movetime") => go_command.move_time = parse_helper(&mut chunks)?,
            Some("depth") => go_command.max_ply = parse_helper(&mut chunks)?,
            Some("wtime") => go_command.white_time = Some(parse_helper(&mut chunks)?),
            Some("btime") => go_command.black_time = Some(parse_helper(&mut chunks)?),
            Some("winc") => go_command.white_increment = Some(parse_helper(&mut chunks)?),
            Some("binc") => go_command.black_increment = Some(parse_helper(&mut chunks)?),
            _ => return Err(ParseGoError::InvalidArgument),
        };

        while let Some(chunk) = chunks.next() {
            match chunk {
                "movetime" => go_command.move_time = parse_helper(&mut chunks)?,
                "depth" => go_command.max_ply = parse_helper(&mut chunks)?,
                "wtime" => go_command.white_time = Some(parse_helper(&mut chunks)?),
                "btime" => go_command.black_time = Some(parse_helper(&mut chunks)?),
                "winc" => go_command.white_increment = Some(parse_helper(&mut chunks)?),
                "binc" => go_command.black_increment = Some(parse_helper(&mut chunks)?),
                _ => return Err(ParseGoError::InvalidArgument),
            };
        }

        Ok(go_command)
    }
}

fn parse_helper<I: FromStr>(iter: &mut SplitAsciiWhitespace) -> Result<I, ParseGoError> {
    Ok(iter
        .next()
        .ok_or(ParseGoError::MissingArgument)?
        .parse()
        .map_err(|_| ParseGoError::ParseIntError)?)
}

impl GoCommand {
    pub fn default() -> Self {
        Self {
            move_time: MAX_TIME,
            max_ply: MAX_PLY,
            white_time: None,
            black_time: None,
            white_increment: None,
            black_increment: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_go_parsing() {
        let mut control_command = GoCommand::default();

        let default_args = ["", "infinite"];

        for arg in default_args {
            let test_command = arg.parse().expect("malformed argument");
            assert_eq!(control_command, test_command);
        }

        {
            let bad_args = [
                ("lol", Err(ParseGoError::InvalidArgument)),
                ("wtime lmao", Err(ParseGoError::ParseIntError)),
                ("binc", Err(ParseGoError::MissingArgument)),
            ];

            for bad_arg in bad_args {
                let result = bad_arg.0.parse::<GoCommand>();
                assert_eq!(result, bad_arg.1);
            }
        }

        let test_args = (60000, 2, 6767, 2557, 2000, 2000);

        control_command.move_time = test_args.0;
        control_command.max_ply = test_args.1;
        control_command.white_time = Some(test_args.2);
        control_command.black_time = Some(test_args.3);
        control_command.white_increment = Some(test_args.4);
        control_command.black_increment = Some(test_args.5);

        let test_arg = format!(
            "movetime {} depth {} wtime {} btime {} winc {} binc {}",
            test_args.0, test_args.1, test_args.2, test_args.3, test_args.4, test_args.5
        );

        let test_command = test_arg.parse().expect("malformed arguments");

        assert_eq!(control_command, test_command);
    }
}
