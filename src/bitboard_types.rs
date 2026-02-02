use std::ops::{
    Add, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign, Sub,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub fn contains(self, square: Square) -> bool {
        self.0 & (1 << square as u8) != 0
    }

    pub fn is_empty(self) -> bool {
        self.0.count_ones() == 0
    }

    pub fn popcount(self) -> u8 {
        self.0.count_ones() as u8
    }

    pub fn lsb(self) -> Square {
        Square::new(self.0.trailing_zeros() as u8)
    }

    pub fn set(&mut self, square: Square) {
        self.0 |= 1 << square as u8;
    }

    pub fn clear(&mut self, square: Square) {
        self.0 &= !(1 << square as u8);
    }
}

impl Iterator for BitBoard {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            return None;
        }

        let lsb = self.lsb();
        self.0 &= self.0 - 1;
        Some(lsb)
    }
}

impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitXor for BitBoard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

pub const SQUARES: u8 = 64;

#[repr(u8)]
#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
    pub fn index(self) -> u8 {
        self as u8
    }

    pub const fn new(value: u8) -> Self {
        debug_assert!(value < SQUARES);

        unsafe { std::mem::transmute(value) }
    }
}

impl Add for Square {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self as u8 + rhs as u8)
    }
}

impl Sub for Square {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self as u8 - rhs as u8)
    }
}

impl BitXor<u8> for Square {
    type Output = Self;

    fn bitxor(self, rhs: u8) -> Self::Output {
        Self::new(self as u8 ^ rhs)
    }
}

impl BitXorAssign<u8> for Square {
    fn bitxor_assign(&mut self, rhs: u8) {
        *self = self.bitxor(rhs)
    }
}

impl Shl<u8> for Square {
    type Output = Self;

    fn shl(self, rhs: u8) -> Self::Output {
        Self::new((self as u8) << rhs)
    }
}

impl ShlAssign<u8> for Square {
    fn shl_assign(&mut self, rhs: u8) {
        *self = self.shl(rhs)
    }
}

impl Shr<u8> for Square {
    type Output = Self;

    fn shr(self, rhs: u8) -> Self::Output {
        Self::new((self as u8) >> rhs)
    }
}

impl ShrAssign<u8> for Square {
    fn shr_assign(&mut self, rhs: u8) {
        *self = *self >> rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitboard_iterator() {
        let mut test_bitboard = BitBoard(0);

        for i in (0..SQUARES).step_by(8) {
            test_bitboard.set(Square::new(i));
        }

        for (set_bit, square) in test_bitboard.zip((0..SQUARES).step_by(8)) {
            assert_eq!(set_bit, Square::new(square))
        }

        //assert_eq!(test_bitboard, BitBoard(0));
    }
}
