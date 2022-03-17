// Proper declaration and implementation requires GATs

use nom::{combinator::opt, IResult};
use crate::parser::span::StrSpan;

pub trait ParseStr<'a>: Sized {
    /// Returns [`Result<P,E>`] where any [`Ok(p)`] is a pair `(i,val)`, s.t.
    /// * `i` is the remaining input after parsing
    /// * `val` is the parsed value
    fn parse<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
    where
        'b: 'c,
        'b: 'a;
}

impl<'a, T> ParseStr<'a> for Option<T>
where
    T: ParseStr<'a>,
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
