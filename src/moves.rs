/// Constants representing the different arrangements of bitflags
/// for the four leftmost bits of a u16 representing a move.
pub mod move_constants {
    pub const QUIET_MOVE: u16 = 0b0000 << 12;
    pub const DOUBLE_PAWN_PUSH: u16 = 0b0001 << 12;
    pub const KING_CASTLE: u16 = 0b0010 << 12;
    pub const QUEEN_CASTLE: u16 = 0b0011 << 12;
    pub const CAPTURE: u16 = 0b0100 << 12;
    pub const EN_PASSANT_CAPTURE: u16 = 0b0101 << 12;
    pub const PROMOTION: u16 = 0b1000 << 12;
    pub const KNIGHT_PROMOTION: u16 = 0b1000 << 12;
    pub const BISHOP_PROMOTION: u16 = 0b1001 << 12;
    pub const ROOK_PROMOTION: u16 = 0b1010 << 12;
    pub const QUEEN_PROMOTION: u16 = 0b1011 << 12;
    pub const KNIGHT_PROMOTION_CAPTURE: u16 = 0b1100 << 12;
    pub const BISHOP_PROMOTION_CAPTURE: u16 = 0b1101 << 12;
    pub const ROOK_PROMOTION_CAPTURE: u16 = 0b1110 << 12;
    pub const QUEEN_PROMOTION_CAPTURE: u16 = 0b1111 << 12;

    /// Used with a bitwise AND operation to set the bitflag of
    /// a [`Move`](super::Move) to [`QUIET_MOVE`].
    pub const QUIET_MASK: u16 = !(0b1111 << 12);
}

/// The maximum possible moves from any given chess position.
const MAX_POSSIBLE_MOVES: usize = 218;

use crate::util::*;
use move_constants::*;

/// Struct encapsulating the logic for encoding and decoding moves.
/// All information is stored in a [`u16`] field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Move(u16);

impl Move {
    pub fn new() -> Self {
        Self(0)
    }

    /// Returns an optional [`Move`] instance from square indices.
    pub fn from_squares(initial_square: u8, target_square: u8) -> Option<Self> {
        let initial_square = checked_square_u8_to_square_u16(initial_square)?;
        let target_square = checked_square_u8_to_square_u16(target_square)?;

        Some(Self((initial_square << 6) | target_square))
    }

    /// Returns a [`Move`] instance from square indices. Do not call this
    /// with invalid square indices. Bad things will happen.
    pub fn unchecked_from_squares(initial_square: u8, target_square: u8) -> Self {
        Self(((initial_square as u16) << 6) | target_square as u16)
    }

    /// Returns an optional [`Move`] instance from square string slices.
    pub fn from_squares_str(initial_square: &str, target_square: &str) -> Option<Self> {
        let i_square_index = square_str_to_index(initial_square)?;
        let t_square_index = square_str_to_index(target_square)?;

        let new_move = Move::from_squares(i_square_index, t_square_index)?;

        Some(new_move)
    }

    /// Extracts the initial square encoded in the move and returns
    /// it as a [`u8`].
    pub fn get_initial_square(&self) -> u8 {
        ((self.0 >> 6) & 0b111111) as u8
    }

    /// Extracts the target square encoded in the move and returns
    /// it as a [`u8`].
    pub fn get_target_square(&self) -> u8 {
        (self.0 & 0b111111) as u8
    }

    /// Returns `true` if the [`QUIET_MOVE`] bitflag is set, and `false`
    /// otherwise.
    pub fn is_quiet(&self) -> bool {
        (self.0 >> 12) & QUIET_MOVE == QUIET_MOVE
    }

    /// Sets the bitflag of the [`Move`] to [`QUIET_MOVE`].
    pub fn set_quiet(&mut self) {
        self.0 &= QUIET_MASK;
    }

    /// Returns `true` if the [`DOUBLE_PAWN_PUSH`] bitflag is set, and `false`
    /// otherwise.
    pub fn is_double_pawn_push(&self) -> bool {
        self.0 & DOUBLE_PAWN_PUSH == DOUBLE_PAWN_PUSH
    }

