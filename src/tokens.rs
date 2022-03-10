use crate::parser::traits::{MapParsedValInResult, Parse};
use core::default::Default;
use nom::{
    character::complete::{char, line_ending},
    IResult,
};

pub trait CharToken {
    const CHAR: char;
    const CHAR_STR: &'static str;
}

macro_rules! declare_char_token_ty {
    ($t:ident[$lit:literal]) => {
        pub struct $t(pub char);

        impl CharToken for $t {
            const CHAR: char = $lit;
            const CHAR_STR: &'static str = stringify!($lit);
        }

        impl<'a> Parse<'a> for $t {
            fn parse<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
            where
                'b: 'c,
                'b: 'a,
            {
                char($lit)(i).map_parsed_val($t)
            }
        }

        impl Default for $t {
            fn default() -> Self {
                Self(Self::CHAR)
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
                $($parsing)+(i).map_parsed_val($t)
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

pub trait DelimPair {
    type Left;
    type Right;
}

pub type Brackets = (LeftBracket, RightBracket);
pub type Braces = (LeftBrace, RightBrace);

impl DelimPair for Brackets {
    type Left = LeftBracket;
    type Right = RightBracket;
}

impl DelimPair for Braces {
    type Left = LeftBrace;
    type Right = RightBrace;
}
