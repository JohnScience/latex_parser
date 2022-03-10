// Proper declaration and implementation requires GATs

use nom::IResult;

pub trait MapParsedValInTuple<'a,T>
    where
        T: super::Parse<'a>
{
    fn map_parsed_val<'b,U,F: FnOnce(T) -> U>(self,f: F) -> (&'b str, U)
    where
        'a: 'b;
}

impl<'a,T> MapParsedValInTuple<'a,T> for (&'a str,T)
    where
        T: super::Parse<'a>
{
    fn map_parsed_val<'b,U,F: FnOnce(T) -> U>(self,f: F) -> (&'b str, U)
    where
        'a: 'b
    {
        (self.0,f(self.1))
    }
}

pub trait MapParsedValInResult<'a,T>
    where
        T: super::Parse<'a>
{
    fn map_parsed_val<'b,U,F: FnOnce(T) -> U>(self,f: F) -> IResult<&'b str, U>
    where
        'a: 'b;
}

impl<'a,T> MapParsedValInResult<'a,T> for IResult<&'a str, T>
    where
        T: super::Parse<'a>
{
    fn map_parsed_val<'b,U,F: FnOnce(T) -> U>(self,f: F) -> IResult<&'b str, U>
    where
        'a: 'b
    {
        self.map(|parsed_pair| parsed_pair.map_parsed_val(f))
    }
}
