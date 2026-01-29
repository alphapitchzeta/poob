use crate::moves::Move;
use std::io::{self, Write};

/// Helper function to convert a square string slice (`"a8"`, `"h3"`, etc.)
/// to a [`u8`] square index. Returns [`None`] if the string slice is malformed.
pub fn square_str_to_index(s: &str) -> Option<u8> {
    if s.len() != 2 {
        return None;
    }

    let mut chars = s.chars();
    let file_char = chars.next()?;
    let rank_char = chars.next()?;

    let file = match file_char {
        'a'..='h' => file_char as u8 - 'a' as u8,
        _ => return None,
    };

    let rank = match rank_char {
        '1'..='8' => rank_char as u8 - '1' as u8,
        _ => return None,
    };

    Some(rank * 8 + file)
}

/// Helper function to convert a [`u8`] square index to a square [`String`]
/// (`"a8"`, `"h3"`, etc.). Returns [`None`] if the index is invalid (not
/// in the range `0..64`).
pub fn index_to_square_str(index: u8) -> Option<String> {
    match index {
        0..64 => (),
        _ => return None,
    };

    let mut s = String::with_capacity(2);

    s.push((index % 8 + 'a' as u8) as char);
    s.push((index / 8 + '1' as u8) as char);

    Some(s)
}

/// Helper function to convert a [`u8`] square index to a [`u16`] square
/// index. Returns [`None`] if the index is invalid (not in the range
/// `0..64`).
pub fn checked_square_u8_to_square_u16(square_u8: u8) -> Option<u16> {
    match square_u8 {
        0..64 => (),
        _ => return None,
    };

    Some(square_u8 as u16)
}

/// Helper function to read a [`Move`] from [`stdin()`](std::io::stdin). Loops until a valid
/// move is inputted.
///
/// # Panics
/// Calls [`unwrap()`](std::option::Option::unwrap) on [`flush()`](Write::flush) and `read_line()`.
pub fn read_move() -> Option<Move> {
    let mut buf = String::with_capacity(6);

    loop {
        print!("Make a move: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut buf).unwrap();

        if buf.trim().split_ascii_whitespace().count() == 2 {
            break;
        }

        buf.clear();
    }

    let mut squares = buf.trim().split_ascii_whitespace();

    let i_square_str = squares.next()?;
    let t_square_str = squares.next()?;

    Move::from_squares_str(i_square_str, t_square_str)
}

#[derive(Debug, Clone, Copy)]
pub struct SquareList {
    squares: [u8; 64],
    len: usize,
}

impl SquareList {
    pub fn new() -> Self {
        SquareList {
            squares: [0; 64],
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get(&self, index: usize) -> Option<u8> {
        if index >= self.len {
            return None;
        }

        let element = *self.squares.get(index)?;

        Some(element)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_str_to_index() {
        assert_eq!(square_str_to_index("lmao"), None);
        assert_eq!(square_str_to_index("h3"), Some(23));
        assert_eq!(square_str_to_index("a1"), Some(0));
        assert_eq!(square_str_to_index("h8"), Some(63));
    }

    #[test]
    fn test_index_to_square_str() {
        assert_eq!(index_to_square_str(69), None);
        assert_eq!(index_to_square_str(23), Some("h3".to_string()));
        assert_eq!(index_to_square_str(0), Some("a1".to_string()));
        assert_eq!(index_to_square_str(63), Some("h8".to_string()))
    }
}
