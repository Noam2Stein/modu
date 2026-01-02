use std::marker::PhantomData;

use crate::query::{QueryData, QueryFilter};

pub struct QueryIter<'a, Data, Filter> {
    _data: PhantomData<&'a (Data, Filter)>,
}

pub struct QueryIterMut<'a, Data, Filter> {
    _data: PhantomData<&'a (Data, Filter)>,
}

impl<'a, Data, Filter> Iterator for QueryIter<'a, Data, Filter>
where
    Data: QueryData,
    Filter: QueryFilter,
{
    type Item = Data::Ref<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a, Data, Filter> Iterator for QueryIterMut<'a, Data, Filter>
where
    Data: QueryData,
    Filter: QueryFilter,
{
    type Item = Data::Mut<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
