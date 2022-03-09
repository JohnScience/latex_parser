use crate::parser::Parse;

use crate::tokens::Backslash;
use nom::{bytes::complete::take_till1, character::complete::char, sequence::pair, IResult};

pub mod arbitrary;

pub use arbitrary::ArbitraryCommand;

pub struct Command<'a, A>
where
    A: Args<'a>,
{
    pub backslash: Backslash,
    pub cmd_name: &'a str,
    pub arguments: A,
}

pub trait Args<'a>: Parse<'a> {}

impl<'a, A> Parse<'a> for Command<'a, A>
where
    A: Args<'a>,
{
    fn parse<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
    where
        'b: 'c,
        'b: 'a,
    {
        let (i, (backslash, cmd_name)) = pair(char('\\'), take_till1(|c| c == '[' || c == '{'))(i)
            .map(|(i, (backslash, cmd_name))| ((i, (Backslash(backslash), cmd_name))))?;
        let (i, arguments) = A::parse(i)?;
        Ok((
            i,
            Self {
                backslash,
                cmd_name,
                arguments,
            },
        ))
    }
}
