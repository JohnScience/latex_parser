use crate::{
    parser::traits::Parse,
    tokens::{LineEnding, PercentSign},
};
use nom::{character::complete::not_line_ending, sequence::tuple, IResult};

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
        let (i, (percent_sign, text, opt_line_ending)) = tuple((
            PercentSign::parse,
            not_line_ending,
            Option::<LineEnding>::parse,
        ))(i)?;
        Ok((
            i,
            Comment {
                percent_sign,
                text,
                opt_line_ending,
            },
        ))
    }
}
