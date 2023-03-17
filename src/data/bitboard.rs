use super::square::Square;
use std::cmp::PartialEq;
use std::fmt::Display;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not, Shl, Shr};

#[derive(Debug, Clone, Copy, Default, PartialOrd, Ord, Eq, PartialEq)]
pub struct BitBoard(u64);

impl BitBoard {
    pub fn set(&mut self, sq: Square) {
        self.0 |= (1 as u64) << sq as usize;
    }

    pub fn unset(&mut self, sq: Square) {
        self.0 &= !((1 as u64) << sq as usize);
    }

    pub fn is_set(&self, sq: Square) -> bool {
        self.0 & ((1 as u64) << sq as usize) > 0
    }

    pub fn make_move(&mut self, prev: Square, new: Square) {
        self.unset(prev);
        self.set(new);
    }

    pub fn data(&self) -> u64 {
        self.0
    }
}

impl PartialEq<u64> for BitBoard {
    fn eq(&self, other: &u64) -> bool {
        self.0 == *other
    }
}

impl From<BitBoard> for u64 {
    fn from(value: BitBoard) -> Self {
        value.0
    }
}

impl From<u64> for BitBoard {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut b = (0..8)
            .rev()
            .map(|x| {
                format!(
                    "{:08b} {}",
                    ((self.0 & (0xff << x * 8)) >> x * 8) as u8,
                    x + 1
                )
            })
            .map(|s| s.chars().rev().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");
        b.push_str("\n  abcdefgh");
        b = b.replace("0", ".");
        write!(f, "{}", b)
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
        *self = Self(self.0 | rhs.0)
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
        *self = Self(self.0 & rhs.0)
    }
}

impl Shr<BitBoard> for BitBoard {
    type Output = Self;

    fn shr(self, Self(rhs): Self) -> Self::Output {
        let Self(lhs) = self;
        Self(lhs >> rhs)
    }
}

impl Shl<BitBoard> for BitBoard {
    type Output = Self;

    fn shl(self, Self(rhs): Self) -> Self::Output {
        let Self(lhs) = self;
        Self(lhs << rhs)
    }
}

impl Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}
