pub use modu_ecs_proc_macros::Component;

pub trait Component: Sized + Send + Sync + 'static {}
