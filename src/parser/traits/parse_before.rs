use nom::IResult;
use crate::parser::traits::ParsableInput;

pub trait ParseBefore<'a,I>
where
    I: ParsableInput
{
    fn parse_before<'b, 'c>(i: I::Lifetimized<'b>) -> IResult<I::Lifetimized<'c>, &'a str>
    where
        'b: 'c,
        'b: 'a;
}

impl<'a,T> ParseBefore<'a,&str> for Vec<T>
where
    for<'b> T: ParseBefore<'a,&'b str>,
{
    fn parse_before<'b, 'c>(i: &'b str) -> IResult<&'c str, &'a str>
    where
        'b: 'c,
        'b: 'a,
    {
        T::parse_before(i)
    }
}
