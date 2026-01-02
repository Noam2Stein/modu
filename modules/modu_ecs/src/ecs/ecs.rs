use std::{any::type_name, fmt::Debug};

use crate::{Entity, EntityMut, EntityRef, Resource, ecs::Components};

#[derive(Default)]
pub struct Ecs {
    _data: (),
}

impl Ecs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn spawn_empty(&mut self) -> EntityMut<'_> {
        self.spawn(())
    }

    pub fn spawn(&mut self, components: impl Components) -> EntityMut<'_> {
        let _ = components;
        todo!()
    }

    pub fn get_entity(&self, entity: Entity) -> Option<EntityRef<'_>> {
        let _ = entity;
        todo!()
    }

    pub fn get_entity_mut(&mut self, entity: Entity) -> Option<EntityMut<'_>> {
        let _ = entity;
        todo!()
    }

    pub fn entity(&self, entity: Entity) -> EntityRef<'_> {
        let _ = entity;
        todo!()
    }

    pub fn entity_mut(&mut self, entity: Entity) -> EntityMut<'_> {
        let _ = entity;
        todo!()
    }

    pub fn despawn(&mut self, entity: Entity) -> bool {
        let _ = entity;
        todo!()
    }

    pub fn insert_resource<T: Resource>(&mut self, value: T) -> Option<T> {
        let _ = value;
        todo!()
    }

    pub fn get_resource<T: Resource>(&self) -> Option<&T> {
        todo!()
    }

    pub fn get_resource_mut<T: Resource>(&mut self) -> Option<&mut T> {
        todo!()
    }

    pub fn resource<T: Resource>(&self) -> &T {
        self.get_resource::<T>()
            .unwrap_or_else(|| panic!("resource `{}` not found", type_name::<T>()))
    }

    pub fn resource_mut<T: Resource>(&mut self) -> &mut T {
        self.get_resource_mut::<T>()
            .unwrap_or_else(|| panic!("resource `{}` not found", type_name::<T>()))
    }

    pub fn remove_resource<T: Resource>(&mut self) -> Option<T> {
        todo!()
    }
}

impl<'a> Debug for Ecs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f;
        todo!()
    }
}
