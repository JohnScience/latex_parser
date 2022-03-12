pub mod parser;
pub mod tokens;

#[cfg(test)]
mod tests {
    use nom::combinator::value;

    use crate::parser::{self, traits::Parse};

    #[test]
    fn parse_comment() {
        use parser::comment::Comment;

        let (i, Comment {
            percent_sign,
            text,
            opt_line_ending,
        }) = Comment::parse("% latex comment").unwrap();

        assert_eq!(percent_sign.0, '%');
        assert_eq!(text, " latex comment");
        matches!(opt_line_ending, None);

        assert_eq!(i, "");
    }

    #[test]
    fn skip_comment() {
        use parser::comment::Comment;

        assert_eq!(value((), Comment::parse)("% latex comment"), Ok(("", ())));
    }

    #[test]
    fn parse_comment_in_multiline() {
        use parser::comment::Comment;

        let (i, Comment {
            percent_sign,
            text,
            opt_line_ending,
        }) = Comment::parse(
            "% latex comment\n\
            \\view{...}",
        )
        .unwrap();

        assert_eq!(percent_sign.0, '%');
        assert_eq!(text, " latex comment");
        assert_eq!(opt_line_ending.map(|newtype| newtype.0), Some("\n"));

        assert_eq!(i, "\\view{...}");
    }

    #[test]
    fn parse_arbitrary_command_without_optional_arg() {
        use crate::parser::command::arbitrary::{
            ArbitraryArg::{Bracketed, Braced},
            ArbitraryCommand,
        };

        let (i, ArbitraryCommand {
            backslash,
            cmd_name,
            arguments 
        }) = ArbitraryCommand::parse(
            "\\textbf{Command without optional arguments} % bold text\n\
            Next line",
        )
        .unwrap();

        assert_eq!(backslash.0, '\\');
        assert_eq!(cmd_name, "textbf");
        assert_eq!(arguments.len(), 1);

        match &arguments[0] {
            Bracketed(_) => panic!(
                "{} is expected to be {} variant",
                stringify!(&arguments[0]),
                stringify!(Required)
            ),
            Braced(required_argument) => {
                assert_eq!(required_argument.left_delim.0, '{');
                assert_eq!(
                    required_argument.verbatim,
                    "Command without optional arguments"
                );
                assert_eq!(required_argument.right_delim.0, '}');
            }
        }

        assert_eq!(
            i,
            " % bold text\n\
            Next line"
        );
    }

    #[test]
    fn parse_frac() {
        use crate::parser::command::arbitrary::{
            ArbitraryArg::{Bracketed, Braced},
            ArbitraryBracedArg, ArbitraryBracketedArg, ArbitraryCommand,
        };

        let (i, ArbitraryCommand {
            backslash,
            cmd_name,
            arguments 
        }) = ArbitraryCommand::parse("\\frac{2}{5}").unwrap();

        assert_eq!(backslash.0, '\\');
        assert_eq!(cmd_name, "frac");
        assert_eq!(arguments.len(), 2);

        let (bracketed_args, braced_args) = arguments.into_iter().fold(
            (
                Vec::<ArbitraryBracketedArg>::new(),
                Vec::<ArbitraryBracedArg>::new(),
            ),
            |(mut bracketed_args, mut braced_args), arg| {
                match arg {
                    Braced(braced_arg) => braced_args.push(braced_arg),
                    Bracketed(bracketed_arg) => bracketed_args.push(bracketed_arg),
                };
                (bracketed_args, braced_args)
            },
        );

        assert_eq!(bracketed_args.len(), 0);
        assert_eq!(braced_args.len(), 2);

        assert_eq!(braced_args[0].left_delim.0, '{');
        assert_eq!(braced_args[0].verbatim, "2");
        assert_eq!(braced_args[0].right_delim.0, '}');
        assert_eq!(braced_args[1].left_delim.0, '{');
        assert_eq!(braced_args[1].verbatim, "5");
        assert_eq!(braced_args[1].right_delim.0, '}');
        assert_eq!(i, "");
    }
}
