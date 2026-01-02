use std::{
    hash::Hash,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref,
        DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl,
        ShlAssign, Shr, ShrAssign, Sub, SubAssign,
    },
};

use crate::{Floats, FloatsBackend, Scalar, ScalarBackend};

impl<T: Scalar> VecN<T> {
    #[inline(always)]
    pub const fn from_array(array: [T; N]) -> Self {
        if N == 3 && size_of::<Self>() == size_of::<T>() * 4 {
            unsafe {
                std::mem::transmute_copy::<[T; 4], VecN<T>>(&[
                    array[0], array[1], array[2], array[2],
                ])
            }
        } else {
            unsafe { std::mem::transmute_copy::<[T; N], VecN<T>>(&array) }
        }
    }

    #[inline(always)]
    pub const fn to_array(self) -> [T; N] {
        *self.as_array_ref()
    }

    #[inline(always)]
    pub const fn as_array_ref(&self) -> &[T; N] {
        unsafe { std::mem::transmute::<&VecN<T>, &[T; N]>(self) }
    }

    #[inline(always)]
    pub const fn as_array_mut(&mut self) -> &mut [T; N] {
        unsafe { std::mem::transmute::<&mut VecN<T>, &mut [T; N]>(self) }
    }
}

impl<T: Scalar + PartialEq> PartialEq for VecN<T> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        <T as ScalarBackend<N>>::vec_eq(&self.0, &other.0)
    }

    #[inline(always)]
    fn ne(&self, other: &Self) -> bool {
        <T as ScalarBackend<N>>::vec_ne(&self.0, &other.0)
    }
}

impl<T: Scalar + Eq> Eq for VecN<T> {}

impl<T: Scalar + Hash> Hash for VecN<T> {
    #[inline(always)]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_array_ref().hash(state);
    }
}

impl<T: Scalar + Default> Default for VecN<T> {
    #[inline(always)]
    fn default() -> Self {
        Self::from_array([T::default(); N])
    }
}

impl<T: Scalar> Index<usize> for VecN<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_array_ref()[index]
    }
}

impl<T: Scalar> IndexMut<usize> for VecN<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_array_mut()[index]
    }
}

impl<T: Scalar> Deref for VecN<T> {
    type Target = ComponentsN<T>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute::<&VecN<T>, &ComponentsN<T>>(self) }
    }
}

impl<T: Scalar> DerefMut for VecN<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute::<&mut VecN<T>, &mut ComponentsN<T>>(self) }
    }
}

impl<T: Scalar + Neg<Output = T>> Neg for VecN<T> {
    type Output = VecN<T>;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self(<T as ScalarBackend<N>>::vec_neg(self.0))
    }
}

impl<T: Scalar + Not<Output = T>> Not for VecN<T> {
    type Output = VecN<T>;

    #[inline(always)]
    fn not(self) -> Self::Output {
        Self(<T as ScalarBackend<N>>::vec_not(self.0))
    }
}

impl<T: Scalar + Add<Output = T>> Add for VecN<T> {
    type Output = VecN<T>;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self(<T as ScalarBackend<N>>::vec_add(self.0, rhs.0))
    }
}

impl<T: Scalar + Sub<Output = T>> Sub for VecN<T> {
    type Output = VecN<T>;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(<T as ScalarBackend<N>>::vec_sub(self.0, rhs.0))
    }
}

impl<T: Scalar + Mul<Output = T>> Mul for VecN<T> {
    type Output = VecN<T>;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        Self(<T as ScalarBackend<N>>::vec_mul(self.0, rhs.0))
    }
}

impl<T: Scalar + Div<Output = T>> Div for VecN<T> {
    type Output = VecN<T>;

    #[inline(always)]
    fn div(self, rhs: Self) -> Self::Output {
        Self(<T as ScalarBackend<N>>::vec_div(self.0, rhs.0))
    }
}

