use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
        SubAssign,
    },
};

use crate::{Bools, Scalar};

pub trait Ints:
    Scalar
    + Debug
    + Clone
    + Copy
    + Hash
    + Default
    + Display
    + Not<Output = Self>
    + Neg<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + Shl<Output = Self>
    + Shr<Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
    + ShlAssign
    + ShrAssign
    + BitAndAssign
    + BitOrAssign
    + BitXorAssign
{
    type Single: Int;

    type Bools: Bools;

    fn select(bools: Self::Bools, if_true: Self, if_false: Self) -> Self;
}

pub trait Int: Ints<Single = Self> + PartialEq + Eq + PartialOrd + Ord {}

macro_rules! impl_int {
    ($T:ident) => {
        impl Ints for $T {
            type Single = Self;

            type Bools = bool;

            #[inline(always)]
            fn select(bools: bool, if_true: $T, if_false: $T) -> $T {
                if bools { if_true } else { if_false }
            }
        }

        impl Int for $T {}
    };
}
impl_int!(i8);
impl_int!(i16);
impl_int!(i32);
impl_int!(i64);
impl_int!(i128);
impl_int!(isize);
