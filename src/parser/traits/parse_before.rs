use nom::IResult;

pub trait ParseBefore<'a> {
    fn parse_before<'b, 'c>(i: &'b str) -> IResult<&'c str, &'a str>
    where
        'b: 'c,
        'b: 'a;
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