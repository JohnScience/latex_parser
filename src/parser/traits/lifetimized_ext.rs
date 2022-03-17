pub trait LifetimizedExt {
    type Lifetimized<'a>;
}

impl<T> LifetimizedExt for Option<T>
where
    T: LifetimizedExt
{
    type Lifetimized<'a> = Option<T::Lifetimized<'a>>;
}

impl<'a> LifetimizedExt for &'a str {
    type Lifetimized<'b> = &'b str;
}

// impl<'a> 