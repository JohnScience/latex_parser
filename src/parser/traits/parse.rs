// Proper declaration and implementation requires GATs

use nom::{combinator::opt, IResult};
use crate::parser::{
    traits::{LifetimizedExt, ParsableInput}
};

pub trait Parse<'a,I>: Sized
where
    I: ParsableInput,
    Self: LifetimizedExt<Lifetimized<'a> = Self>
{
    /// Returns [`Result<P,E>`] where any [`Ok(p)`] is a pair `(i,val)`, s.t.
    /// * `i` is the remaining input after parsing
    /// * `val` is the parsed value
    fn parse<'b, 'c>(i: I::Lifetimized<'b>) -> IResult<I::Lifetimized<'c>, Self::Lifetimized<'a>>
    where
        'b: 'c,
        'b: 'a;
}

impl<'a,T,I> Parse<'a,I> for Option<T>
where
    I: ParsableInput,
    Self: LifetimizedExt<Lifetimized<'a> = Self>,
    for <'b> T: Parse<'a,I::Lifetimized<'b>> + LifetimizedExt,
    for <'b> <I as LifetimizedExt>::Lifetimized<'b>: ParsableInput,
{
    fn parse<'b, 'c>(i: I::Lifetimized<'b>) -> IResult<I::Lifetimized<'c>, Self::Lifetimized<'a>>
    where
        'b: 'c,
        'b: 'a,
    {
        opt(T::parse)(i)
    }
}
