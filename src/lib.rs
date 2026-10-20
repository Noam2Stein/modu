//! A modular game engine.

#[cfg(feature = "app")]
#[doc(inline)]
pub use modu_app as app;

#[cfg(feature = "gpu")]
#[doc(inline)]
pub use modu_gpu as gpu;

#[cfg(feature = "math")]
#[doc(inline)]
pub use modu_math as math;

#[cfg(feature = "render")]
#[doc(inline)]
pub use modu_render as render;

#[cfg(feature = "window")]
#[doc(inline)]
pub use modu_window as window;
