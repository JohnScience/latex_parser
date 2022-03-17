use crate::{
    parser::traits::{MapParsedValInResult, Parse, LifetimizedExt},
    tokens::{LineEnding, PercentSign},
};
use from_tuple::OrderDependentFromTuple;
use nom::{character::complete::not_line_ending, sequence::tuple, IResult};

#[derive(OrderDependentFromTuple)]
pub struct Comment<'a> {
    pub percent_sign: PercentSign,
    pub text: &'a str,
    pub opt_line_ending: Option<LineEnding<'a>>,
}

// TODO: write a derive macros
impl<'a> LifetimizedExt for Comment<'a> {
    type Lifetimized<'b> = Comment<'b>;
}

impl<'a> Parse<'a,&str> for Comment<'a> {
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
        .map_parsed_val(<_ as Into<Comment>>::into)
    }
}
