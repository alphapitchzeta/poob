use crate::bitboards::bitboard_constants::rank_file::*;

pub fn compute_knight_attacks() -> [u64; 64] {
    let mut attacks = [0; 64];

    let not_h = !FILE_H;
    let not_a = !FILE_A;
    let not_gh = !(FILE_G | FILE_H);
    let not_ab = !(FILE_A | FILE_B);

    for square in 0..64 {
        let knight = 1 << square;

        attacks[square] = knight << 17 & not_h
            | knight << 15 & not_a
            | knight << 10 & not_gh
            | knight << 6 & not_ab
            | knight >> 17 & not_a
            | knight >> 15 & not_h
            | knight >> 10 & not_ab
            | knight >> 6 & not_gh;
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
        assert_eq!(knight_attacks[0], 0b00000010_00000100_00000000);
    }
}
