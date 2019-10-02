pub trait Unwrap<T> {
    fn unwrap(self) -> T;
}

impl<T> Unwrap<T> for Option<T> {
    fn unwrap(self) -> T {
        self.unwrap_or_else(|| unreachable!())
    }
}

impl<T, E> Unwrap<T> for Result<T, E> {
    fn unwrap(self) -> T {
        self.unwrap_or_else(|_| unreachable!())
    }
}

pub fn unwrap<T, U: Unwrap<T>>(x: U) -> T {
    Unwrap::unwrap(x)
}
