use super::context::{AppContext, FromContext};

pub trait Factory<T, R>: Clone + 'static {
    fn call_with_context(&self, ctx: &AppContext) -> R;
}

impl<F, R> Factory<(), R> for F
where
    F: Fn() -> R + Clone + 'static,
{
    fn call_with_context(&self, _: &AppContext) -> R {
        (self)()
    }
}

macro_rules! tuple_fn_factory {
    ($(($T:ident, $n:tt)),+) => {
        impl<Func, $($T,)+ R> Factory<($($T,)+), R> for Func
        where
        Func: Fn($($T,)+) -> R + Clone + 'static,
        $($T: FromContext),+
        {
            fn call_with_context(&self, ctx: &AppContext) -> R {
                let param = (
                    $($T::extract(ctx),)+
                );
                (self)($(param.$n,)+)
            }
        }
    }
}

#[rustfmt::skip]
mod m {
    use super::*;
    tuple_fn_factory!((A, 0));
    tuple_fn_factory!((A, 0), (B, 1));
    tuple_fn_factory!((A, 0), (B, 1), (C, 2));
    tuple_fn_factory!((A, 0), (B, 1), (C, 2), (D, 3));
    tuple_fn_factory!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4));
    tuple_fn_factory!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5));
    tuple_fn_factory!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6));
    tuple_fn_factory!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7));
}
