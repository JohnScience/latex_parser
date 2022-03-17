// Proper declaration and implementation requires GATs

use nom::IResult;

pub trait MapParsedValInTuple<'a, T>
where
    T: super::Parse<'a>,
{
    fn map_parsed_val<'b, U, F: FnOnce(T) -> U>(self, f: F) -> (&'b str, U)
    where
        'a: 'b;
}

impl<'a, T> MapParsedValInTuple<'a, T> for (&'a str, T)
where
    T: super::Parse<'a>,
{
    fn map_parsed_val<'b, U, F: FnOnce(T) -> U>(self, f: F) -> (&'b str, U)
    where
        'a: 'b,
    {
        let (remaining_input,parsed_val) = (self.0, self.1);
        (remaining_input, f(parsed_val))
    }
}

pub trait MapParsedValInResult<'a, T> {
    fn map_parsed_val<'b, U, F: FnOnce(T) -> U>(self, f: F) -> IResult<&'b str, U>
    where
        'a: 'b;
}

impl<'a, T> MapParsedValInResult<'a, T> for IResult<&'a str, T> {
    fn map_parsed_val<'b, U, F: FnOnce(T) -> U>(self, f: F) -> IResult<&'b str, U>
    where
        'a: 'b,
    {
        self.map(|(i, parsed_val)| (i, f(parsed_val)))
    }
}