impl<T: Scalar + Rem<Output = T>> Rem for VecN<T> {
    type Output = VecN<T>;

    #[inline(always)]
    fn rem(self, rhs: Self) -> Self::Output {
        Self(<T as ScalarBackend<N>>::vec_rem(self.0, rhs.0))
    }
}

impl<T: Scalar + Shl<Output = T>> Shl for VecN<T> {
    type Output = VecN<T>;

    #[inline(always)]
    fn shl(self, rhs: Self) -> Self::Output {
        Self(<T as ScalarBackend<N>>::vec_shl(self.0, rhs.0))
    }
}

impl<T: Scalar + Shr<Output = T>> Shr for VecN<T> {
    type Output = VecN<T>;

    #[inline(always)]
    fn shr(self, rhs: Self) -> Self::Output {
        Self(<T as ScalarBackend<N>>::vec_shr(self.0, rhs.0))
    }
}

impl<T: Scalar + BitAnd<Output = T>> BitAnd for VecN<T> {
    type Output = VecN<T>;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(<T as ScalarBackend<N>>::vec_bitand(self.0, rhs.0))
    }
}

impl<T: Scalar + BitOr<Output = T>> BitOr for VecN<T> {
    type Output = VecN<T>;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(<T as ScalarBackend<N>>::vec_bitor(self.0, rhs.0))
    }
}

impl<T: Scalar + BitXor<Output = T>> BitXor for VecN<T> {
    type Output = VecN<T>;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(<T as ScalarBackend<N>>::vec_bitxor(self.0, rhs.0))
    }
}

impl<T: Scalar + Add<Output = T>> AddAssign for VecN<T> {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        self.0 = <T as ScalarBackend<N>>::vec_add(self.0, rhs.0);
    }
}

impl<T: Scalar + Sub<Output = T>> SubAssign for VecN<T> {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = <T as ScalarBackend<N>>::vec_sub(self.0, rhs.0);
    }
}

impl<T: Scalar + Mul<Output = T>> MulAssign for VecN<T> {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = <T as ScalarBackend<N>>::vec_mul(self.0, rhs.0);
    }
}

impl<T: Scalar + Div<Output = T>> DivAssign for VecN<T> {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        self.0 = <T as ScalarBackend<N>>::vec_div(self.0, rhs.0);
    }
}

impl<T: Scalar + Rem<Output = T>> RemAssign for VecN<T> {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Self) {
        self.0 = <T as ScalarBackend<N>>::vec_rem(self.0, rhs.0);
    }
}

impl<T: Scalar + Shl<Output = T>> ShlAssign for VecN<T> {
    #[inline(always)]
    fn shl_assign(&mut self, rhs: Self) {
        self.0 = <T as ScalarBackend<N>>::vec_shl(self.0, rhs.0);
    }
}

impl<T: Scalar + Shr<Output = T>> ShrAssign for VecN<T> {
    #[inline(always)]
    fn shr_assign(&mut self, rhs: Self) {
        self.0 = <T as ScalarBackend<N>>::vec_shr(self.0, rhs.0);
    }
}

impl<T: Scalar + BitAnd<Output = T>> BitAndAssign for VecN<T> {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 = <T as ScalarBackend<N>>::vec_bitand(self.0, rhs.0);
    }
}

impl<T: Scalar + BitOr<Output = T>> BitOrAssign for VecN<T> {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 = <T as ScalarBackend<N>>::vec_bitor(self.0, rhs.0);
    }
}

impl<T: Scalar + BitXor<Output = T>> BitXorAssign for VecN<T> {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 = <T as ScalarBackend<N>>::vec_bitxor(self.0, rhs.0);
    }
}

////////////////////////////////////////////////////////////////////////////////
// Floats
////////////////////////////////////////////////////////////////////////////////

impl<T: Floats> VecN<T> {
    #[inline(always)]
    pub fn abs(self) -> Self {
        Self(<T as FloatsBackend<N>>::vec_abs(self.0))
    }
}
