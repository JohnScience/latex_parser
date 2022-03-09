use crate::{
    parser::Parse,
    tokens::{LineEnding, PercentSign},
};
use nom::{
    character::complete::{char, not_line_ending},
    sequence::tuple,
    IResult,
};

pub struct Comment<'a> {
    pub percent_sign: PercentSign,
    pub text: &'a str,
    pub opt_line_ending: Option<LineEnding<'a>>,
}

impl<'a> Parse<'a> for Comment<'a> {
    fn parse<'b, 'c>(i: &'b str) -> IResult<&'c str, Self>
    where
        'b: 'c,
        'b: 'a,
    {
        let (i, (percent_sign, text, opt_line_ending)) =
            tuple((char('%'), not_line_ending, Option::<LineEnding>::parse))(i)?;
        Ok((
            i,
            Comment {
                percent_sign: PercentSign(percent_sign),
                text,
                opt_line_ending,
            },
        ))
    }
}
