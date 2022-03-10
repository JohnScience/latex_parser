// Currently, the introspection capabilities of Rust don't permit the necessary check

pub trait FromTuple<T> {
    fn from_tuple(t: T) -> Self;
}
