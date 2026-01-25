use crate::bitboards::bitboard_constants::rank_file::*;

pub mod movegen_constants {
    use super::*;
    pub mod rank_file_exclusions {
        use super::*;

        pub const NOT_A: u64 = !FILE_A;
        pub const NOT_H: u64 = !FILE_H;
        pub const NOT_AB: u64 = !(FILE_A | FILE_B);
        pub const NOT_GH: u64 = !(FILE_G | FILE_H);

        pub const NOT_1: u64 = !RANK_1;
        pub const NOT_8: u64 = !RANK_8;

        pub const NOT_A1: u64 = !(FILE_A | RANK_1);
        pub const NOT_H8: u64 = !(FILE_H | RANK_8);
        pub const NOT_A8: u64 = !(FILE_A | RANK_8);
        pub const NOT_H1: u64 = !(FILE_H | RANK_1);
    }
}

use movegen_constants::rank_file_exclusions::*;

pub struct MoveGenerator {
    white_pawn_moves: [u64; 64],
    black_pawn_moves: [u64; 64],
    white_pawn_attacks: [u64; 64],
    black_pawn_attacks: [u64; 64],
    knight_attacks: [u64; 64],
    king_attacks: [u64; 64],
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

    pub fn get_white_pawn_moves(&self, square: u8) -> u64 {
        self.white_pawn_moves[square as usize]
    }

    pub fn get_black_pawn_moves(&self, square: u8) -> u64 {
        self.black_pawn_moves[square as usize]
    }

    pub fn get_white_pawn_attacks(&self, square: u8) -> u64 {
        self.white_pawn_attacks[square as usize]
    }

    pub fn get_black_pawn_attacks(&self, square: u8) -> u64 {
        self.black_pawn_attacks[square as usize]
    }

    pub fn get_knight_attacks(&self, square: u8) -> u64 {
        self.knight_attacks[square as usize]
    }

    pub fn get_king_attacks(&self, square: u8) -> u64 {
        self.king_attacks[square as usize]
    }

    pub fn get_rook_attacks(square: u8, open_squares: u64) -> u64 {
        let rook = 1 << square;
        let mut attacks = 0;

        {
            let mut north_fill = rook;

            for _ in 0..8 {
                north_fill <<= 8;
                attacks |= north_fill;

                north_fill &= open_squares & NOT_8;
            }
        }

        {
            let mut south_fill = rook;

            for _ in 0..8 {
                south_fill >>= 8;
                attacks |= south_fill;

                south_fill &= open_squares & NOT_1;
            }
        }

        {
            let mut east_fill = rook;

            for _ in 0..8 {
                east_fill <<= 1;
                attacks |= east_fill;

                east_fill &= open_squares & NOT_H;
            }
        }

        {
            let mut west_fill = rook;

            for _ in 0..8 {
                west_fill >>= 1;
                attacks |= west_fill;

                west_fill &= open_squares & NOT_A;
            }
        }

        attacks
    }

    pub fn get_bishop_attacks(square: u8, open_squares: u64) -> u64 {
        let bishop = 1 << square;
        let mut attacks = 0;

        {
            let mut ne_fill = bishop;

            for _ in 0..8 {
                ne_fill <<= 7;
                attacks |= ne_fill;

                ne_fill &= open_squares & NOT_H8;
            }
        }

        {
            let mut nw_fill = bishop;

            for _ in 0..8 {
                nw_fill <<= 9;
                attacks |= nw_fill;

                nw_fill &= open_squares & NOT_A8;
            }
        }

        {
            let mut sw_fill = bishop;

            for _ in 0..8 {
                sw_fill >>= 7;
                attacks |= sw_fill;

                sw_fill &= open_squares & NOT_A1;
            }
        }

        {
            let mut se_fill = bishop;

            for _ in 0..8 {
                se_fill >>= 9;
                attacks |= se_fill;

                se_fill &= open_squares & NOT_H1;
            }
        }

        attacks
    }
}

pub fn compute_white_pawn_moves() -> [u64; 64] {
    let mut moves = [0; 64];

    for square in 0..64 {
        let pawn = 1 << square;

        moves[square] = pawn << 8;

        if pawn & RANK_2 != 0 {
            moves[square] |= pawn << 16;
        }
    }

    moves
}

