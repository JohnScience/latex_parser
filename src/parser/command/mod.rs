use crate::parser::Parse;

use crate::tokens::Backslash;
use nom::{
    character::complete::char,
    sequence::pair,
    IResult, bytes::complete::take_till1,
};

pub mod arbitrary;

pub use arbitrary::ArbitraryCommand;

pub trait OptionalArgument<'a>: Parse<'a> {}

pub trait RequiredArgument<'a>: Parse<'a> {}

pub trait OptionalArgumentTuple<'a>: Parse<'a> {}

pub trait RequiredArgumentTuple<'a>: Parse<'a> {}

pub struct Command<'a, O, R>
where
    O: OptionalArgumentTuple<'a>,
    R: RequiredArgumentTuple<'a>,
{
    pub backslash: Backslash,
    pub cmd_name: &'a str,
    pub optional_arguments: O,
    pub required_arguments: R,
}

impl<'a,O,R> Parse<'a> for Command<'a,O,R>
where
    O: OptionalArgumentTuple<'a>,
    R: RequiredArgumentTuple<'a>,
{
    fn parse<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
    where
            'b: 'c,
            'b: 'a {
        let (i, (backslash, cmd_name))
            = pair(char('\\'), take_till1(|c| c == '[' || c == '{'))(i)
            .map(|(i,(backslash, cmd_name))| (
                (i, (Backslash(backslash), cmd_name))
            ))?;
        let (i, optional_argument) = O::parse(i)?;
        let (i, required_argument) = R::parse(i)?;
        Ok((i, Self { backslash, cmd_name, optional_arguments: optional_argument, required_arguments: required_argument }))
    }
}
