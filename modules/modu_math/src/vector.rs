use std::fmt::Debug;

use crate::{Scalar, ScalarBackend};

#[derive(Clone, Copy)]
pub struct Vec2<T: Scalar>(<T as ScalarBackend<2>>::Vec);

#[derive(Clone, Copy)]
pub struct Vec3<T: Scalar>(<T as ScalarBackend<3>>::Vec);

#[derive(Clone, Copy)]
pub struct Vec4<T: Scalar>(<T as ScalarBackend<4>>::Vec);

impl<T: Scalar + Debug> Debug for Vec2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}

impl<T: Scalar + Debug> Debug for Vec3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?}, {:?})", self.x, self.y, self.z)
    }
}

impl<T: Scalar + Debug> Debug for Vec4<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({:?}, {:?}, {:?}, {:?})",
            self.x, self.y, self.z, self.w
        )
    }
}

////////////////////////////////////////////////////////////////////////////////
// Constructor
////////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! vec2 {
    ($($arg:expr),*$(,)?) => {
        $crate::Vec2::from(($($arg,)*))
    };
}

#[macro_export]
macro_rules! vec3 {
    ($($arg:expr),*$(,)?) => {
        $crate::Vec3::from(($($arg,)*))
    };
}

#[macro_export]
macro_rules! vec4 {
    ($($arg:expr),*$(,)?) => {
        $crate::Vec4::from(($($arg,)*))
    };
}

impl<T: Scalar> From<(T,)> for Vec2<T> {
    #[inline(always)]
    fn from(value: (T,)) -> Self {
        Self::from_array([value.0, value.0])
    }
}

impl<T: Scalar> From<(T, T)> for Vec2<T> {
    #[inline(always)]
    fn from(value: (T, T)) -> Self {
        Self::from_array([value.0, value.1])
    }
}

impl<T: Scalar> From<(Vec2<T>,)> for Vec2<T> {
    #[inline(always)]
    fn from(value: (Vec2<T>,)) -> Self {
        value.0
    }
}

impl<T: Scalar> From<(T,)> for Vec3<T> {
    #[inline(always)]
    fn from(value: (T,)) -> Self {
        Self::from_array([value.0, value.0, value.0])
    }
}

impl<T: Scalar> From<(T, T, T)> for Vec3<T> {
    #[inline(always)]
    fn from(value: (T, T, T)) -> Self {
        Self::from_array([value.0, value.1, value.2])
    }
}

impl<T: Scalar> From<(T, Vec2<T>)> for Vec3<T> {
    #[inline(always)]
    fn from(value: (T, Vec2<T>)) -> Self {
        Self::from_array([value.0, value.1.x, value.1.y])
    }
}

impl<T: Scalar> From<(Vec2<T>, T)> for Vec3<T> {
    #[inline(always)]
    fn from(value: (Vec2<T>, T)) -> Self {
        Self::from_array([value.0.x, value.0.y, value.1])
    }
}

impl<T: Scalar> From<(Vec3<T>,)> for Vec3<T> {
    #[inline(always)]
    fn from(value: (Vec3<T>,)) -> Self {
        value.0
    }
}

impl<T: Scalar> From<(T,)> for Vec4<T> {
    #[inline(always)]
    fn from(value: (T,)) -> Self {
        Self::from_array([value.0, value.0, value.0, value.0])
    }
}

impl<T: Scalar> From<(T, T, T, T)> for Vec4<T> {
    #[inline(always)]
    fn from(value: (T, T, T, T)) -> Self {
        Self::from_array([value.0, value.1, value.2, value.3])
    }
}

impl<T: Scalar> From<(T, T, Vec2<T>)> for Vec4<T> {
    #[inline(always)]
    fn from(value: (T, T, Vec2<T>)) -> Self {
        Self::from_array([value.0, value.1, value.2.x, value.2.y])
    }
}

impl<T: Scalar> From<(T, Vec2<T>, T)> for Vec4<T> {
    #[inline(always)]
    fn from(value: (T, Vec2<T>, T)) -> Self {
        Self::from_array([value.0, value.1.x, value.1.y, value.2])
    }
}

impl<T: Scalar> From<(T, Vec3<T>)> for Vec4<T> {
    #[inline(always)]
    fn from(value: (T, Vec3<T>)) -> Self {
        Self::from_array([value.0, value.1.x, value.1.y, value.1.z])
    }
}

impl<T: Scalar> From<(Vec2<T>, T, T)> for Vec4<T> {
    #[inline(always)]
    fn from(value: (Vec2<T>, T, T)) -> Self {
        Self::from_array([value.0.x, value.0.y, value.1, value.2])
    }
}

impl<T: Scalar> From<(Vec2<T>, Vec2<T>)> for Vec4<T> {
    #[inline(always)]
    fn from(value: (Vec2<T>, Vec2<T>)) -> Self {
        Self::from_array([value.0.x, value.0.y, value.1.x, value.1.y])
    }
}

impl<T: Scalar> From<(Vec3<T>, T)> for Vec4<T> {
    #[inline(always)]
    fn from(value: (Vec3<T>, T)) -> Self {
        Self::from_array([value.0.x, value.0.y, value.0.z, value.1])
    }
}

impl<T: Scalar> From<(Vec4<T>,)> for Vec4<T> {
    #[inline(always)]
    fn from(value: (Vec4<T>,)) -> Self {
        value.0
    }
}

////////////////////////////////////////////////////////////////////////////////
// Common
////////////////////////////////////////////////////////////////////////////////

mod common2 {
    type VecN<T> = crate::Vec2<T>;

    const N: usize = 2;

    #[repr(C)]
    pub struct ComponentsN<T> {
        pub x: T,
        pub y: T,
    }

    include!("vector_common.rs");
}

mod common3 {
    type VecN<T> = crate::Vec3<T>;

    const N: usize = 3;

    #[repr(C)]
    pub struct ComponentsN<T> {
        pub x: T,
        pub y: T,
        pub z: T,
    }

    include!("vector_common.rs");
}

mod common4 {
    type VecN<T> = crate::Vec4<T>;

    const N: usize = 4;

    #[repr(C)]
    pub struct ComponentsN<T> {
        pub x: T,
        pub y: T,
        pub z: T,
        pub w: T,
    }

    include!("vector_common.rs");
}
