use crate::bitboard_types::*;
use crate::bitboards::bitboard_constants::rank_file::*;

/// Constants used in move generation.
pub mod movegen_constants {
    use super::*;

    /// Constants used to exclude certain ranks and files
    /// (useful for preventing wraps).
    pub mod rank_file_exclusions {
        use super::*;

        pub const NOT_A: BitBoard = BitBoard(!FILE_A.0);
        pub const NOT_H: BitBoard = BitBoard(!FILE_H.0);
        pub const NOT_AB: BitBoard = BitBoard(!(FILE_A.0 | FILE_B.0));
        pub const NOT_GH: BitBoard = BitBoard(!(FILE_G.0 | FILE_H.0));

        pub const NOT_1: BitBoard = BitBoard(!RANK_1.0);
        pub const NOT_8: BitBoard = BitBoard(!RANK_8.0);

        pub const NOT_A1: BitBoard = BitBoard(!(FILE_A.0 | RANK_1.0));
        pub const NOT_H8: BitBoard = BitBoard(!(FILE_H.0 | RANK_8.0));
        pub const NOT_A8: BitBoard = BitBoard(!(FILE_A.0 | RANK_8.0));
        pub const NOT_H1: BitBoard = BitBoard(!(FILE_H.0 | RANK_1.0));
    }
}

use movegen_constants::rank_file_exclusions::*;

/// Struct encapsulating the lookup tables and attack generation
/// for all pieces.
#[derive(Debug)]
pub struct MoveGenerator {
    white_pawn_moves: [BitBoard; SQUARES],
    black_pawn_moves: [BitBoard; SQUARES],
    white_pawn_attacks: [BitBoard; SQUARES],
    black_pawn_attacks: [BitBoard; SQUARES],
    knight_attacks: [BitBoard; SQUARES],
    king_attacks: [BitBoard; SQUARES],
}

impl MoveGenerator {
    pub fn new() -> Self {
        Self {
            white_pawn_moves: compute_white_pawn_moves(),
            black_pawn_moves: compute_black_pawn_moves(),
            white_pawn_attacks: compute_white_pawn_attacks(),
            black_pawn_attacks: compute_black_pawn_attacks(),
            knight_attacks: compute_knight_attacks(),
            king_attacks: compute_king_attacks(),
        }
    }

    pub fn get_white_pawn_moves(&self, square: Square) -> BitBoard {
        self.white_pawn_moves[square as usize]
    }

    pub fn get_black_pawn_moves(&self, square: Square) -> BitBoard {
        self.black_pawn_moves[square as usize]
    }

    pub fn get_white_pawn_attacks(&self, square: Square) -> BitBoard {
        self.white_pawn_attacks[square as usize]
    }

    pub fn get_black_pawn_attacks(&self, square: Square) -> BitBoard {
        self.black_pawn_attacks[square as usize]
    }

    pub fn get_knight_attacks(&self, square: Square) -> BitBoard {
        self.knight_attacks[square as usize]
    }

    pub fn get_king_attacks(&self, square: Square) -> BitBoard {
        self.king_attacks[square as usize]
    }

    /// Calculates all squares a [rook](crate::Piece::Rook) is attacking from the given square.
    /// Current implementation uses [dumb7fill](https://www.chessprogramming.org/Dumb7Fill).
    pub fn get_rook_attacks(square: Square, open_squares: BitBoard) -> BitBoard {
        let rook = square.to_bitboard();
        let mut attacks = BitBoard(0);

        {
            let mut north_fill = rook & NOT_8;

            for _ in 0..8 {
                north_fill <<= 8;
                attacks |= north_fill;

                north_fill &= open_squares & NOT_8;
            }
        }

        {
            let mut south_fill = rook & NOT_1;

            for _ in 0..8 {
                south_fill >>= 8;
                attacks |= south_fill;

                south_fill &= open_squares & NOT_1;
            }
        }

        {
            let mut east_fill = rook & NOT_H;

            for _ in 0..8 {
                east_fill <<= 1;
                attacks |= east_fill;

                east_fill &= open_squares & NOT_H;
            }
        }

        {
            let mut west_fill = rook & NOT_A;

            for _ in 0..8 {
                west_fill >>= 1;
                attacks |= west_fill;

                west_fill &= open_squares & NOT_A;
            }
        }

        attacks
    }

