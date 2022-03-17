use crate::parser::traits::ParseStr;

use crate::tokens::Backslash;
use nom::{sequence::tuple, IResult};

pub mod arbitrary;

use super::traits::{MapParsedValInResult, ParseBefore};
use from_tuple::OrderDependentFromTuple;

#[derive(OrderDependentFromTuple)]
pub struct Command<'a, A>
where
    A: Args<'a>,
{
    pub backslash: Backslash,
    pub cmd_name: &'a str,
    pub arguments: A,
}

pub trait Args<'a>: ParseStr<'a> + ParseBefore<'a> {}

impl<'a, A> ParseStr<'a> for Command<'a, A>
where
    A: Args<'a>,
{
    fn parse<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
    where
        'b: 'c,
        'b: 'a,
    {
        tuple((Backslash::parse, A::parse_before, A::parse))(i)
            .map_parsed_val(<_ as Into<Command<A>>>::into)
    }
}
