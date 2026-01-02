use std::{
    fmt::{Debug, Pointer},
    ops::{Deref, DerefMut},
};

use crate::{Resource, system::SystemParam};

pub struct Res<T>(T);

impl<'a, T: Resource> Debug for Res<&'a T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<'a, T: Resource> Debug for Res<&'a mut T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<'a, T: Resource> Deref for Res<&'a T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T: Resource> Deref for Res<&'a mut T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T: Resource> DerefMut for Res<&'a mut T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Resource> SystemParam for Res<&'static T> {
    type WithLifetime<'a> = Res<&'a T>;
}

impl<T: Resource> SystemParam for Res<&'static mut T> {
    type WithLifetime<'a> = Res<&'a mut T>;
}
