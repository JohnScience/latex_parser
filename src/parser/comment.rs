use crate::{
    parser::traits::{FromTuple, MapParsedValInResult, Parse},
    tokens::{LineEnding, PercentSign},
};
use nom::{character::complete::not_line_ending, sequence::tuple, IResult};

pub struct Comment<'a> {
    pub percent_sign: PercentSign,
    pub text: &'a str,
    pub opt_line_ending: Option<LineEnding<'a>>,
}

// FIXME: make work for `Self`
impl<'a> FromTuple<(PercentSign, &'a str, Option<LineEnding<'a>>)> for Comment<'a> {
    fn from_tuple(
        (percent_sign, text, opt_line_ending): (PercentSign, &'a str, Option<LineEnding<'a>>),
    ) -> Self {
        Self {
            percent_sign,
            text,
            opt_line_ending,
        }
    }
}

impl<'a> Parse<'a> for Comment<'a> {
    fn parse<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
    where
        'b: 'c,
        'b: 'a,
    {
        tuple((
            PercentSign::parse,
            not_line_ending,
            Option::<LineEnding>::parse,
        ))(i)
        // FIXME: make FromTuple work for `Self`
        .map_parsed_val(Comment::from_tuple)
    }
}
