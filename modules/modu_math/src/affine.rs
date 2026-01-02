use crate::{Mat2, Mat3, Mat4, Scalar, Vec2, Vec3, Vec4};

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Affine2<T: Scalar> {
    matrix: Mat2<T>,
    translation: Vec2<T>,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Affine3<T: Scalar> {
    matrix: Mat3<T>,
    translation: Vec3<T>,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Affine4<T: Scalar> {
    matrix: Mat4<T>,
    translation: Vec4<T>,
}
