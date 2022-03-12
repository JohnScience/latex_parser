use crate::parser::command::arbitrary::{
    ArbitraryArg::{self, Braced, Bracketed},
    ArbitraryBracedArg, ArbitraryBracketedArg,
};

pub trait GroupByDelims<'a> {
    fn group_by_delims(self) -> (Vec<ArbitraryBracketedArg<'a>>, Vec<ArbitraryBracedArg<'a>>);
}

impl<'a> GroupByDelims<'a> for Vec<ArbitraryArg<'a>> {
    fn group_by_delims(self) -> (Vec<ArbitraryBracketedArg<'a>>, Vec<ArbitraryBracedArg<'a>>) {
        self.into_iter().fold(
            (
                Vec::<ArbitraryBracketedArg>::new(),
                Vec::<ArbitraryBracedArg>::new(),
            ),
            |(mut bracketed, mut braced), arg| {
                match arg {
                    Braced(braced_arg) => braced.push(braced_arg),
                    Bracketed(bracketed_arg) => bracketed.push(bracketed_arg),
                };
                (bracketed, braced)
            },
        )
    }
}