    /// Calculates all squares a [bishop](crate::Piece::Bishop) is attacking from the given square.
    /// Current implementation uses [dumb7fill](https://www.chessprogramming.org/Dumb7Fill).
    pub fn get_bishop_attacks(square: Square, open_squares: BitBoard) -> BitBoard {
        let bishop = square.to_bitboard();
        let mut attacks = BitBoard(0);

        {
            let mut ne_fill = bishop & NOT_H8;

            for _ in 0..8 {
                ne_fill <<= 9;
                attacks |= ne_fill;

                ne_fill &= open_squares & NOT_H8;
            }
        }

        {
            let mut nw_fill = bishop & NOT_A8;

            for _ in 0..8 {
                nw_fill <<= 7;
                attacks |= nw_fill;

                nw_fill &= open_squares & NOT_A8;
            }
        }

        {
            let mut sw_fill = bishop & NOT_A1;

            for _ in 0..8 {
                sw_fill >>= 9;
                attacks |= sw_fill;

                sw_fill &= open_squares & NOT_A1;
            }
        }

        {
            let mut se_fill = bishop & NOT_H1;

            for _ in 0..8 {
                se_fill >>= 7;
                attacks |= se_fill;

                se_fill &= open_squares & NOT_H1;
            }
        }

        attacks
    }

    /// Calculates all squares a [queen](crate::Piece::Queen) is attacking from the given square.
    /// Current implementation uses [dumb7fill](https://www.chessprogramming.org/Dumb7Fill).
    pub fn get_queen_attacks(square: Square, open_squares: BitBoard) -> BitBoard {
        MoveGenerator::get_bishop_attacks(square, open_squares)
            | MoveGenerator::get_rook_attacks(square, open_squares)
    }
}

/// Generates and returns a lookup table of every non-capture move a
/// [white](crate::Color::White) [pawn](crate::Piece::Pawn) can make from each square on the board.
pub fn compute_white_pawn_moves() -> [BitBoard; SQUARES] {
    let mut moves = [BitBoard(0); SQUARES];

    for square in 0..SQUARES {
        let pawn = BitBoard(1 << square);

        moves[square] = pawn << 8;

        if pawn & RANK_2 != BitBoard(0) {
            moves[square] |= pawn << 16;
        }
    }

    moves
}

/// Generates and returns a lookup table of every non-capture move a
/// [black](crate::Color::Black) [pawn](crate::Piece::Pawn) can make from each square on the board.
pub fn compute_black_pawn_moves() -> [BitBoard; SQUARES] {
    let mut moves = [BitBoard(0); SQUARES];

    for square in 0..SQUARES {
        let pawn = BitBoard(1 << square);

        moves[square] = pawn >> 8;

        if pawn & RANK_7 != BitBoard(0) {
            moves[square] |= pawn >> 16;
        }
    }

    moves
}

/// Generates and returns a lookup table of every square a [white](crate::Color::White) [pawn](crate::Piece::Pawn)
/// is attacking from each square on the board.
pub fn compute_white_pawn_attacks() -> [BitBoard; SQUARES] {
    let mut attacks = [BitBoard(0); SQUARES];

    for square in 0..SQUARES {
        let pawn = BitBoard(1 << square);

        attacks[square] = ((pawn << 9) & NOT_A) | ((pawn << 7) & NOT_H);
    }

    attacks
}

/// Generates and returns a lookup table of every square a [black](crate::Color::Black) [pawn](crate::Piece::Pawn)
/// is attacking from each square on the board.
pub fn compute_black_pawn_attacks() -> [BitBoard; SQUARES] {
    let mut attacks = [BitBoard(0); SQUARES];

    for square in 0..SQUARES {
        let pawn = BitBoard(1 << square);

        attacks[square] = ((pawn >> 9) & NOT_H) | ((pawn >> 7) & NOT_A);
    }

    attacks
}

/// Generates and returns a lookup table of every move a [knight](crate::Piece::Knight) can make
/// (and therefore every square it is attacking) from each square on the
/// board.
pub fn compute_knight_attacks() -> [BitBoard; SQUARES] {
    let mut attacks = [BitBoard(0); SQUARES];

    for square in 0..SQUARES {
        let knight = BitBoard(1 << square);

        attacks[square] = ((knight << 17) & NOT_A)
            | ((knight << 15) & NOT_H)
            | ((knight << 10) & NOT_AB)
            | ((knight << 6) & NOT_GH)
            | ((knight >> 17) & NOT_H)
            | ((knight >> 15) & NOT_A)
            | ((knight >> 10) & NOT_GH)
            | ((knight >> 6) & NOT_AB);
    }

    attacks
}

