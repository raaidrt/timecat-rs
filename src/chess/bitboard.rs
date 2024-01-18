use super::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct BitBoard(u64);

impl fmt::Display for BitBoard {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: String = "".to_owned();
        for x in 0..64 {
            if self.get_mask() & (1u64 << x) == (1u64 << x) {
                s.push_str("X ");
            } else {
                s.push_str(". ");
            }
            if x % 8 == 7 {
                s.push('\n');
            }
        }
        write!(f, "{}", s)
    }
}

impl BitBoard {
    #[inline]
    pub const fn new(bb: u64) -> Self {
        Self(bb)
    }

    #[inline]
    pub const fn get_mask(self) -> u64 {
        self.0
    }

    #[inline]
    pub fn set_mask(&mut self, mask: u64) {
        self.0 = mask;
    }

    #[inline]
    pub const fn from_square(sq: Square) -> BitBoard {
        BitBoard(1u64 << sq.to_int())
    }

    #[inline]
    pub const fn from_rank_and_file(rank: Rank, file: File) -> BitBoard {
        BitBoard(1u64 << ((rank.to_int() << 3) + file.to_int()))
    }

    #[inline]
    pub const fn popcnt(self) -> u32 {
        self.get_mask().count_ones()
    }

    #[inline]
    pub const fn reverse_colors(self) -> BitBoard {
        BitBoard(self.get_mask().swap_bytes())
    }

    #[inline]
    pub const fn to_size(self, right_shift: u8) -> usize {
        (self.get_mask() >> right_shift) as usize
    }

    #[inline]
    pub const fn to_square_index(self) -> usize {
        self.get_mask().trailing_zeros() as usize
    }

    #[inline]
    pub fn to_square(self) -> Square {
        unsafe { *ALL_SQUARES.get_unchecked(self.to_square_index()) }
    }
}

