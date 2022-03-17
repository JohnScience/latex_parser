use super::{Args, Command};
use crate::{
    parser::traits::{MapParsedValInResult, Parse},
    tokens::{Braces, Brackets, DelimPair},
};
use from_tuple::OrderDependentFromTuple;
use nom::IResult;

#[derive(OrderDependentFromTuple)]
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
    Bracketed(ArbitraryBracketedArg<'a>),
    Braced(ArbitraryBracedArg<'a>),
}

pub type ArbitraryCommand<'a> = Command<'a, Vec<ArbitraryArg<'a>>>;

// TODO: generate the code below using e.g. stateful macros
impl<'a> ArbitraryArg<'a> {
    fn parse_bracketed<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
    where
        'b: 'c,
        'b: 'a,
    {
        ArbitraryBracketedArg::parse(i).map_parsed_val(ArbitraryArg::Bracketed)
    }

    fn parse_braced<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
    where
        'b: 'c,
        'b: 'a,
    {
        ArbitraryBracedArg::parse(i).map_parsed_val(ArbitraryArg::Braced)
    }
}

mod lifetimized_ext_impls {
    use crate::{
        parser::traits::{LifetimizedExt,Parse, ParseBefore},
        tokens::{CharToken, DelimPair},
    };

    use super::{ArbitraryArg, ArbitraryDelimitedArg};

    impl<'a, D> LifetimizedExt for ArbitraryDelimitedArg<'a, D>
    where
        D: DelimPair,
        for<'b,'c> D::Left: ParseBefore<'a,&'c str> + CharToken + Parse<'a,&'b str>,
        for<'b,'c> D::Right: ParseBefore<'a,&'c str> + CharToken + Parse<'a,&'b str>,
    {
        type Lifetimized<'b> = ArbitraryDelimitedArg<'b,D>;
    }
    impl<'a> LifetimizedExt for ArbitraryArg<'a> {
        type Lifetimized<'b> = ArbitraryArg<'b>;
    }
    impl<'a> LifetimizedExt for Vec<ArbitraryArg<'a>> {
        type Lifetimized<'b> = Vec<ArbitraryArg<'b>>;
    }
}

mod args_impls {
    use crate::{
        parser::traits::{Parse, ParseBefore},
        tokens::{CharToken, DelimPair},
    };

    use super::{ArbitraryArg, ArbitraryDelimitedArg, Args};

    impl<'a, D> Args<'a,&str> for ArbitraryDelimitedArg<'a, D>
    where
        D: DelimPair,
        for<'b,'c> D::Left: ParseBefore<'a,&'c str> + CharToken + Parse<'a,&'b str>,
        for<'b,'c> D::Right: ParseBefore<'a,&'c str> + CharToken + Parse<'a,&'b str>,
    {
    }
    impl<'a> Args<'a,&str> for ArbitraryArg<'a> {}
    impl<'a> Args<'a,&str> for Vec<ArbitraryArg<'a>> {}
}

mod parse_impls {
    use crate::{
        parser::traits::ParseBefore,
        tokens::{CharToken, DelimPair},
    };

    use super::{ArbitraryArg, ArbitraryDelimitedArg, MapParsedValInResult, Parse};
    use nom::{branch::alt, multi::many0, sequence::tuple, IResult};

    impl<'a,D> Parse<'a,&str> for ArbitraryDelimitedArg<'a, D>
    where
        D: DelimPair,
        for<'b,'c> D::Left: ParseBefore<'a,&'c str> + CharToken + Parse<'a,&'b str>,
        for<'b,'c> D::Right: Parse<'a,&'b str> + ParseBefore<'a,&'c str> + CharToken,
    {
        fn parse<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
        where
            'b: 'c,
            'b: 'a,
        {
            // FIXME: Handle nested braces, e.g. [before[action]after]
            (tuple((D::Left::parse, D::Right::parse_before, D::Right::parse))(i))
                .map_parsed_val(<_ as Into<Self>>::into)
        }
    }

    impl<'a> Parse<'a,&str> for ArbitraryArg<'a> {
        fn parse<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
        where
            'b: 'c,
            'b: 'a,
        {
            // TODO: generate the code below using e.g. stateful macros
            alt((Self::parse_bracketed, Self::parse_braced))(i)
        }
    }

    impl<'a> Parse<'a,&str> for Vec<ArbitraryArg<'a>> {
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

    impl<'a, C> ParseBefore<'a,&str> for C
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

    impl<'a, D> ParseBefore<'a,&str> for ArbitraryDelimitedArg<'a, D>
    where
        D: DelimPair,
        for<'b> D::Left: CharToken + ParseBefore<'a,&'b str>,
    {
        fn parse_before<'b, 'c>(i: &'b str) -> nom::IResult<&'c str, &'a str>
        where
            'b: 'c,
            'b: 'a,
        {
            D::Left::parse_before(i)
        }
    }

    impl<'a> ParseBefore<'a,&str> for ArbitraryArg<'a> {
        fn parse_before<'b, 'c>(i: &'b str) -> nom::IResult<&'c str, &'a str>
        where
            'b: 'c,
            'b: 'a,
        {
            take_till1(|c| c == LeftBrace::CHAR || c == LeftBracket::CHAR)(i)
        }
    }
}
