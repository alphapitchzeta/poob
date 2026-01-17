use crate::bitboards::BitBoards;

pub struct BoardState {
    position: BitBoards,
    turn_count: u16,
    fifty_move_rule: u8,
    can_castle_kingside_white: bool,
    can_castle_kingside_black: bool,
    can_castle_queenside_white: bool,
    can_castle_queenside_black: bool,
}