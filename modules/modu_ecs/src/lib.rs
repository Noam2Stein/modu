mod commands;
mod ecs;
mod param;
mod query;
mod removed;
mod res;
mod system;
pub use commands::Commands;
pub use ecs::{Component, Ecs, Entity, EntityMut, EntityRef, Resource};
pub use param::Param;
pub use query::{Added, Query, With, Without};
pub use removed::Removed;
pub use res::Res;
pub use system::{Parameter, Params, Schedule, Systems, system};

pub mod advanced {
    use crate::*;

    pub use commands::EntityCommands;
    pub use ecs::EntityPartialMut;
    pub use query::{QueryIter, QueryIterMut};
    pub use removed::RemovedIter;
}

#[doc(hidden)]
pub mod hidden {
    use crate::*;

    #[doc(hidden)]
    pub use system::System;
}

mod variadic;
