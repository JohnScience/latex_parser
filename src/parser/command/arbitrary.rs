use super::{Args, Command};
use crate::{
    parser::traits::{Parse, MapParsedValInResult, FromTuple},
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
    fn parse_bracketed<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
    where
        'b: 'c,
        'b: 'a,
    {
        ArbitraryBracketedArg::parse(i).map_parsed_val(ArbitraryArg::Optional)
    }

    fn parse_braced<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
    where
        'b: 'c,
        'b: 'a,
    {
        ArbitraryBracedArg::parse(i).map_parsed_val(ArbitraryArg::Required)
    }
}

impl<'a, D> FromTuple<(D::Left,&'a str, D::Right)> for ArbitraryDelimitedArg<'a, D>
    where
        D: DelimPair,
{
    fn from_tuple((left_delim,verbatim,right_delim): (D::Left,&'a str, D::Right)) -> Self {
        Self { left_delim, verbatim, right_delim }
    }
}

mod args_impls {
    use crate::{
        parser::traits::{Parse, ParseBefore},
        tokens::{CharToken, DelimPair},
    };

    use super::{ArbitraryArg, ArbitraryDelimitedArg, Args};

    impl<'a, D> Args<'a> for ArbitraryDelimitedArg<'a, D>
    where
        D: DelimPair,
        D::Left: ParseBefore<'a> + CharToken + Parse<'a>,
        D::Right: ParseBefore<'a> + CharToken + Parse<'a>,
    {
    }
    impl<'a> Args<'a> for ArbitraryArg<'a> {}
    impl<'a> Args<'a> for Vec<ArbitraryArg<'a>> {}
}

mod parse_impls {
    use crate::{
        parser::traits::ParseBefore,
        tokens::{CharToken, DelimPair},
    };

    use super::{ArbitraryArg, ArbitraryDelimitedArg, Parse, FromTuple, MapParsedValInResult};
    use nom::{branch::alt, multi::many0, sequence::tuple, IResult};

    impl<'a, D> Parse<'a> for ArbitraryDelimitedArg<'a, D>
    where
        D: DelimPair,
        D::Left: Parse<'a>,
        D::Right: Parse<'a> + ParseBefore<'a> + CharToken,
    {
        fn parse<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
        where
            'b: 'c,
            'b: 'a,
        {
            // FIXME: Handle nested braces, e.g. [before[action]after]
            (tuple((D::Left::parse, D::Right::parse_before, D::Right::parse))(i)).map_parsed_val(Self::from_tuple)
        }
    }

    impl<'a> Parse<'a> for ArbitraryArg<'a> {
        fn parse<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
        where
            'b: 'c,
            'b: 'a,
        {
            // TODO: generate the code below using e.g. stateful macros
            alt((Self::parse_bracketed, Self::parse_braced))(i)
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

mod parse_before_impls {
    use nom::bytes::complete::{is_not, take_till1};

    use crate::{
        parser::traits::ParseBefore,
        tokens::{CharToken, DelimPair, LeftBrace, LeftBracket},
    };

    use super::{ArbitraryArg, ArbitraryDelimitedArg};

    impl<'a, C> ParseBefore<'a> for C
    where
        C: CharToken,
    {
        fn parse_before<'b, 'c>(i: &'b str) -> nom::IResult<&'c str, &'a str>
        where
            'b: 'c,
            'b: 'a,
        {
            is_not(C::CHAR_STR)(i)
        }
    }

    impl<'a, D> ParseBefore<'a> for ArbitraryDelimitedArg<'a, D>
    where
        D: DelimPair,
        D::Left: CharToken + ParseBefore<'a>,
    {
        fn parse_before<'b, 'c>(i: &'b str) -> nom::IResult<&'c str, &'a str>
        where
            'b: 'c,
            'b: 'a,
        {
            D::Left::parse_before(i)
        }
    }

    impl<'a> ParseBefore<'a> for ArbitraryArg<'a> {
        fn parse_before<'b, 'c>(i: &'b str) -> nom::IResult<&'c str, &'a str>
        where
            'b: 'c,
            'b: 'a,
        {
            take_till1(|c| c == LeftBrace::CHAR || c == LeftBracket::CHAR)(i)
        }
    }
}
