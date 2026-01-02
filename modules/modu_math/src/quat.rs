use crate::{Scalar, Vec4};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Quat<T: Scalar>(Vec4<T>);
