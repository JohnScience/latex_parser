use crate::parser::Parse;

use crate::tokens::Backslash;
use nom::{bytes::complete::take_till1, sequence::tuple, IResult};

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
        let (i, (backslash, cmd_name, arguments)) = tuple((
            Backslash::parse,
            take_till1(|c| c == '[' || c == '{'),
            A::parse,
        ))(i)?;
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
