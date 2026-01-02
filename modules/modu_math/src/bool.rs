use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

use crate::Scalar;

pub trait Bools:
    Scalar
    + Debug
    + Clone
    + Copy
    + Hash
    + Default
    + Display
    + Not<Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + BitAndAssign
    + BitOrAssign
    + BitXorAssign
{
    fn select(bools: Self, if_true: Self, if_false: Self) -> Self;
}

impl Bools for bool {
    fn select(bools: Self, if_true: Self, if_false: Self) -> Self {
        if bools { if_true } else { if_false }
    }
}
