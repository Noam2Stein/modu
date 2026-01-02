use std::ops::{Deref, DerefMut};

use crate::{Parameter, system::SystemParam};

pub struct Param<T>(T);

impl<'a, T: Parameter> Deref for Param<&'a T>
where
    T: ?Sized,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T: Parameter> Deref for Param<&'a mut T>
where
    T: ?Sized,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T: Parameter> DerefMut for Param<&'a mut T>
where
    T: ?Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Parameter> SystemParam for Param<&'static T>
where
    T: ?Sized,
{
    type WithLifetime<'a> = Param<&'a T>;
}

impl<T: Parameter> SystemParam for Param<&'static mut T>
where
    T: ?Sized,
{
    type WithLifetime<'a> = Param<&'a mut T>;
}
