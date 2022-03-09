use super::{Args, Command};
use crate::{
    parser::Parse,
    tokens::{LeftBrace, LeftBracket, RightBrace, RightBracket},
};
use nom::IResult;

pub struct ArbitraryBracketedArg<'a> {
    pub left_bracket: LeftBracket,
    pub verbatim: &'a str,
    pub right_bracket: RightBracket,
}

pub struct ArbitraryBracedArg<'a> {
    pub left_brace: LeftBrace,
    pub verbatim: &'a str,
    pub right_brace: RightBrace,
}

pub enum ArbitraryArg<'a> {
    Optional(ArbitraryBracketedArg<'a>),
    Required(ArbitraryBracedArg<'a>),
}

pub type ArbitraryCommand<'a> = Command<'a, Vec<ArbitraryArg<'a>>>;

// TODO: generate the code below using e.g. stateful macros
impl<'a> ArbitraryArg<'a> {
    fn parse_optional<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
    where
        'b: 'c,
        'b: 'a,
    {
        ArbitraryBracketedArg::parse(i).map(|(i, arg)| (i, ArbitraryArg::Optional(arg)))
    }

    fn parse_required<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
    where
        'b: 'c,
        'b: 'a,
    {
        ArbitraryBracedArg::parse(i).map(|(i, arg)| (i, ArbitraryArg::Required(arg)))
    }
}

mod args_impls {
    use super::{ArbitraryArg, ArbitraryBracedArg, ArbitraryBracketedArg, Args};

    impl<'a> Args<'a> for ArbitraryBracketedArg<'a> {}
    impl<'a> Args<'a> for ArbitraryBracedArg<'a> {}
    impl<'a> Args<'a> for ArbitraryArg<'a> {}
    impl<'a> Args<'a> for Vec<ArbitraryArg<'a>> {}
}

mod parse_impls {
    use super::{
        ArbitraryArg, ArbitraryBracedArg, ArbitraryBracketedArg, LeftBrace, LeftBracket, Parse,
        RightBrace, RightBracket,
    };
    use nom::{
        branch::alt, bytes::complete::is_not, character::complete::char, multi::many0,
        sequence::tuple, IResult,
    };

    impl<'a> Parse<'a> for ArbitraryBracketedArg<'a> {
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
                                LeftBracket(left_bracket),
                                verbatim,
                                RightBracket(right_bracket),
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

    impl<'a> Parse<'a> for ArbitraryBracedArg<'a> {
        fn parse<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
        where
            'b: 'c,
            'b: 'a,
        {
            let (i, (left_brace, verbatim, right_brace)) =
                // FIXME: Handle nested braces, e.g. {before{action}after}
                (tuple((char('{'), is_not("}"), char('}')))(i)).map(
                    |(i, (left_brace, verbatim, right_brace))| {
                        (
                            i,
                            (LeftBrace(left_brace), verbatim, RightBrace(right_brace)),
                        )
                    },
                )?;
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

    impl<'a> Parse<'a> for ArbitraryArg<'a> {
        fn parse<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
        where
            'b: 'c,
            'b: 'a,
        {
            // TODO: generate the code below using e.g. stateful macros
            alt((Self::parse_optional, Self::parse_required))(i)
        }
    }

    impl<'a> Parse<'a> for Vec<ArbitraryArg<'a>> {
        fn parse<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
        where
            'b: 'c,
            'b: 'a,
        {
            many0(ArbitraryArg::parse)(i)
        }
    }
}
