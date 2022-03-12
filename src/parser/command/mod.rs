use crate::parser::traits::Parse;

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

pub trait Args<'a>: Parse<'a> + ParseBefore<'a> {}

impl<'a, A> Parse<'a> for Command<'a, A>
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
