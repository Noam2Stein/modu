use crate::{Component, Entity, EntityMut, EntityRef, ecs::EntityPartialMut, variadic::variadic};

pub trait QueryData: Sized + Send + Sync {
    type Ref<'a>;
    type Mut<'a>;
}

pub trait QueryDataItem: Sized + Send + Sync {
    type Ref<'a>;
    type Mut<'a>;
}

impl<T: QueryDataItem> QueryData for T {
    type Ref<'a> = T::Ref<'a>;
    type Mut<'a> = T::Mut<'a>;
}

variadic! {
    $($T:ident)* =>

    impl<$($T),*> QueryData for ($($T,)*)
    where
        $($T: QueryDataItem),*
    {
        type Ref<'a> = ($($T::Ref<'a>,)*);
        type Mut<'a> = ($($T::Mut<'a>,)*);
    }
}

impl QueryDataItem for Entity {
    type Ref<'a> = Entity;
    type Mut<'a> = Entity;
}

impl<'a, T: Component> QueryDataItem for &'a T {
    type Ref<'b> = &'b T;
    type Mut<'b> = &'b T;
}

impl<'a, T: Component> QueryDataItem for &'a mut T {
    type Ref<'b> = &'b T;
    type Mut<'b> = &'b mut T;
}

impl<'a, T: Component> QueryDataItem for Option<&'a T> {
    type Ref<'b> = Option<&'b T>;
    type Mut<'b> = Option<&'b T>;
}

impl<'a, T: Component> QueryDataItem for Option<&'a mut T> {
    type Ref<'b> = Option<&'b T>;
    type Mut<'b> = Option<&'b mut T>;
}

impl<'a> QueryDataItem for EntityRef<'a> {
    type Ref<'b> = EntityRef<'b>;
    type Mut<'b> = EntityRef<'b>;
}

impl<'a> QueryDataItem for EntityMut<'a> {
    type Ref<'b> = EntityRef<'b>;
    type Mut<'b> = EntityPartialMut<'b>;
}
