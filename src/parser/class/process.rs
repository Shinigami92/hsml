use nom::{
    bytes::complete::tag,
    error::{Error, ErrorKind},
    IResult, Needed,
};

pub fn process_class(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag(".")(input)?;

    let mut remaining = input;

    let mut class_index = 0;

    loop {
        // get first char and check if it is a `[`
        // if so, it is an arbitrary tailwind value
        let first_char = remaining.get(..1);

        match first_char {
            Some("#") => {
                // we hit a id, so we are done
                break;
            }
            Some(".") => {
                // we hit a new class, so we are done
                break;
            }
            Some("(") => {
                // we hit the start of attributes, so we are done
                break;
            }
            Some(" ") => {
                // we hit a whitespace, so we are done
                break;
            }
            Some("\t") => {
                // we hit a tab, so we are done
                break;
            }
            Some("\r") if remaining.get(1..2) == Some("\n") => {
                // we hit a newline, so we are done
                break;
            }
            Some("\r") => {}
            Some("\n") => {
                // we hit a newline, so we are done
                break;
            }
            Some("[") => {
                // Parse arbitrary tailwind values (https://tailwindcss.com/docs/adding-custom-styles#using-arbitrary-values)

                let closing_bracket = ']';

                let mut closing_bracket_index = 0;
                let mut is_escaped = false;

                for (index, c) in remaining.chars().enumerate() {
                    if index == 0 {
                        // skip first char, because it is the opening bracket
                        continue;
                    }

                    if c == '\\' {
                        is_escaped = true;
                        continue;
                    }

                    if c == closing_bracket && !is_escaped {
                        closing_bracket_index = index;
                        break;
                    }

                    is_escaped = false;
                }

                if closing_bracket_index == 0 {
                    return Err(nom::Err::Error(Error::new(remaining, ErrorKind::Tag)));
                }

                class_index += closing_bracket_index;
                remaining = input.get(class_index..).unwrap();

                continue;
            }
            Some(_) => {
                // we hit a char, so we need to append it to the class
                class_index += 1;
                remaining = remaining.get(1..).unwrap();
                continue;
            }
            None => {
                return Err(nom::Err::Incomplete(Needed::Unknown));
            }
        }
    }

    let class = input.get(..class_index).unwrap();

    Ok((remaining, class))
}

#[cfg(test)]
mod tests {
    use nom::error::{Error, ErrorKind};

    use crate::parser::class::process::process_class;

    #[test]
    fn it_should_process_class_with_text() {
        let input = ".text-red Text";

        let (rest, class) = process_class(input).unwrap();

        assert_eq!(class, "text-red");
        assert_eq!(rest, " Text");
    }

    #[test]
    fn it_should_process_class_with_colon() {
        let input = ".focus:outline-none Text";

        let (rest, class) = process_class(input).unwrap();

        assert_eq!(class, "focus:outline-none");
        assert_eq!(rest, " Text");
    }

    #[test]
    fn it_should_process_class_with_arbitrary_tailwind_value() {
        let input = ".bg-[#1da1f2]#name Text";

        let (rest, class) = process_class(input).unwrap();

        assert_eq!(class, "bg-[#1da1f2]");
        assert_eq!(rest, "#name Text");
    }

    #[test]
    fn it_should_process_class_with_arbitrary_tailwind_value_2() {
        let input = ".lg:[&:nth-child(3)]:hover:underline#name Text";

        let (rest, class) = process_class(input).unwrap();

        assert_eq!(class, "lg:[&:nth-child(3)]:hover:underline");
        assert_eq!(rest, "#name Text");
    }

    #[test]
    fn it_should_process_class_with_arbitrary_tailwind_value_3() {
        let input = ".bg-[url('/what_a_rush.png')]#name Text";

        let (rest, class) = process_class(input).unwrap();

        assert_eq!(class, "bg-[url('/what_a_rush.png')]");
        assert_eq!(rest, "#name Text");
    }

    #[test]
    fn it_should_process_class_with_id() {
        let input = ".text-red#name Text";

        let (rest, class) = process_class(input).unwrap();

        assert_eq!(class, "text-red");
        assert_eq!(rest, "#name Text");
    }

    #[test]
    fn it_should_process_class_with_attribute() {
        let input = ".text-red(disabled) Text";

        let (rest, class) = process_class(input).unwrap();

        assert_eq!(class, "text-red");
        assert_eq!(rest, "(disabled) Text");
    }

    #[test]
    fn it_should_process_class_with_whitespace() {
        let input = ".text-red Text";

        let (rest, class) = process_class(input).unwrap();

        assert_eq!(class, "text-red");
        assert_eq!(rest, " Text");
    }

    #[test]
    fn it_should_process_class_with_tab() {
        let input = ".text-red\t";

        let (rest, class) = process_class(input).unwrap();

        assert_eq!(class, "text-red");
        assert_eq!(rest, "\t");
    }

    #[test]
    fn it_should_process_class_with_line_ending() {
        let input = ".text-red\n";

        let (rest, class) = process_class(input).unwrap();

        assert_eq!(class, "text-red");
        assert_eq!(rest, "\n");
    }

    #[test]
    fn it_should_process_class_with_crlf() {
        let input = ".text-red\r\n";

        let (rest, class) = process_class(input).unwrap();

        assert_eq!(class, "text-red");
        assert_eq!(rest, "\r\n");
    }

    // Negative tests

    #[test]
    fn it_should_not_process_class_without_dot() {
        let input = "text-red(disabled) Text";

        assert_eq!(
            Err(nom::Err::Error(Error {
                input: "text-red(disabled) Text",
                code: ErrorKind::Tag
            })),
            process_class(input)
        );

        let input = "#text-red(disabled) Text";

        assert_eq!(
            Err(nom::Err::Error(Error {
                input: "#text-red(disabled) Text",
                code: ErrorKind::Tag
            })),
            process_class(input)
        );
    }
}