macro_rules! implement_bitwise_operations {
    ($direct_trait: ident, $assign_trait: ident, $direct_func: ident, $assign_func: ident) => {
        implement_bitwise_operations!(@integer_implementation $direct_trait, $assign_trait, $direct_func, $assign_func, u128);
        implement_bitwise_operations!(@integer_implementation $direct_trait, $assign_trait, $direct_func, $assign_func, usize);
        implement_bitwise_operations!(@integer_implementation $direct_trait, $assign_trait, $direct_func, $assign_func, u64);
        implement_bitwise_operations!(@integer_implementation $direct_trait, $assign_trait, $direct_func, $assign_func, u32);
        implement_bitwise_operations!(@integer_implementation $direct_trait, $assign_trait, $direct_func, $assign_func, u16);
        implement_bitwise_operations!(@integer_implementation $direct_trait, $assign_trait, $direct_func, $assign_func, u8);
        implement_bitwise_operations!(@integer_implementation $direct_trait, $assign_trait, $direct_func, $assign_func, i128);
        implement_bitwise_operations!(@integer_implementation $direct_trait, $assign_trait, $direct_func, $assign_func, isize);
        implement_bitwise_operations!(@integer_implementation $direct_trait, $assign_trait, $direct_func, $assign_func, i64);
        implement_bitwise_operations!(@integer_implementation $direct_trait, $assign_trait, $direct_func, $assign_func, i32);
        implement_bitwise_operations!(@integer_implementation $direct_trait, $assign_trait, $direct_func, $assign_func, i16);
        implement_bitwise_operations!(@integer_implementation $direct_trait, $assign_trait, $direct_func, $assign_func, i8);

        impl $assign_trait<&BitBoard> for BitBoard {
            fn $assign_func(&mut self, rhs: &Self) {
                self.$assign_func(rhs.get_mask())
            }
        }

        impl $assign_trait for BitBoard {
            fn $assign_func(&mut self, rhs: Self) {
                self.$assign_func(&rhs)
            }
        }

        impl $direct_trait for &BitBoard {
            type Output = BitBoard;

            fn $direct_func(self, rhs: Self) -> Self::Output {
                self.$direct_func(rhs.get_mask())
            }
        }

        impl $direct_trait for BitBoard {
            type Output = Self;

            fn $direct_func(self, rhs: Self) -> Self::Output {
                (&self).$direct_func(&rhs)
            }
        }

        impl $direct_trait<BitBoard> for &BitBoard {
            type Output = BitBoard;

            fn $direct_func(self, rhs: BitBoard) -> Self::Output {
                self.$direct_func(&rhs)
            }
        }

        impl $direct_trait<&BitBoard> for BitBoard {
            type Output = Self;

            fn $direct_func(self, rhs: &Self) -> Self::Output {
                (&self).$direct_func(rhs)
            }
        }
    };

    (@integer_implementation $direct_trait: ident, $assign_trait: ident, $direct_func: ident, $assign_func: ident, $int_type: ident) => {
        impl $assign_trait<$int_type> for BitBoard {
            fn $assign_func(&mut self, rhs: $int_type) {
                self.set_mask(self.get_mask().$direct_func(rhs as u64))
            }
        }

        impl $assign_trait<&$int_type> for BitBoard {
            fn $assign_func(&mut self, rhs: &$int_type) {
                self.$assign_func(*rhs)
            }
        }

        impl $direct_trait<&$int_type> for BitBoard {
            type Output = Self;

            fn $direct_func(mut self, rhs: &$int_type) -> Self::Output {
                self.$assign_func(rhs);
                self
            }
        }

        impl $direct_trait<$int_type> for BitBoard {
            type Output = Self;

            fn $direct_func(self, rhs: $int_type) -> Self::Output {
                self.$direct_func(&rhs)
            }
        }

        impl $direct_trait<&$int_type> for &BitBoard {
            type Output = BitBoard;

            fn $direct_func(self, rhs: &$int_type) -> Self::Output {
                (*self).$direct_func(rhs)
            }
        }

        impl $direct_trait<$int_type> for &BitBoard {
            type Output = BitBoard;

            fn $direct_func(self, rhs: $int_type) -> Self::Output {
                (*self).$direct_func(rhs)
            }
        }

        impl $assign_trait<&BitBoard> for $int_type {
            fn $assign_func(&mut self, rhs: &BitBoard) {
                self.$assign_func(rhs.get_mask() as $int_type)
            }
        }

        impl $assign_trait<BitBoard> for $int_type {
            fn $assign_func(&mut self, rhs: BitBoard) {
                self.$assign_func(&rhs)
            }
        }

        impl $direct_trait<&BitBoard> for $int_type {
            type Output = $int_type;

            fn $direct_func(mut self, rhs: &BitBoard) -> Self::Output {
                self.$assign_func(rhs);
                self
            }
        }

        impl $direct_trait<BitBoard> for $int_type {
            type Output = $int_type;

            fn $direct_func(self, rhs: BitBoard) -> Self::Output {
                self.$direct_func(&rhs)
            }
        }

        impl $direct_trait<&BitBoard> for &$int_type {
            type Output = $int_type;

            fn $direct_func(self, rhs: &BitBoard) -> Self::Output {
                (*self).$direct_func(rhs)
            }
        }

        impl $direct_trait<BitBoard> for &$int_type {
            type Output = $int_type;

            fn $direct_func(self, rhs: BitBoard) -> Self::Output {
                self.$direct_func(&rhs)
            }
        }
    };
}

implement_bitwise_operations!(BitAnd, BitAndAssign, bitand, bitand_assign);
implement_bitwise_operations!(BitOr, BitOrAssign, bitor, bitor_assign);
implement_bitwise_operations!(BitXor, BitXorAssign, bitxor, bitxor_assign);

impl Mul for BitBoard {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.get_mask().wrapping_mul(rhs.get_mask()))
    }
}

impl Not for &BitBoard {
    type Output = BitBoard;

    #[inline]
    fn not(self) -> BitBoard {
        BitBoard(!self.get_mask())
    }
}

impl Not for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn not(self) -> BitBoard {
        !&self
    }
}

impl Iterator for BitBoard {
    type Item = Square;

    #[inline]
    fn next(&mut self) -> Option<Square> {
        if self.get_mask() == 0 {
            None
        } else {
            let result = self.to_square();
            *self ^= BitBoard::from_square(result);
            Some(result)
        }
    }
}