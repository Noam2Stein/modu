//! A modular game engine.

#![forbid(missing_docs)]

#[cfg(feature = "gpu")]
pub use modu_gpu as gpu;

#[cfg(feature = "math")]
pub use modu_math as math;

#[cfg(feature = "window")]
pub use modu_window as window;
