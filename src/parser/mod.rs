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

pub trait ParseBefore<'a> {
    fn parse_before<'b, 'c>(i: &'b str) -> IResult<&'c str, &'a str>
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

impl<'a, T> ParseBefore<'a> for Vec<T>
where
    T: ParseBefore<'a>,
{
    fn parse_before<'b, 'c>(i: &'b str) -> IResult<&'c str, &'a str>
    where
        'b: 'c,
        'b: 'a,
    {
        T::parse_before(i)
    }
}
