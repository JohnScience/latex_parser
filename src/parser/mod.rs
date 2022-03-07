pub trait Parse<'a>: Sized {
    fn parse<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
    where
        'b: 'c,
        'b: 'a;
}

pub mod command;
pub mod comment;

pub use command::{arbitrary::ArbitraryCommand, Command};
use nom::IResult;
