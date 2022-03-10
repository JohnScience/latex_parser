use crate::parser::traits::Parse;

use crate::tokens::Backslash;
use nom::{sequence::tuple, IResult};

pub mod arbitrary;

pub use arbitrary::ArbitraryCommand;

use super::traits::{FromTuple, MapParsedValInResult, ParseBefore};

pub struct Command<'a, A>
where
    A: Args<'a>,
{
    pub backslash: Backslash,
    pub cmd_name: &'a str,
    pub arguments: A,
}

pub trait Args<'a>: Parse<'a> + ParseBefore<'a> {}

impl<'a, A> FromTuple<(Backslash, &'a str, A)> for Command<'a, A>
where
    A: Args<'a>,
{
    fn from_tuple((backslash, cmd_name, arguments): (Backslash, &'a str, A)) -> Self {
        Self {
            backslash,
            cmd_name,
            arguments,
        }
    }
}

impl<'a, A> Parse<'a> for Command<'a, A>
where
    A: Args<'a>,
{
    fn parse<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
    where
        'b: 'c,
        'b: 'a,
    {
        tuple((Backslash::parse, A::parse_before, A::parse))(i).map_parsed_val(Command::from_tuple)
    }
}
