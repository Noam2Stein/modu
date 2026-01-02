use crate::{Added, Component, Entity, With, Without, variadic::variadic};

pub trait QueryFilter: Sized + Send + Sync + 'static {}

trait QueryFilterItem: Sized + Send + Sync + 'static {}

impl<T: QueryFilterItem> QueryFilter for T {}

variadic! {
    $($T:ident)* =>

    impl<$($T),*> QueryFilter for ($($T,)*)
    where
        $($T: QueryFilterItem),*
    {
    }
}

impl<T: Component> QueryFilterItem for With<T> {}

impl<T: Component> QueryFilterItem for Without<T> {}

impl<T: Component> QueryFilterItem for Added<T> {}

impl QueryFilterItem for Added<Entity> {}
