// provides a trait for types that are equivalent to a "sum" type.
pub trait Either {
    type Left;
    type Right;

    fn either<T>(self, left: impl Fn(Self::Left) -> T, right: impl Fn(Self::Right) -> T) -> T;
    fn left(left: Self::Left) -> Self;
    fn right(right: Self::Right) -> Self;
}

impl<R, E> Either for Result<R, E> {
    type Left = E;
    type Right = R;

    fn either<T>(self, left: impl Fn(Self::Left) -> T, right: impl Fn(Self::Right) -> T) -> T {
        match self {
            Ok(t) => right(t),
            Err(e) => left(e),
        }
    }

    fn left(left: Self::Left) -> Self {
        Err(left)
    }

    fn right(right: Self::Right) -> Self {
        Ok(right)
    }
}

impl<A> Either for Option<A> {
    type Left = ();
    type Right = A;

    fn either<T>(self, left: impl Fn(Self::Left) -> T, right: impl Fn(Self::Right) -> T) -> T {
        self.ok_or(()).either(left, right)
    }

    fn left(_: Self::Left) -> Self {
        None
    }

    fn right(right: Self::Right) -> Self {
        Some(right)
    }
}
