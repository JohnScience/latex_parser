// Proper declaration and implementation requires GATs

use nom::{combinator::opt, IResult};
use crate::parser::{
    span::StrSpan,
    traits::LifetimizedExt
};

pub trait Parse<'a>: Sized + LifetimizedExt {
    /// Returns [`Result<P,E>`] where any [`Ok(p)`] is a pair `(i,val)`, s.t.
    /// * `i` is the remaining input after parsing
    /// * `val` is the parsed value
    fn parse<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
    where
        'b: 'c,
        'b: 'a;
}

impl<'a, T> Parse<'a> for Option<T>
where
    T: Parse<'a> + LifetimizedExt,
{
    fn parse<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
    where
        'b: 'c,
        'b: 'a,
    {
        opt(T::parse)(i)
    }
}

pub trait ParseSpanned<'a>: Sized {
    /// Returns [`Result<P,E>`] where any [`Ok(p)`] is a pair `(i,val)`, s.t.
    /// * `i` is the remaining input after parsing
    /// * `val` is the parsed value
    fn parse<'b, 'c>(i: StrSpan<'b>) -> IResult<StrSpan<'c>, Self>
    where
        'b: 'c,
        'b: 'a;
}
