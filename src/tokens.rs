use crate::parser::Parse;
use nom::{character::complete::char, IResult};

macro_rules! declare_token_ty {
    ($t:ident[$char:literal]) => {
        pub struct $t(pub char);

        impl<'a> Parse<'a> for $t {
            fn parse<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
            where
                'b: 'c,
                'b: 'a,
            {
                let (remainder, token) = char($char)(i)?;
                Ok((remainder, $t(token)))
            }
        }
    };
}

declare_token_ty!(Backslash['\\']);
declare_token_ty!(LeftSquareBracket['[']);
declare_token_ty!(RightSquareBracket[']']);
declare_token_ty!(LeftCurlyBrace['{']);
declare_token_ty!(RightCurlyBrace['}']);

trait DelimPair {
    type Left;
    type Right;
}

impl DelimPair for (LeftSquareBracket, RightSquareBracket) {
    type Left = LeftSquareBracket;
    type Right = RightSquareBracket;
}

impl DelimPair for (LeftCurlyBrace, RightCurlyBrace) {
    type Left = LeftCurlyBrace;
    type Right = RightCurlyBrace;
}
