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
declare_token_ty!(LeftBracket['[']);
declare_token_ty!(RightBracket[']']);
declare_token_ty!(LeftBrace['{']);
declare_token_ty!(RightBrace['}']);

trait DelimPair {
    type Left;
    type Right;
}

impl DelimPair for (LeftBracket, RightBracket) {
    type Left = LeftBracket;
    type Right = RightBracket;
}

impl DelimPair for (LeftBrace, RightBrace) {
    type Left = LeftBrace;
    type Right = RightBrace;
}
