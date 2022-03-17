use crate::parser::traits::{Parse, LifetimizedExt};

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

#[marker]
pub trait Args<'a>: LifetimizedExt + Parse<'a> + ParseBefore<'a> {}

impl<'a,A> LifetimizedExt for Command<'a, A>
where
    A: Args<'a>,
    for<'b> <A as LifetimizedExt>::Lifetimized<'b>: Args<'b>
{
    type Lifetimized<'b> = Command<'b, A::Lifetimized<'b>>;
}

impl<'a, A> Parse<'a> for Command<'a, A>
where
    A: Args<'a>,
    for<'b> <A as LifetimizedExt>::Lifetimized<'b>: Args<'b>
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
