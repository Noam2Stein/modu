pub trait Scalar: ScalarBackend<2> + ScalarBackend<3> + ScalarBackend<4> {}

pub(crate) trait ScalarBackend<const N: usize>: Copy {
    type Vec: Copy;

    fn vec_eq(vec: &Self::Vec, other: &Self::Vec) -> bool;
    fn vec_ne(vec: &Self::Vec, other: &Self::Vec) -> bool;

    fn vec_neg(vec: Self::Vec) -> Self::Vec;
    fn vec_not(vec: Self::Vec) -> Self::Vec;

    fn vec_add(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec;
    fn vec_sub(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec;
    fn vec_mul(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec;
    fn vec_div(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec;
    fn vec_rem(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec;
    fn vec_shl(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec;
    fn vec_shr(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec;
    fn vec_bitand(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec;
    fn vec_bitor(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec;
    fn vec_bitxor(vec: Self::Vec, rhs: Self::Vec) -> Self::Vec;
}
