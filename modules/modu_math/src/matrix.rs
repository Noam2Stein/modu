use crate::{Scalar, Vec3, Vec4};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Mat2<T: Scalar>(Vec4<T>);

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Mat3<T: Scalar> {
    column0: Vec3<T>,
    column1: Vec3<T>,
    column2: Vec3<T>,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Mat4<T: Scalar> {
    column0: Vec4<T>,
    column1: Vec4<T>,
    column2: Vec4<T>,
    column3: Vec4<T>,
}
