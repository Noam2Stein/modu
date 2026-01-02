macro_rules! variadic {
    ($d0:tt($d1:tt$T:ident:ident)* => $($body:tt)*) => {
        macro_rules! _variadic {
            { $d0($d1$T:ident)* } => {
                $($body)*
            }
        }
        _variadic! {}
        _variadic! { T0 }
        _variadic! { T0 T1 }
        _variadic! { T0 T1 T2 }
        _variadic! { T0 T1 T2 T3 }
        _variadic! { T0 T1 T2 T3 T4 }
        _variadic! { T0 T1 T2 T3 T4 T5 }
        _variadic! { T0 T1 T2 T3 T4 T5 T6 }
        _variadic! { T0 T1 T2 T3 T4 T5 T6 T7 }
    };

    ($d0:tt($d1:tt$T:ident:ident)* $d2:tt$len:ident:literal => $($body:tt)*) => {
        macro_rules! _variadic {
            { $d0($d1$T:ident)* $d2$len:literal } => {
                $($body)*
            }
        }
        _variadic! { 0 }
        _variadic! { T0 1 }
        _variadic! { T0 T1 2 }
        _variadic! { T0 T1 T2 3 }
        _variadic! { T0 T1 T2 T3 4 }
        _variadic! { T0 T1 T2 T3 T4 5 }
        _variadic! { T0 T1 T2 T3 T4 T5 6 }
        _variadic! { T0 T1 T2 T3 T4 T5 T6 7 }
        _variadic! { T0 T1 T2 T3 T4 T5 T6 T7 8 }
    };
}

pub(crate) use variadic;
