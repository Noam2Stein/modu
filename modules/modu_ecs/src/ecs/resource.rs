pub use modu_ecs_proc_macros::Resource;

pub trait Resource: Sized + Send + Sync + 'static {}
