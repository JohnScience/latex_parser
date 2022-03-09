use nom::{combinator::opt, IResult};

pub mod command;
pub mod comment;

pub use command::{arbitrary::ArbitraryCommand, Command};

pub trait Parse<'a>: Sized {
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
