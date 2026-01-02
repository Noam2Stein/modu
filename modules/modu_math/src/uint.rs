use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
        SubAssign,
    },
};

use crate::{Bools, Scalar};

pub trait Uints:
    Scalar
    + Debug
    + Clone
    + Copy
    + Hash
    + Default
    + Display
    + Not<Output = Self>
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
    type Single: Uint;

    type Bools: Bools;

    fn select(bools: Self::Bools, if_true: Self, if_false: Self) -> Self;
}

pub trait Uint: Uints<Single = Self> + PartialEq + Eq + PartialOrd + Ord {}

macro_rules! impl_uint {
    ($T:ident) => {
        impl Uints for $T {
            type Single = Self;

            type Bools = bool;

            #[inline(always)]
            fn select(bools: bool, if_true: $T, if_false: $T) -> $T {
                if bools { if_true } else { if_false }
            }
        }

        impl Uint for $T {}
    };
}
impl_uint!(u8);
impl_uint!(u16);
impl_uint!(u32);
impl_uint!(u64);
impl_uint!(u128);
impl_uint!(usize);
