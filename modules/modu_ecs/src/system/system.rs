use std::marker::PhantomData;

pub use modu_ecs_proc_macros::system;

#[doc(hidden)]
pub struct System<F, Params> {
    pub f: F,
    pub marker: PhantomData<Params>,
}

impl<F: Clone, Marker> Clone for System<F, Marker> {
    fn clone(&self) -> Self {
        Self {
            f: self.f.clone(),
            marker: PhantomData,
        }
    }
}

impl<F: Copy, Marker> Copy for System<F, Marker> {}
