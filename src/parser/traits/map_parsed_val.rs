pub trait MapParsedVal<'a,T>
    where
        T: super::Parse<'a>
{
    fn map_parsed_val<'b,U,F: FnOnce(T) -> U>(self,f: F) -> (&'b str, U)
    where
        'a: 'b;
}

impl<'a,T> MapParsedVal<'a,T> for (&'a str,T)
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