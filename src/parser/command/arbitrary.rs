use super::{Command, Arguments};
use crate::{
    parser::Parse,
    tokens::{LeftCurlyBrace, LeftSquareBracket, RightCurlyBrace, RightSquareBracket},
};
use nom::{bytes::complete::is_not, character::complete::char, sequence::tuple, IResult, multi::many0, branch::alt};

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

pub enum ArbitraryArgument<'a> {
    Optional(ArbitraryOptionalArgument<'a>),
    Required(ArbitraryRequiredArgument<'a>),   
}

pub type ArbitraryCommand<'a> = Command<'a, Vec<ArbitraryArgument<'a>>>;

impl<'a> Arguments<'a> for ArbitraryOptionalArgument<'a> {}

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

impl<'a> Arguments<'a> for ArbitraryRequiredArgument<'a> {}

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

impl<'a> Arguments<'a> for ArbitraryArgument<'a> {}

// TODO: generate the code below using e.g. stateful macros
impl<'a> ArbitraryArgument<'a> {
    fn parse_optional<'b,'c>(i: &'b str) -> IResult<&'c str, Self>
    where
        'b: 'c,
        'b: 'a 
    {
        ArbitraryOptionalArgument::parse(i)
            .map(|(i, arg)| (i, ArbitraryArgument::Optional(arg)))
    }

    fn parse_required<'b,'c>(i: &'b str) -> IResult<&'c str, Self>
    where
        'b: 'c,
        'b: 'a 
    {
        ArbitraryRequiredArgument::parse(i)
            .map(|(i, arg)| (i, ArbitraryArgument::Required(arg)))
    }
}

impl<'a> Parse<'a> for ArbitraryArgument<'a> {
    fn parse<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
    where
            'b: 'c,
            'b: 'a {
        // TODO: generate the code below using e.g. stateful macros
        alt((Self::parse_optional, Self::parse_required))(i)
    }
}

impl<'a> Arguments<'a> for Vec<ArbitraryArgument<'a>> {}

impl<'a> Parse<'a> for Vec<ArbitraryArgument<'a>> {
    fn parse<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
    where
        'b: 'c,
        'b: 'a
    {
        many0(ArbitraryArgument::parse)(i)
    }
}
