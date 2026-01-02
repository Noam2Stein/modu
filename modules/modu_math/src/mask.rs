use crate::{Scalar, ScalarBackend};

#[derive(Clone, Copy)]
pub struct Mask2<T: Scalar>(<T as ScalarBackend<2>>::Vec);

#[derive(Clone, Copy)]
pub struct Mask3<T: Scalar>(<T as ScalarBackend<3>>::Vec);

#[derive(Clone, Copy)]
pub struct Mask4<T: Scalar>(<T as ScalarBackend<4>>::Vec);
