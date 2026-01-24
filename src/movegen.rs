use crate::bitboards::bitboard_constants::rank_file::*;

pub mod movegen_constants {
    use super::*;
    pub mod file_exclusions {
        use super::*;

        pub const NOT_A: u64 = !FILE_A;
        pub const NOT_H: u64 = !FILE_H;
        pub const NOT_AB: u64 = !(FILE_A | FILE_B);
        pub const NOT_GH: u64 = !(FILE_G | FILE_H);
    }
}

use movegen_constants::file_exclusions::*;

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
        assert_eq!(knight_attacks[0], 0b00000010_00000100_00000000);

        assert_eq!(knight_attacks[28].count_ones(), 8);
    }

    #[test]
    fn test_compute_king_attacks() {
        let king_attacks = compute_king_attacks();

        assert_eq!(king_attacks[0].count_ones(), 3);
        assert_eq!(king_attacks[4].count_ones(), 5);
        assert_eq!(king_attacks[28].count_ones(), 8);
    }
}
