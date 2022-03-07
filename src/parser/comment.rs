use nom::{
    character::complete::{char, line_ending, not_line_ending},
    combinator::opt,
    error::ParseError,
    sequence::tuple,
    IResult,
};

pub struct Comment<'a>(&'a str);

pub fn comment<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &str, E> {
    let (i, (_percent_sign, comment_text, _opt_line_ending)) =
        tuple((char('%'), not_line_ending, opt(line_ending)))(i)?;
    Ok((i, comment_text))
}
