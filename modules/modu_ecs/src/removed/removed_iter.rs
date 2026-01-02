use std::marker::PhantomData;

use crate::Entity;

pub struct RemovedIter<'a> {
    _data: PhantomData<&'a ()>,
}

impl<'a> Iterator for RemovedIter<'a> {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