pub fn compute_black_pawn_moves() -> [u64; 64] {
    let mut moves = [0; 64];

    for square in 0..64 {
        let pawn = 1 << square;

        moves[square] = pawn >> 8;

        if pawn & RANK_7 != 0 {
            moves[square] |= pawn >> 16;
        }
    }

    moves
}

pub fn compute_white_pawn_attacks() -> [u64; 64] {
    let mut attacks = [0; 64];

    for square in 0..64 {
        let pawn = 1 << square;

        attacks[square] = ((pawn << 9) & NOT_A) | ((pawn << 7) & NOT_H);
    }

    attacks
}

pub fn compute_black_pawn_attacks() -> [u64; 64] {
    let mut attacks = [0; 64];

    for square in 0..64 {
        let pawn = 1 << square;

        attacks[square] = ((pawn >> 9) & NOT_H) | ((pawn >> 7) & NOT_A);
    }

    attacks
}

pub fn compute_knight_attacks() -> [u64; 64] {
    let mut attacks = [0; 64];

    for square in 0..64 {
        let knight = 1 << square;

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

pub fn compute_king_attacks() -> [u64; 64] {
    let mut attacks = [0; 64];

    for square in 0..64 {
        let king = 1 << square;

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

        assert_eq!(white_pawn_moves[0].count_ones(), 1);
        assert_eq!(white_pawn_moves[8].count_ones(), 2);
        assert_eq!(white_pawn_moves[63].count_ones(), 0);
    }

    #[test]
    fn test_compute_black_pawn_moves() {
        let black_pawn_moves = compute_black_pawn_moves();

        assert_eq!(black_pawn_moves[0].count_ones(), 0);
        assert_eq!(black_pawn_moves[28].count_ones(), 1);
        assert_eq!(black_pawn_moves[54].count_ones(), 2);
    }

    #[test]
    fn test_compute_white_pawn_attacks() {
        let white_pawn_attacks = compute_white_pawn_attacks();

        assert_eq!(white_pawn_attacks[0].count_ones(), 1);
        assert_eq!(white_pawn_attacks[28].count_ones(), 2);
        assert_eq!(white_pawn_attacks[60].count_ones(), 0);
    }

    #[test]
    fn test_compute_black_pawn_attacks() {
        let black_pawn_attacks = compute_black_pawn_attacks();

        assert_eq!(black_pawn_attacks[0].count_ones(), 0);
        assert_eq!(black_pawn_attacks[28].count_ones(), 2);
        assert_eq!(black_pawn_attacks[63].count_ones(), 1);
    }

    #[test]
    fn test_compute_knight_attacks() {
        let knight_attacks = compute_knight_attacks();

        assert_eq!(knight_attacks[0].count_ones(), 2);
        assert_eq!(knight_attacks[1].count_ones(), 3);
        assert_eq!(knight_attacks[28].count_ones(), 8);
    }

    #[test]
    fn test_compute_king_attacks() {
        let king_attacks = compute_king_attacks();

        assert_eq!(king_attacks[0].count_ones(), 3);
        assert_eq!(king_attacks[4].count_ones(), 5);
        assert_eq!(king_attacks[28].count_ones(), 8);
    }

    #[test]
    fn test_get_rook_attacks() {
        let attacks_1 = MoveGenerator::get_rook_attacks(0, !0);
        assert_eq!(attacks_1.count_ones(), 14);

        let attacks_2 = MoveGenerator::get_rook_attacks(28, !0);
        assert_eq!(attacks_2.count_ones(), 14);

        let mut bitboard = 0b00010000;
        let attacks_3 = MoveGenerator::get_rook_attacks(3, !bitboard);
        assert_eq!(attacks_3.count_ones(), 11);

        bitboard = 0b00010100 << 24 | 0b00001000 << 40;
        let attacks_4 = MoveGenerator::get_rook_attacks(27, !bitboard);
        assert_eq!(attacks_4.count_ones(), 7);
    }

    #[test]
    fn test_get_bishop_attacks() {
        let attacks_1 = MoveGenerator::get_bishop_attacks(0, !0);
        assert_eq!(attacks_1.count_ones(), 7);

        let attacks_2 = MoveGenerator::get_bishop_attacks(28, !0);
        assert_eq!(attacks_2.count_ones(), 13);

        let bitboard = 0b00010000 << 8;
        let attacks_3 = MoveGenerator::get_bishop_attacks(3, !bitboard);
        assert_eq!(attacks_3.count_ones(), 4);
    }
}
