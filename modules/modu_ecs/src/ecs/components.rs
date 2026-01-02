use crate::{Component, variadic::variadic};

pub trait Components {}

impl<T: Component> Components for T {}

variadic! {
    $($T:ident)* =>

    impl<$($T),*> Components for ($($T,)*)
    where
        $($T: Components),*
    {
    }
}
