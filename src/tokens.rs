use crate::parser::Parse;
use core::default::Default;
use nom::{
    character::complete::{char, line_ending},
    IResult,
};

macro_rules! declare_char_token_ty {
    ($t:ident[$lit:literal]) => {
        pub struct $t(pub char);

        impl<'a> Parse<'a> for $t {
            fn parse<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
            where
                'b: 'c,
                'b: 'a,
            {
                let (remainder, token) = char($lit)(i)?;
                Ok((remainder, $t(token)))
            }
        }

        impl Default for $t {
            fn default() -> Self {
                Self($lit)
            }
        }
    };
}

macro_rules! declare_token_ty {
    ($t:ident$(<$l:lifetime>)?[$lit:literal: $lit_t:ty]::$($parsing:tt)+) => {
        pub struct $t$(<$l>)?(pub $lit_t);

        impl<'a> Parse<'a> for $t$(<$l>)? {
            fn parse<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
            where
                'b: 'c,
                'b: 'a,
            {
                let (remainder, token) = $($parsing)+(i)?;
                Ok((remainder, $t(token)))
            }
        }

        impl$(<$l>)? Default for $t$(<$l>)? {
            fn default() -> Self {
                Self($lit)
            }
        }
    };
}

declare_char_token_ty!(Backslash['\\']);
declare_char_token_ty!(PercentSign['%']);
declare_char_token_ty!(LeftBracket['[']);
declare_char_token_ty!(RightBracket[']']);
declare_char_token_ty!(LeftBrace['{']);
declare_char_token_ty!(RightBrace['}']);
declare_token_ty!(LineEnding<'a>["\n": &'a str]::line_ending);

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