    /// Sets the bitflag of the [`Move`] to [`DOUBLE_PAWN_PUSH`].
    pub fn set_double_pawn_push(&mut self) {
        self.0 &= QUIET_MASK;
        self.0 |= DOUBLE_PAWN_PUSH;
    }

    /// Returns `true` if the [`KING_CASTLE`] bitflag is set, and `false`
    /// otherwise.
    pub fn is_kingside_castle(&self) -> bool {
        self.0 & KING_CASTLE == KING_CASTLE
    }

    /// Sets the bitflag of the [`Move`] to [KING_CASTLE].
    pub fn set_kingside_castle(&mut self) {
        self.0 &= QUIET_MASK;
        self.0 |= KING_CASTLE;
    }

    /// Returns `true` if the [`QUEEN_CASTLE`] bitflag is set, and `false`
    /// otherwise.
    pub fn is_queenside_castle(&self) -> bool {
        self.0 & QUEEN_CASTLE == QUEEN_CASTLE
    }

    /// Sets the bitflag of the [`Move`] to [`QUEEN_CASTLE`].
    pub fn set_queenside_castle(&mut self) {
        self.0 &= QUIET_MASK;
        self.0 |= QUEEN_CASTLE;
    }

    /// Returns `true` if the [`CAPTURE`] bitflag is set, and `false`
    /// otherwise.
    pub fn is_capture(&self) -> bool {
        self.0 & CAPTURE == CAPTURE
    }

    /// Sets the bitflag of the [`Move`] to [`CAPTURE`].
    pub fn set_capture(&mut self) {
        self.0 &= QUIET_MASK;
        self.0 |= CAPTURE;
    }

    /// Sets the [`CAPTURE`] bit of the [`Move`] bitflag without altering
    /// the other bits.
    pub fn add_capture(&mut self) {
        self.0 |= CAPTURE;
    }

    /// Returns `true` if the [`EN_PASSANT_CAPTURE`] bitflag is set, and `false`
    /// otherwise.
    pub fn is_en_passant_capture(&self) -> bool {
        self.0 & EN_PASSANT_CAPTURE == EN_PASSANT_CAPTURE
    }

    /// Sets the bitflag of the [`Move`] to [`EN_PASSANT_CAPTURE`].
    pub fn set_en_passant_capture(&mut self) {
        self.0 &= QUIET_MASK;
        self.0 |= EN_PASSANT_CAPTURE;
    }

    /// Returns `true` if the [`PROMOTION`] bitflag is set, and `false`
    /// otherwise.
    pub fn is_promotion(&self) -> bool {
        self.0 & PROMOTION == PROMOTION
    }

    /// Returns `true` if the [`KNIGHT_PROMOTION`] bitflag is set, and `false`
    /// otherwise.
    pub fn is_knight_promotion(&self) -> bool {
        self.0 & KNIGHT_PROMOTION == KNIGHT_PROMOTION
    }

    /// Sets the bitflag of the [`Move`] to [`KNIGHT_PROMOTION`].
    pub fn set_knight_promotion(&mut self) {
        self.0 &= QUIET_MASK;
        self.0 |= KNIGHT_PROMOTION;
    }

    /// Sets the [`KNIGHT_PROMOTION`] bits of the [`Move`] bitflag without
    /// altering the other bits.
    pub fn add_knight_promotion(&mut self) {
        self.0 |= KNIGHT_PROMOTION;
    }

    /// Returns `true` if the [`BISHOP_PROMOTION`] bitflag is set, and `false`
    /// otherwise.
    pub fn is_bishop_promotion(&self) -> bool {
        self.0 & BISHOP_PROMOTION == BISHOP_PROMOTION
    }

    /// Sets the bitflag of the [`Move`] to [`BISHOP_PROMOTION`].
    pub fn set_bishop_promotion(&mut self) {
        self.0 &= QUIET_MASK;
        self.0 |= BISHOP_PROMOTION;
    }

    /// Sets the [`BISHOP_PROMOTION`] bits of the [`Move`] bitflag without
    /// altering the other bits.
    pub fn add_bishop_promotion(&mut self) {
        self.0 |= BISHOP_PROMOTION;
    }

