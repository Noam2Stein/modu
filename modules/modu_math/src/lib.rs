#![expect(private_bounds)]

mod affine;
mod bool;
mod float;
mod int;
mod mask;
mod matrix;
mod quat;
mod scalar;
mod uint;
mod vector;
pub use affine::*;
pub use bool::*;
pub use float::*;
pub use int::*;
pub use mask::*;
pub use matrix::*;
pub use quat::*;
pub use scalar::*;
pub use uint::*;
pub use vector::*;

mod impls;
