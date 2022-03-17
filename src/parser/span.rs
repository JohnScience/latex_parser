// TODO: come up with better naming
use core::marker::PhantomData;
use nom_locate::LocatedSpan;
use crate::parser::traits::ParseStr;

pub type StrSpan<'a> = LocatedSpan<&'a str>;

struct Span {
    offset: u32,
    line: u32,
    column: u32,
}

trait SpanTuple<'a> {
    type BeginSpanInfo;
    type Lexeme: ParseStr<'a>;
    type EndSpanInfo;
}

struct ShallowSpanned<'a,S>
where
    S: SpanTuple<'a>,
{
    begin: S::BeginSpanInfo,
    value: S::Lexeme,
    end: S::EndSpanInfo,
    phantom: PhantomData<&'a ()>
}
