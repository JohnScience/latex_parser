use super::{Command, OptionalArgument, RequiredArgument, OptionalArgumentTuple, RequiredArgumentTuple};
use crate::{
    parser::Parse,
    tokens::{LeftCurlyBrace, LeftSquareBracket, RightCurlyBrace, RightSquareBracket},
};
use nom::{bytes::complete::is_not, character::complete::char, sequence::tuple, IResult, multi::many0};

pub struct ArbitraryOptionalArgument<'a> {
    pub left_bracket: LeftSquareBracket,
    pub verbatim: &'a str,
    pub right_bracket: RightSquareBracket,
}

pub struct ArbitraryRequiredArgument<'a> {
    pub left_brace: LeftCurlyBrace,
    pub verbatim: &'a str,
    pub right_brace: RightCurlyBrace,
}

type ArbitraryOptionalArguments<'a> = Vec<ArbitraryOptionalArgument<'a>>;
type ArbitraryRequiredArguments<'a> = Vec<ArbitraryRequiredArgument<'a>>;

impl<'a> OptionalArgument<'a> for ArbitraryOptionalArgument<'a> {}

impl<'a> Parse<'a> for ArbitraryOptionalArgument<'a> {
    fn parse<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
    where
        'b: 'c,
        'b: 'a,
    {
        let (i, (left_bracket, verbatim, right_bracket)) =
            (tuple((char('['), is_not("]"), char(']')))(i)).map(
                |(i, (left_bracket, verbatim, right_bracket))| {
                    (
                        i,
                        (
                            LeftSquareBracket(left_bracket),
                            verbatim,
                            RightSquareBracket(right_bracket),
                        ),
                    )
                },
            )?;
        Ok((
            i,
            Self {
                left_bracket,
                verbatim,
                right_bracket,
            },
        ))
    }
}

impl<'a> RequiredArgument<'a> for ArbitraryRequiredArgument<'a> {}

impl<'a> Parse<'a> for ArbitraryRequiredArgument<'a> {
    fn parse<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
    where
        'b: 'c,
        'b: 'a,
    {
        let (i, (left_brace, verbatim, right_brace))
            = (tuple((char('{'), is_not("}"), char('}')))(
                i,
            ))
            .map(|(i, (left_brace, verbatim, right_brace))| {
                (
                    i,
                    (
                        LeftCurlyBrace(left_brace),
                        verbatim,
                        RightCurlyBrace(right_brace),
                    ),
                )
            })?;
        Ok((
            i,
            Self {
                left_brace,
                verbatim,
                right_brace,
            },
        ))
    }
}

impl<'a> Parse<'a> for ArbitraryOptionalArguments<'a> {
    fn parse<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
    where
        'b: 'c,
        'b: 'a
    {
        many0(ArbitraryOptionalArgument::parse)(i)
    }
}

impl<'a> Parse<'a> for ArbitraryRequiredArguments<'a> {
    fn parse<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
    where
        'b: 'c,
        'b: 'a
    {
        many0(ArbitraryRequiredArgument::parse)(i)
    }
}

impl<'a> OptionalArgumentTuple<'a> for ArbitraryOptionalArguments<'a> {}
impl<'a> RequiredArgumentTuple<'a> for ArbitraryRequiredArguments<'a> {}

pub type ArbitraryCommand<'a> =
    Command<'a, ArbitraryOptionalArguments<'a>, ArbitraryRequiredArguments<'a>>;
