#[cfg(feature = "gpu")]
pub mod gpu {
    pub use modu_gpu::*;
}

#[cfg(feature = "math")]
pub mod math {
    pub use modu_math::*;
}

#[cfg(feature = "window")]
pub mod window {
    pub use modu_window::*;
}
