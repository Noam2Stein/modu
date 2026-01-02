use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
    num::FpCategory,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

use crate::{Bools, Scalar, ScalarBackend, Uint, Uints};

pub trait Floats:
    Scalar
    + FloatsBackend<2>
    + FloatsBackend<3>
    + FloatsBackend<4>
    + Debug
    + Clone
    + Copy
    + Default
    + Display
    + Neg<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
{
    type Single: Float;

    type Bools: Bools;
    type Uints: Uints;
    type FpCategories: Debug + Clone + Copy;
    type Orderings: Debug + Clone + Copy;

    const RADIX: u32;
    const MANTISSA_DIGITS: u32;
    const DIGITS: u32;
    const EPSILON: Self;
    const MIN: Self;
    const MIN_POSITIVE: Self;
    const MAX: Self;
    const MIN_EXP: i32;
    const MAX_EXP: i32;
    const MIN_10_EXP: i32;
    const MAX_10_EXP: i32;
    const NAN: Self;
    const INFINITY: Self;
    const NEG_INFINITY: Self;

    fn is_nan(self) -> Self::Bools;
    fn is_infinite(self) -> Self::Bools;
    fn is_finite(self) -> Self::Bools;
    fn is_subnormal(self) -> Self::Bools;
    fn is_normal(self) -> Self::Bools;
    fn classify(self) -> Self::FpCategories;
    fn is_sign_positive(self) -> Self::Bools;
    fn is_sign_negative(self) -> Self::Bools;
    fn next_up(self) -> Self;
    fn next_down(self) -> Self;
    fn recip(self) -> Self;
    fn to_degrees(self) -> Self;
    fn to_radians(self) -> Self;
    fn max(self, other: Self) -> Self;
    fn min(self, other: Self) -> Self;
    fn midpoint(self, other: Self) -> Self;
    fn to_bits(self) -> Self::Uints;
    fn from_bits(v: Self::Uints) -> Self;
    fn total_cmp(&self, other: &Self) -> Self::Orderings;
    fn clamp(self, min: Self, max: Self) -> Self;
    fn abs(self) -> Self;
    fn signum(self) -> Self;
    fn copysign(self, sign: Self) -> Self;

    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn round(self) -> Self;
    fn round_ties_even(self) -> Self;
    fn trunc(self) -> Self;
    fn fract(self) -> Self;
    fn mul_add(self, a: Self, b: Self) -> Self;
    fn div_euclid(self, rhs: Self) -> Self;
    fn rem_euclid(self, rhs: Self) -> Self;
    fn powf(self, n: Self) -> Self;
    fn sqrt(self) -> Self;
    fn exp(self) -> Self;
    fn exp2(self) -> Self;
    fn ln(self) -> Self;
    fn log(self, base: Self) -> Self;
    fn log2(self) -> Self;
    fn log10(self) -> Self;
    fn cbrt(self) -> Self;
    fn hypot(self, other: Self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn asin(self) -> Self;
    fn acos(self) -> Self;
    fn atan(self) -> Self;
    fn atan2(self, other: Self) -> Self;
    fn sin_cos(self) -> (Self, Self);
    fn exp_m1(self) -> Self;
    fn ln_1p(self) -> Self;
    fn sinh(self) -> Self;
    fn cosh(self) -> Self;
    fn tanh(self) -> Self;
    fn asinh(self) -> Self;
    fn acosh(self) -> Self;
    fn atanh(self) -> Self;

    fn select(bools: Self::Bools, if_true: Self, if_false: Self) -> Self;
}

pub trait Float:
    Floats<
        Single = Self,
        Bools = bool,
        Uints: Uint,
        FpCategories = FpCategory,
        Orderings = Ordering,
    > + PartialEq
    + PartialOrd
{
    fn powi(self, n: i32) -> Self;
}

pub(crate) trait FloatsBackend<const N: usize>: ScalarBackend<N> {
    fn vec_abs(vec: Self::Vec) -> Self::Vec;
}

macro_rules! impl_float {
    ($T:ident, Uint = $Uint:ident) => {
        impl Floats for $T {
            type Single = Self;

            type Bools = bool;
            type FpCategories = FpCategory;
            type Uints = $Uint;
            type Orderings = Ordering;

            const RADIX: u32 = Self::RADIX;
            const MANTISSA_DIGITS: u32 = Self::MANTISSA_DIGITS;
            const DIGITS: u32 = Self::DIGITS;
            const EPSILON: Self = Self::EPSILON;
            const MIN: Self = Self::MIN;
            const MIN_POSITIVE: Self = Self::MIN_POSITIVE;
            const MAX: Self = Self::MAX;
            const MIN_EXP: i32 = Self::MIN_EXP;
            const MAX_EXP: i32 = Self::MAX_EXP;
            const MIN_10_EXP: i32 = Self::MIN_10_EXP;
            const MAX_10_EXP: i32 = Self::MAX_10_EXP;
            const NAN: Self = Self::NAN;
            const INFINITY: Self = Self::INFINITY;
            const NEG_INFINITY: Self = Self::NEG_INFINITY;

            #[inline(always)]
            fn is_nan(self) -> bool {
                self.is_nan()
            }

            #[inline(always)]
            fn is_infinite(self) -> bool {
                self.is_infinite()
            }

            #[inline(always)]
            fn is_finite(self) -> bool {
                self.is_finite()
            }

            #[inline(always)]
            fn is_subnormal(self) -> bool {
                self.is_subnormal()
            }

            #[inline(always)]
            fn is_normal(self) -> bool {
                self.is_normal()
            }

            #[inline(always)]
            fn classify(self) -> FpCategory {
                self.classify()
            }

            #[inline(always)]
            fn is_sign_positive(self) -> bool {
                self.is_sign_positive()
            }

            #[inline(always)]
            fn is_sign_negative(self) -> bool {
                self.is_sign_negative()
            }

            #[inline(always)]
            fn next_up(self) -> Self {
                self.next_up()
            }

            #[inline(always)]
            fn next_down(self) -> Self {
                self.next_down()
            }

            #[inline(always)]
            fn recip(self) -> Self {
                self.recip()
            }

            #[inline(always)]
            fn to_degrees(self) -> Self {
                self.to_degrees()
            }

            #[inline(always)]
            fn to_radians(self) -> Self {
                self.to_radians()
            }

            #[inline(always)]
            fn max(self, other: Self) -> Self {
                self.max(other)
            }

            #[inline(always)]
            fn min(self, other: Self) -> Self {
                self.min(other)
            }

            #[inline(always)]
            fn midpoint(self, other: Self) -> Self {
                self.midpoint(other)
            }

            #[inline(always)]
            fn to_bits(self) -> Self::Uints {
                self.to_bits()
            }

            #[inline(always)]
            fn from_bits(bits: Self::Uints) -> Self {
                Self::from_bits(bits)
            }

            #[inline(always)]
            fn total_cmp(&self, other: &Self) -> Ordering {
                self.total_cmp(other)
            }

            #[inline(always)]
            fn clamp(self, min: Self, max: Self) -> Self {
                self.clamp(min, max)
            }

            #[inline(always)]
            fn abs(self) -> Self {
                self.abs()
            }

            #[inline(always)]
            fn signum(self) -> Self {
                self.signum()
            }

            #[inline(always)]
            fn copysign(self, sign: Self) -> Self {
                self.copysign(sign)
            }

            #[inline(always)]
            fn floor(self) -> Self {
                self.floor()
            }

            #[inline(always)]
            fn ceil(self) -> Self {
                self.ceil()
            }

            #[inline(always)]
            fn round(self) -> Self {
                self.round()
            }

            #[inline(always)]
            fn round_ties_even(self) -> Self {
                self.round_ties_even()
            }

            #[inline(always)]
            fn trunc(self) -> Self {
                self.trunc()
            }

            #[inline(always)]
            fn fract(self) -> Self {
                self.fract()
            }

            #[inline(always)]
            fn mul_add(self, a: Self, b: Self) -> Self {
                self.mul_add(a, b)
            }

            #[inline(always)]
            fn div_euclid(self, other: Self) -> Self {
                self.div_euclid(other)
            }

            #[inline(always)]
            fn rem_euclid(self, other: Self) -> Self {
                self.rem_euclid(other)
            }

            #[inline(always)]
            fn powf(self, other: Self) -> Self {
                self.powf(other)
            }

            #[inline(always)]
            fn sqrt(self) -> Self {
                self.sqrt()
            }

            #[inline(always)]
            fn exp(self) -> Self {
                self.exp()
            }

            #[inline(always)]
            fn exp2(self) -> Self {
                self.exp2()
            }

            #[inline(always)]
            fn ln(self) -> Self {
                self.ln()
            }

            #[inline(always)]
            fn log(self, base: Self) -> Self {
                self.log(base)
            }

            #[inline(always)]
            fn log2(self) -> Self {
                self.log2()
            }

            #[inline(always)]
            fn log10(self) -> Self {
                self.log10()
            }

            #[inline(always)]
            fn cbrt(self) -> Self {
                self.cbrt()
            }

            #[inline(always)]
            fn hypot(self, other: Self) -> Self {
                self.hypot(other)
            }

            #[inline(always)]
            fn sin(self) -> Self {
                self.sin()
            }

            #[inline(always)]
            fn cos(self) -> Self {
                self.cos()
            }

            #[inline(always)]
            fn tan(self) -> Self {
                self.tan()
            }

            #[inline(always)]
            fn asin(self) -> Self {
                self.asin()
            }

            #[inline(always)]
            fn acos(self) -> Self {
                self.acos()
            }

            #[inline(always)]
            fn atan(self) -> Self {
                self.atan()
            }

            #[inline(always)]
            fn atan2(self, other: Self) -> Self {
                self.atan2(other)
            }

            #[inline(always)]
            fn sin_cos(self) -> (Self, Self) {
                self.sin_cos()
            }

            #[inline(always)]
            fn exp_m1(self) -> Self {
                self.exp_m1()
            }

            #[inline(always)]
            fn ln_1p(self) -> Self {
                self.ln_1p()
            }

            #[inline(always)]
            fn sinh(self) -> Self {
                self.sinh()
            }

            #[inline(always)]
            fn cosh(self) -> Self {
                self.cosh()
            }

            #[inline(always)]
            fn tanh(self) -> Self {
                self.tanh()
            }

            #[inline(always)]
            fn asinh(self) -> Self {
                self.asinh()
            }

            #[inline(always)]
            fn acosh(self) -> Self {
                self.acosh()
            }

            #[inline(always)]
            fn atanh(self) -> Self {
                self.atanh()
            }

            #[inline(always)]
            fn select(bools: bool, if_true: Self, if_false: Self) -> Self {
                if bools { if_true } else { if_false }
            }
        }

        impl Float for $T {
            fn powi(self, n: i32) -> Self {
                self.powi(n)
            }
        }
    };
}
impl_float!(f32, Uint = u32);
impl_float!(f64, Uint = u64);
