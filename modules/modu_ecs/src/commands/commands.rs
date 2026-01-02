use std::{fmt::Debug, marker::PhantomData};

use crate::{Entity, Resource, advanced::EntityCommands, ecs::Components, system::SystemParam};

pub struct Commands<'a> {
    _data: PhantomData<&'a ()>,
}

impl<'a> Commands<'a> {
    pub fn spawn_empty(&mut self) -> EntityCommands<'_> {
        todo!()
    }

    pub fn spawn(&mut self, components: impl Components) -> EntityCommands<'_> {
        let _ = components;
        todo!()
    }

    pub fn entity(&mut self, entity: Entity) -> EntityCommands<'_> {
        let _ = entity;
        todo!()
    }

    pub fn insert_resource<T: Resource>(&mut self, value: T) {
        let _ = value;
        todo!()
    }

    pub fn remove_resource<T: Resource>(&mut self) {
        todo!()
    }
}

impl<'a> Debug for Commands<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f;
        todo!()
    }
}

impl SystemParam for Commands<'static> {
    type WithLifetime<'a> = Commands<'a>;
}
