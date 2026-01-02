#[cfg(feature = "ecs")]
pub mod ecs {
    pub use modu_ecs::*;
}

#[cfg(feature = "math")]
pub mod math {
    pub use modu_math::*;
}
