// Proper declaration and implementation requires GATs

use nom::{combinator::opt, IResult};
use crate::parser::{
    span::StrSpan,
    traits::LifetimizedExt
};
pub trait ParsableInput: LifetimizedExt {}

impl<'a> ParsableInput for &'a str {}
impl<'a> ParsableInput for StrSpan<'a> {}

pub trait Parse<'a,I>: Sized + LifetimizedExt
where
    I: ParsableInput
{
    /// Returns [`Result<P,E>`] where any [`Ok(p)`] is a pair `(i,val)`, s.t.
    /// * `i` is the remaining input after parsing
    /// * `val` is the parsed value
    fn parse<'b, 'c>(i: I::Lifetimized<'b>) -> IResult<I::Lifetimized<'c>, Self>
    where
        'b: 'c,
        'b: 'a;
}

impl<'a,T> Parse<'a,&str> for Option<T>
where
    for <'d> T: Parse<'a,&'d str> + LifetimizedExt,
{
    fn parse<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
    where
        'b: 'c,
        'b: 'a,
    {
        opt(T::parse)(i)
    }
}
