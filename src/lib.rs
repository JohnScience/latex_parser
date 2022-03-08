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
        let (i, arbitrary_cmd) = parser::command::ArbitraryCommand::parse(
            "\\textbf{Command without optional arguments} % bold text\n\
            Next line"
        ).unwrap();

        assert_eq!(arbitrary_cmd.backslash.0, '\\');
        assert_eq!(arbitrary_cmd.cmd_name, "textbf");
        assert_eq!(arbitrary_cmd.optional_arguments.len(), 0);
        assert_eq!(arbitrary_cmd.required_arguments.len(), 1);
        assert_eq!(arbitrary_cmd.required_arguments[0].left_brace.0, '{');
        assert_eq!(arbitrary_cmd.required_arguments[0].verbatim, "Command without optional arguments");
        assert_eq!(arbitrary_cmd.required_arguments[0].right_brace.0, '}');
        assert_eq!(
            i,
            " % bold text\n\
            Next line"
        );
    }

    #[test]
    fn parse_frac() {
        let (i, arbitrary_cmd) = parser::command::ArbitraryCommand::parse(
            "\\frac{2}{5}"
        ).unwrap();
        assert_eq!(arbitrary_cmd.backslash.0, '\\');
        assert_eq!(arbitrary_cmd.cmd_name, "frac");
        assert_eq!(arbitrary_cmd.optional_arguments.len(), 0);
        assert_eq!(arbitrary_cmd.required_arguments.len(), 2);
        assert_eq!(arbitrary_cmd.required_arguments[0].left_brace.0, '{');
        assert_eq!(arbitrary_cmd.required_arguments[0].verbatim, "2");
        assert_eq!(arbitrary_cmd.required_arguments[0].right_brace.0, '}');
        assert_eq!(arbitrary_cmd.required_arguments[1].left_brace.0, '{');
        assert_eq!(arbitrary_cmd.required_arguments[1].verbatim, "5");
        assert_eq!(arbitrary_cmd.required_arguments[1].right_brace.0, '}');
        assert_eq!(i,"");
    }
}
