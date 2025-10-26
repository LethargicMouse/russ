pub trait Either {
    type Left;
    type Right;

    fn either<T>(
        self,
        left: impl FnOnce(Self::Left) -> T,
        right: impl FnOnce(Self::Right) -> T,
    ) -> T;
}

impl<A> Either for Option<A> {
    type Left = ();

    type Right = A;

    fn either<T>(
        self,
        left: impl FnOnce(Self::Left) -> T,
        right: impl FnOnce(Self::Right) -> T,
    ) -> T {
        match self {
            Some(t) => right(t),
            None => left(()),
        }
    }
}

impl<R, E> Either for Result<R, E> {
    type Left = E;

    type Right = R;

    fn either<T>(
        self,
        left: impl FnOnce(Self::Left) -> T,
        right: impl FnOnce(Self::Right) -> T,
    ) -> T {
        match self {
            Ok(r) => right(r),
            Err(e) => left(e),
        }
    }
}
