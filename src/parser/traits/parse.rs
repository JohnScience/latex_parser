use nom::{combinator::opt, IResult};

pub trait Parse<'a>: Sized {
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
    T: Parse<'a>,
{
    fn parse<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
    where
        'b: 'c,
        'b: 'a,
    {
        opt(T::parse)(i)
    }
}