/// Generates and returns a lookup table of every move a [king](crate::Piece::King) can make
/// (and therefore every square it is attacking) from each square on the
/// board.
pub fn compute_king_attacks() -> [BitBoard; SQUARES] {
    let mut attacks = [BitBoard(0); SQUARES];

    for square in 0..SQUARES {
        let king = BitBoard(1 << square);

        let horizontal_attacks = ((king << 1) & NOT_A) | ((king >> 1) & NOT_H);
        attacks[square] = horizontal_attacks;
        attacks[square] |= horizontal_attacks << 8;
        attacks[square] |= horizontal_attacks >> 8;

        attacks[square] |= (king << 8) | (king >> 8);
    }

    attacks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_white_pawn_moves() {
        let white_pawn_moves = compute_white_pawn_moves();

        assert_eq!(white_pawn_moves[0].popcount(), 1);
        assert_eq!(white_pawn_moves[8].popcount(), 2);
        assert_eq!(white_pawn_moves[63].popcount(), 0);
    }

    #[test]
    fn test_compute_black_pawn_moves() {
        let black_pawn_moves = compute_black_pawn_moves();

        assert_eq!(black_pawn_moves[0].popcount(), 0);
        assert_eq!(black_pawn_moves[28].popcount(), 1);
        assert_eq!(black_pawn_moves[54].popcount(), 2);
    }

    #[test]
    fn test_compute_white_pawn_attacks() {
        let white_pawn_attacks = compute_white_pawn_attacks();

        assert_eq!(white_pawn_attacks[0].popcount(), 1);
        assert_eq!(white_pawn_attacks[28].popcount(), 2);
        assert_eq!(white_pawn_attacks[60].popcount(), 0);
    }

    #[test]
    fn test_compute_black_pawn_attacks() {
        let black_pawn_attacks = compute_black_pawn_attacks();

        assert_eq!(black_pawn_attacks[0].popcount(), 0);
        assert_eq!(black_pawn_attacks[28].popcount(), 2);
        assert_eq!(black_pawn_attacks[63].popcount(), 1);
    }

    #[test]
    fn test_compute_knight_attacks() {
        let knight_attacks = compute_knight_attacks();

        assert_eq!(knight_attacks[0].popcount(), 2);
        assert_eq!(knight_attacks[1].popcount(), 3);
        assert_eq!(knight_attacks[28].popcount(), 8);
    }

    #[test]
    fn test_compute_king_attacks() {
        let king_attacks = compute_king_attacks();

        assert_eq!(king_attacks[0].popcount(), 3);
        assert_eq!(king_attacks[4].popcount(), 5);
        assert_eq!(king_attacks[28].popcount(), 8);
    }

    #[test]
    fn test_get_rook_attacks() {
        let attacks_1 = MoveGenerator::get_rook_attacks(Square::new(0), BitBoard(!0));
        assert_eq!(attacks_1.popcount(), 14);

        let attacks_2 = MoveGenerator::get_rook_attacks(Square::new(28), BitBoard(!0));
        assert_eq!(attacks_2.popcount(), 14);

        let mut bitboard = BitBoard(0b00010000);
        let attacks_3 = MoveGenerator::get_rook_attacks(Square::new(3), !bitboard);
        assert_eq!(attacks_3.popcount(), 11);

        bitboard = BitBoard(0b00010100 << 24 | 0b00001000 << 40);
        let attacks_4 = MoveGenerator::get_rook_attacks(Square::new(27), !bitboard);
        assert_eq!(attacks_4.popcount(), 7);
    }

    #[test]
    fn test_get_bishop_attacks() {
        let attacks_1 = MoveGenerator::get_bishop_attacks(Square::new(0), BitBoard(!0));
        assert_eq!(attacks_1.popcount(), 7);

        let attacks_2 = MoveGenerator::get_bishop_attacks(Square::new(28), BitBoard(!0));
        assert_eq!(attacks_2.popcount(), 13);

        let bitboard = BitBoard(0b00010000 << 8);
        let attacks_3 = MoveGenerator::get_bishop_attacks(Square::new(3), !bitboard);
        assert_eq!(attacks_3.popcount(), 4);
    }

    #[test]
    fn test_get_queen_attacks() {
        let attacks_1 = MoveGenerator::get_queen_attacks(Square::new(0), BitBoard(!0));
        assert_eq!(attacks_1.popcount(), 21);

        let attacks_2 = MoveGenerator::get_queen_attacks(Square::new(4), BitBoard(!0));
        assert_eq!(attacks_2.popcount(), 21);

        let attacks_3 = MoveGenerator::get_queen_attacks(Square::new(28), BitBoard(!0));
        assert_eq!(attacks_3.popcount(), 27);

        let bitboard = BitBoard(0b000101000 << 24);
        let attacks_4 = MoveGenerator::get_queen_attacks(Square::new(28), !bitboard);
        assert_eq!(attacks_4.popcount(), 22);
    }
}