    /// Returns `true` if the [`ROOK_PROMOTION`] bitflag is set, and `false`
    /// otherwise.
    pub fn is_rook_promotion(&self) -> bool {
        self.0 & ROOK_PROMOTION == ROOK_PROMOTION
    }

    /// Sets the bitflag of the [`Move`] to [`ROOK_PROMOTION`].
    pub fn set_rook_promotion(&mut self) {
        self.0 &= QUIET_MASK;
        self.0 |= ROOK_PROMOTION;
    }

    /// Sets the [`ROOK_PROMOTION`] bits of the [`Move`] bitflag without
    /// altering the other bits.
    pub fn add_rook_promotion(&mut self) {
        self.0 |= ROOK_PROMOTION;
    }

    /// Returns `true` if the [`QUEEN_PROMOTION`] bitflag is set, and `false`
    /// otherwise.
    pub fn is_queen_promotion(&self) -> bool {
        self.0 & QUEEN_PROMOTION == QUEEN_PROMOTION
    }

    /// Sets the bitflag of the [`Move`] to [`QUEEN_PROMOTION`].
    pub fn set_queen_promotion(&mut self) {
        self.0 &= QUIET_MASK;
        self.0 |= QUEEN_PROMOTION;
    }

    /// Sets the [`QUEEN_PROMOTION`] bits of the [`Move`] bitflag without
    /// altering the other bits.
    pub fn add_queen_promotion(&mut self) {
        self.0 |= QUEEN_PROMOTION;
    }

    /// Returns `true` if the [`KNIGHT_PROMOTION_CAPTURE`] bitflag is set, and `false`
    /// otherwise.
    pub fn is_knight_promotion_capture(&self) -> bool {
        self.0 & KNIGHT_PROMOTION_CAPTURE == KNIGHT_PROMOTION_CAPTURE
    }

    /// Sets the bitflag of the [`Move`] to [`KNIGHT_PROMOTION_CAPTURE`].
    pub fn set_knight_promotion_capture(&mut self) {
        self.0 &= QUIET_MASK;
        self.0 |= KNIGHT_PROMOTION_CAPTURE;
    }

    /// Returns `true` if the [`BISHOP_PROMOTION_CAPTURE`] bitflag is set, and `false`
    /// otherwise.
    pub fn is_bishop_promotion_capture(&self) -> bool {
        self.0 & BISHOP_PROMOTION_CAPTURE == BISHOP_PROMOTION_CAPTURE
    }

    /// Sets the bitflag of the [`Move`] to [`BISHOP_PROMOTION_CAPTURE`].
    pub fn set_bishop_promotion_capture(&mut self) {
        self.0 &= QUIET_MASK;
        self.0 |= BISHOP_PROMOTION_CAPTURE;
    }

    /// Returns `true` if the [`ROOK_PROMOTION_CAPTURE`] bitflag is set, and `false`
    /// otherwise.
    pub fn is_rook_promotion_capture(&self) -> bool {
        self.0 & ROOK_PROMOTION_CAPTURE == ROOK_PROMOTION_CAPTURE
    }

    /// Sets the bitflag of the [`Move`] to [`ROOK_PROMOTION_CAPTURE`].
    pub fn set_rook_promotion_capture(&mut self) {
        self.0 &= QUIET_MASK;
        self.0 |= ROOK_PROMOTION_CAPTURE;
    }

    /// Returns `true` if the [`QUEEN_PROMOTION_CAPTURE`] bitflag is set, and `false`
    /// otherwise.
    pub fn is_queen_promotion_capture(&self) -> bool {
        self.0 & QUEEN_PROMOTION_CAPTURE == QUEEN_PROMOTION_CAPTURE
    }

    /// Sets the bitflag of the [`Move`] to [`QUEEN_PROMOTION_CAPTURE`].
    pub fn set_queen_promotion_capture(&mut self) {
        self.0 &= QUIET_MASK;
        self.0 |= QUEEN_PROMOTION_CAPTURE;
    }
}

/// Struct representing a [`Move`] and its corresponding score.
#[derive(Debug, Clone, Copy)]
pub struct MoveScore {
    pub mv: Move,
    pub score: i32,
}

