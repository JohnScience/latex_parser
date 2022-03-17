// TODO: come up with better naming
use nom_locate::LocatedSpan;
use crate::parser::traits::{Parse,LifetimizedExt};

pub type StrSpan<'a> = LocatedSpan<&'a str>;

pub struct SpanInfo {
    offset: u32,
    line: u32,
    column: u32,
}

pub trait SpanTuple<'a> {
    type BeginSpanInfo;
    type Lexeme: Parse<'a>;
    type EndSpanInfo;
}

pub trait CanonicalSpanTupleExt<'a> {
    type CanonicalSpanTuple: SpanTuple<'a>;
}

pub struct ShallowSpanned<'a,T>
where
    T: CanonicalSpanTupleExt<'a>,
{
    begin: <<T as CanonicalSpanTupleExt<'a>>::CanonicalSpanTuple as SpanTuple<'a>>::BeginSpanInfo,
    value: <<T as CanonicalSpanTupleExt<'a>>::CanonicalSpanTuple as SpanTuple<'a>>::Lexeme,
    end: <<T as CanonicalSpanTupleExt<'a>>::CanonicalSpanTuple as SpanTuple<'a>>::EndSpanInfo,
}

impl<'a> LifetimizedExt for StrSpan<'a> {
    type Lifetimized<'b> = StrSpan<'b>;
}
