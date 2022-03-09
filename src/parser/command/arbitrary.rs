use super::{Args, Command};
use crate::{
    parser::Parse,
    tokens::{Braces, Brackets, DelimPair},
};
use nom::IResult;

pub struct ArbitraryDelimitedArg<'a, D>
where
    D: DelimPair,
{
    pub left_delim: D::Left,
    pub verbatim: &'a str,
    pub right_delim: D::Right,
}

pub type ArbitraryBracketedArg<'a> = ArbitraryDelimitedArg<'a, Brackets>;
pub type ArbitraryBracedArg<'a> = ArbitraryDelimitedArg<'a, Braces>;

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
    use crate::{
        parser::Parse,
        tokens::{CharToken, DelimPair},
    };

    use super::{ArbitraryArg, ArbitraryDelimitedArg, Args};

    impl<'a, D> Args<'a> for ArbitraryDelimitedArg<'a, D>
    where
        D: DelimPair,
        D::Left: Parse<'a>,
        D::Right: Parse<'a> + CharToken,
    {
    }
    impl<'a> Args<'a> for ArbitraryArg<'a> {}
    impl<'a> Args<'a> for Vec<ArbitraryArg<'a>> {}
}

mod parse_impls {
    use crate::tokens::{CharToken, DelimPair};

    use super::{ArbitraryArg, ArbitraryDelimitedArg, Parse};
    use nom::{branch::alt, bytes::complete::is_not, multi::many0, sequence::tuple, IResult};

    impl<'a, D> Parse<'a> for ArbitraryDelimitedArg<'a, D>
    where
        D: DelimPair,
        D::Left: Parse<'a>,
        D::Right: Parse<'a> + CharToken,
    {
        fn parse<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
        where
            'b: 'c,
            'b: 'a,
        {
            let (i, (left_delim, verbatim, right_delim)) =
                // FIXME: Handle nested braces, e.g. [before[action]after]
                (tuple((D::Left::parse, is_not(D::Right::CHAR_STR), D::Right::parse))(i))?;
            Ok((
                i,
                Self {
                    left_delim,
                    verbatim,
                    right_delim,
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