impl MoveScore {
    pub fn default() -> Self {
        Self {
            mv: Move(0),
            score: 0,
        }
    }

    pub fn new(mv: Move) -> Self {
        Self { mv, score: 0 }
    }

    pub fn with_score(mv: Move, score: i32) -> Self {
        Self { mv, score }
    }
}

/// A custom implementation of an [`ArrayVec`](https://docs.rs/arrayvec/latest/arrayvec/struct.ArrayVec.html),
/// storing a collection of [`MoveScore`].
pub struct MoveList {
    list: [MoveScore; MAX_POSSIBLE_MOVES],
    len: usize,
}

impl MoveList {
    pub fn new() -> Self {
        Self {
            list: [MoveScore::default(); MAX_POSSIBLE_MOVES],
            len: 0,
        }
    }

    /// Returns the current length of the [`MoveList`].
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the capacity of the [`MoveList`] minus its current length.
    pub fn spare_capacity(&self) -> usize {
        MAX_POSSIBLE_MOVES - self.len
    }

    /// Returns `true` if the [`MoveList`] is empty, and `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Sorts the [`MoveScore`] elements in the [`MoveList`] by their scores. Currently,
    /// this sort is unstable.
    pub fn sort(&mut self) {
        self.list[0..self.len].sort_unstable_by_key(|mv| mv.score);
    }

    /// Appends a new [`MoveScore`] to the [`MoveList`].
    pub fn push(&mut self, entry: MoveScore) {
        self.list[self.len] = entry;
        self.len += 1;
    }

    /// Removes the last [`MoveScore`] from the [`MoveList`] (if if exists), returning it.
    pub fn pop(&mut self) -> Option<MoveScore> {
        let popped = *self.list.get(self.len - 1)?;

        self.len -= 1;

        Some(popped)
    }

    /// Returns a `Some(MoveScore)` of the [`MoveScore`] entry at the
    /// corresponding index, or [`None`] if the index is out of bounds.
    pub fn get(&self, index: usize) -> Option<MoveScore> {
        if index >= self.len {
            return None;
        }

        Some(self.list[index])
    }

    /// Returns a `Some(Move)` of the [`MoveScore`] entry at the
    /// corresponding index, or [`None`] if the index is out of bounds.
    pub fn get_move(&self, index: usize) -> Option<Move> {
        let move_score = self.get(index)?;

        Some(move_score.mv)
    }

    /// Calls [`sort()`](Self::sort()) on the [`MoveList`] and returns the [`MoveScore`]
    /// entry with the highest score.
    ///
    /// # Panics
    /// Currently directly indexes the underlying array. The index will
    /// be out of bounds if the current length is `0`.
    pub fn get_best(&mut self) -> MoveScore {
        self.sort();

        self.list[self.len - 1]
    }

    /// Calls [`sort()`](Self::sort()) on the [`MoveList`] and returns the [`Move`] of the
    /// [`MoveScore`] entry with the highest score.
    ///
    /// # Panics
    /// Calls [`get_best()`](Self::get_best()), which panics.
    pub fn get_best_move(&mut self) -> Move {
        self.get_best().mv
    }

    /// Takes ownership of another [`MoveList`], appending its elements to itself.
    pub fn append(&mut self, appended_list: MoveList) {
        for i in 0..appended_list.len {
            self.push(appended_list.list[i]);
        }
    }
}

/// A struct to implement [`Iterator`] for [`MoveList`].
pub struct MoveListIterator<'a> {
    move_list: &'a MoveList,
    position: usize,
}

impl<'a> MoveListIterator<'a> {
    pub fn new(move_list: &'a MoveList) -> Self {
        Self {
            move_list,
            position: 0,
        }
    }
}

impl Iterator for MoveListIterator<'_> {
    type Item = MoveScore;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.move_list.get(self.position);
        self.position += 1;

        item
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_squares() {
        let i_square = 8;
        let t_square = 24;

        assert_eq!(
            Move::from_squares(i_square, t_square),
            Some(Move(0b00000010_00011000))
        );
    }
}
