pub mod parser;
pub mod tokens;

#[cfg(test)]
mod tests {
    use nom::combinator::value;

    use crate::parser::{self, Parse};

    #[test]
    fn parse_comment() {
        assert_eq!(
            parser::comment::comment::<()>("% latex comment"),
            Ok(("", " latex comment"))
        );
    }

    #[test]
    fn skip_comment() {
        assert_eq!(
            value((),parser::comment::comment::<()>)("% latex comment"),
            Ok(("", ()))
        );
    }

    #[test]
    fn parse_comment_in_multiline() {
        assert_eq!(
            parser::comment::comment::<()>(
                "% latex comment\n\
                \\view{...}"
            ),
            Ok(("\\view{...}", " latex comment"))
        );
    }

    #[test]
    fn parse_arbitrary_command_without_optional_arg() {
        use crate::parser::command::arbitrary::ArbitraryArgument::{Required, Optional};

        let (i, arbitrary_cmd) = parser::command::ArbitraryCommand::parse(
            "\\textbf{Command without optional arguments} % bold text\n\
            Next line"
        ).unwrap();

        assert_eq!(arbitrary_cmd.backslash.0, '\\');
        assert_eq!(arbitrary_cmd.cmd_name, "textbf");
        assert_eq!(arbitrary_cmd.arguments.len(), 1);

        match &arbitrary_cmd.arguments[0] {
            Optional(_) => panic!("arbitrary_cmd.arguments[0] is not Required variant"),
            Required(required_argument) => {
                assert_eq!(required_argument.left_brace.0, '{');
                assert_eq!(required_argument.verbatim, "Command without optional arguments");
                assert_eq!(required_argument.right_brace.0, '}');
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
        use crate::parser::command::arbitrary::{ArbitraryRequiredArgument, ArbitraryOptionalArgument, ArbitraryArgument::{Required, Optional}};

        let (i, arbitrary_cmd) = parser::command::ArbitraryCommand::parse(
            "\\frac{2}{5}"
        ).unwrap();
        assert_eq!(arbitrary_cmd.backslash.0, '\\');
        assert_eq!(arbitrary_cmd.cmd_name, "frac");
        assert_eq!(arbitrary_cmd.arguments.len(), 2);

        let (optional_arguments, required_arguments)
            = arbitrary_cmd.arguments.into_iter()
            .fold((Vec::<ArbitraryOptionalArgument>::new(), Vec::<ArbitraryRequiredArgument>::new()), |(mut opt_args, mut req_args),arg| {
                match arg {
                    Required(req_arg) => { req_args.push(req_arg) },
                    Optional(opt_arg) => { opt_args.push(opt_arg) },
                };
                (opt_args, req_args)
            }
        );

        assert_eq!(optional_arguments.len(), 0);
        assert_eq!(required_arguments.len(), 2);

        assert_eq!(required_arguments[0].left_brace.0, '{');
        assert_eq!(required_arguments[0].verbatim, "2");
        assert_eq!(required_arguments[0].right_brace.0, '}');
        assert_eq!(required_arguments[1].left_brace.0, '{');
        assert_eq!(required_arguments[1].verbatim, "5");
        assert_eq!(required_arguments[1].right_brace.0, '}');
        assert_eq!(i,"");
    }
}