use std::{any::type_name, fmt::Debug, marker::PhantomData};

use crate::{Component, ecs::Components};

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity(u64);

#[derive(Clone, Copy)]
pub struct EntityRef<'a> {
    _data: PhantomData<&'a ()>,
}

pub struct EntityMut<'a> {
    _data: PhantomData<&'a ()>,
}

pub struct EntityPartialMut<'a> {
    _data: PhantomData<&'a ()>,
}

impl<'a> EntityRef<'a> {
    pub fn id(&self) -> Entity {
        todo!()
    }

    pub fn get<T: Component>(&self) -> Option<&T> {
        todo!()
    }

    pub fn component<T: Component>(&self) -> &T {
        self.get()
            .unwrap_or_else(|| panic!("component `{}` not found", type_name::<T>()))
    }
}

impl<'a> EntityMut<'a> {
    pub fn id(&self) -> Entity {
        todo!()
    }

    pub fn as_ref(&self) -> EntityRef<'_> {
        todo!()
    }

    pub fn get<T: Component>(&self) -> Option<&T> {
        todo!()
    }

    pub fn get_mut<T: Component>(&mut self) -> Option<&mut T> {
        todo!()
    }

    pub fn component<T: Component>(&self) -> &T {
        self.get()
            .unwrap_or_else(|| panic!("component `{}` not found", type_name::<T>()))
    }

    pub fn component_mut<T: Component>(&mut self) -> &mut T {
        self.get_mut()
            .unwrap_or_else(|| panic!("component `{}` not found", type_name::<T>()))
    }

    pub fn insert<T: Components>(&mut self, components: T) {
        let _ = components;
        todo!()
    }

    pub fn remove<T: Components>(&mut self) -> Option<T> {
        todo!()
    }
}

impl<'a> EntityPartialMut<'a> {
    pub fn id(&self) -> Entity {
        todo!()
    }

    pub fn as_ref(&self) -> EntityRef<'_> {
        todo!()
    }

    pub fn get<T: Component>(&self) -> Option<&T> {
        todo!()
    }

    pub fn get_mut<T: Component>(&mut self) -> Option<&mut T> {
        todo!()
    }

    pub fn component<T: Component>(&self) -> &T {
        self.get()
            .unwrap_or_else(|| panic!("component `{}` not found", type_name::<T>()))
    }

    pub fn component_mut<T: Component>(&mut self) -> &mut T {
        self.get_mut()
            .unwrap_or_else(|| panic!("component `{}` not found", type_name::<T>()))
    }
}

impl<'a> Debug for EntityRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f;
        todo!()
    }
}

impl<'a> Debug for EntityMut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f;
        todo!()
    }
}

impl<'a> Debug for EntityPartialMut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f;
        todo!()
    }
}
