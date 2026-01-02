use crate::{
    Ecs, Params,
    system::{System, SystemParam},
    variadic::variadic,
};

pub unsafe trait Systems: Sized + Send + Sync + Clone + Copy + 'static {
    fn run(&self, ecs: &mut Ecs) {
        self.run_with(ecs, ());
    }

    fn run_with(&self, ecs: &mut Ecs, params: impl Params) {
        let _ = ecs;
        let _ = params;
        todo!()
    }

    fn before(self, systems: impl Systems) -> impl Systems {
        Before(self, systems)
    }

    fn after(self, systems: impl Systems) -> impl Systems {
        After(self, systems)
    }

    fn after_commands(self) -> impl Systems {
        AfterCommands(self)
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Before<T: Systems, U: Systems>(T, U);

#[derive(Debug, Clone, Copy, Default)]
struct After<T: Systems, U: Systems>(T, U);

#[derive(Debug, Clone, Copy, Default)]
struct AfterCommands<T: Systems>(T);

#[doc(hidden)]
unsafe impl<F, T> Systems for F
where
    F: Fn() -> T + Send + Sync + Clone + Copy + 'static,
    T: Systems,
{
}

variadic! {
    $($T:ident)* $len:literal =>

    #[doc(hidden)]
    unsafe impl<F, $($T),*> Systems for System<F, ($($T,)*)>
    where
        F: Fn($($T),*) + Send + Sync + Clone + Copy + 'static,
        $($T: SystemParam,)*
        F: for<'a> Fn($($T::WithLifetime<'a>),*),
    {
    }
}

variadic! {
    $($T:ident)* =>

    #[doc(hidden)]
    unsafe impl<$($T),*> Systems for ($($T,)*)
    where
        $($T: Systems),*
    {
    }
}

unsafe impl<T: Systems, U: Systems> Systems for Before<T, U> {}

unsafe impl<T: Systems, U: Systems> Systems for After<T, U> {}

unsafe impl<T: Systems> Systems for AfterCommands<T> {}
