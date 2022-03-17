pub trait LifetimizedExt {
    type Lifetimized<'a>;
}

impl<T> LifetimizedExt for Option<T>
where
    T: LifetimizedExt
{
    type Lifetimized<'a> = Option<T::Lifetimized<'a>>;
}

impl<'a,T> LifetimizedExt for &'a T
where
    T: 'static
{
    type Lifetimized<'b> = &'b T;
}

impl<'a,T> LifetimizedExt for &'a mut T
where
    T: 'static
{
    type Lifetimized<'b> = &'b mut T;
}
