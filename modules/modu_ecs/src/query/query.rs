use std::marker::PhantomData;

use crate::{
    query::{QueryData, QueryFilter, QueryIter, QueryIterMut},
    system::SystemParam,
};

pub struct Query<'a, Data, Filter = ()> {
    _data: PhantomData<&'a (Data, Filter)>,
}

pub struct With<T>(T);
pub struct Without<T>(T);
pub struct Added<T>(T);

impl<'a, Data, Filter> Query<'a, Data, Filter>
where
    Data: QueryData,
    Filter: QueryFilter,
{
    pub fn iter(&self) -> QueryIter<'_, Data, Filter> {
        todo!()
    }

    pub fn iter_mut(&mut self) -> QueryIterMut<'_, Data, Filter> {
        todo!()
    }
}

impl<'a, Data, Filter> IntoIterator for &'a Query<'a, Data, Filter>
where
    Data: QueryData,
    Filter: QueryFilter,
{
    type IntoIter = QueryIter<'a, Data, Filter>;
    type Item = Data::Ref<'a>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

impl<'a, 'b, Data, Filter> IntoIterator for &'a mut Query<'b, Data, Filter>
where
    Data: QueryData,
    Filter: QueryFilter,
{
    type IntoIter = QueryIterMut<'a, Data, Filter>;
    type Item = Data::Mut<'a>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

impl<Data, Filter> SystemParam for Query<'static, Data, Filter>
where
    Data: QueryData,
    Filter: QueryFilter,
{
    type WithLifetime<'a> = Query<'a, Data, Filter>;
}
