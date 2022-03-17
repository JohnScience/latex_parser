use crate::parser::{
    traits::LifetimizedExt,
    span::StrSpan,
};

pub trait ParsableInput: LifetimizedExt {}

impl<'a> ParsableInput for &'a str {}
impl<'a> ParsableInput for StrSpan<'a> {}