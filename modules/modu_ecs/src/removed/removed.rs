use std::{fmt::Debug, marker::PhantomData};

use crate::{Component, Entity, removed::RemovedIter, system::SystemParam};

pub struct Removed<'a, T> {
    _data: PhantomData<&'a T>,
}

impl<'a, T> Removed<'a, T> {
    pub fn iter(&self) -> RemovedIter<'_> {
        todo!()
    }
}

impl<'a, T> Debug for Removed<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f;
        todo!()
    }
}

impl<'a, T> Clone for Removed<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T> Copy for Removed<'a, T> {}

impl<'a, T> IntoIterator for Removed<'a, T> {
    type IntoIter = RemovedIter<'a>;
    type Item = Entity;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

impl<T: Component> SystemParam for Removed<'static, T> {
    type WithLifetime<'a> = Removed<'a, T>;
}

impl SystemParam for Removed<'static, Entity> {
    type WithLifetime<'a> = Removed<'a, Entity>;
}
