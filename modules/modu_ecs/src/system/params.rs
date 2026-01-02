use crate::{Parameter, variadic::variadic};

pub unsafe trait Params {}

unsafe impl<'a, T> Params for &'a T where T: ?Sized + Parameter {}

unsafe impl<'a, T> Params for &'a mut T where T: ?Sized + Parameter {}

variadic! {
    $($T:ident)* =>

    #[doc(hidden)]
    unsafe impl<$($T),*> Params for ($($T,)*)
    where
        $($T: Params,)*
    {
    }
}
