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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_compute_knight_attacks() {
        let knight_attacks = compute_knight_attacks();

        assert_eq!(knight_attacks[0].count_ones(), 2);
        //assert_eq!(knight_attacks[0], 0b00000010_00000100_00000000);

        assert_eq!(knight_attacks[28].count_ones(), 8);
    }
}
