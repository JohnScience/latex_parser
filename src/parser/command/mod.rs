use crate::parser::traits::{Parse, LifetimizedExt, ParsableInput};

use crate::tokens::Backslash;
use nom::{sequence::tuple, IResult};

pub mod arbitrary;

use super::traits::{MapParsedValInResult, ParseBefore};
use from_tuple::OrderDependentFromTuple;

#[derive(OrderDependentFromTuple)]
pub struct Command<'a, A>
where
    for<'b> A: Args<'a,&'b str>,
{
    pub backslash: Backslash,
    pub cmd_name: &'a str,
    pub arguments: A,
}

pub trait Args<'a,I>: LifetimizedExt + Parse<'a,I> + ParseBefore<'a,I>
where
    I: ParsableInput
{}

impl<'a,A> LifetimizedExt for Command<'a, A>
where
    for<'b> A: Args<'a,&'b str>,
    for<'b,'c> <A as LifetimizedExt>::Lifetimized<'b>: Args<'b,&'c str>
{
    type Lifetimized<'b> = Command<'b, A::Lifetimized<'b>>;
}

impl<'a,A> Parse<'a,&str> for Command<'a, A>
where
    for<'b> A: Args<'a,&'b str>,
    for<'b,'c> <A as LifetimizedExt>::Lifetimized<'b>: Args<'b,&'c str>
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
