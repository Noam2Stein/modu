use std::{fmt::Debug, marker::PhantomData};

use crate::ecs::Components;

pub struct EntityCommands<'a> {
    _data: PhantomData<&'a ()>,
}

impl<'a> EntityCommands<'a> {
    pub fn insert<T: Components>(&mut self, components: T) {
        let _ = components;
        todo!()
    }

    pub fn remove<T: Components>(&mut self) {
        todo!()
    }

    pub fn despawn(self) {
        todo!()
    }
}

impl<'a> Debug for EntityCommands<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f;
        todo!()
    }
}
