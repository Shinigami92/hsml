use nom::{
    bytes::complete::{is_not, tag, take_till1},
    character::complete::char,
    sequence::delimited,
    IResult,
};

fn square_bracket_delimited(input: &str) -> IResult<&str, &str> {
    delimited(char('['), is_not("]"), char(']'))(input)
}

pub fn process_class(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag(".")(input)?;
    let (remaining, class) =
        take_till1(|c: char| c == '.' || c == '(' || c.is_whitespace())(input)?;

    // Parse arbitrary tailwind color
    if class.contains("[#") {
        let (remaining2, class_prefix) = take_till1(|c: char| c == '[')(class)?;

        if let Ok((_, color)) = square_bracket_delimited(remaining2) {
            let i = class_prefix.len() + color.len() + 2;
            let (class, _) = class.split_at(i);

            let (_, input) = input.split_at(class.len());

            return Ok((input, class));
        }
    }
    // Cut at id
    else if class.contains('#') {
        let (class, _) = class.split_at(class.find('#').unwrap());

        let (_, input) = input.split_at(class.len());

        return Ok((input, class));
    }

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
        let input = ".focus:outline-none";

        let (rest, class) = process_class(input).unwrap();

        assert_eq!(class, "focus:outline-none");
        assert_eq!(rest, "");
    }

    #[test]
    fn it_should_process_class_with_arbitrary_tailwind_color() {
        let input = ".bg-[#1da1f2]#name Text";

        let (rest, class) = process_class(input).unwrap();

        assert_eq!(class, "bg-[#1da1f2]");
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
        let input = ".text-red(disabled)";

        let (rest, class) = process_class(input).unwrap();

        assert_eq!(class, "text-red");
        assert_eq!(rest, "(disabled)");
    }

    #[test]
    fn it_should_process_class_with_whitespace() {
        let input = ".text-red ";

        let (rest, class) = process_class(input).unwrap();

        assert_eq!(class, "text-red");
        assert_eq!(rest, " ");
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
        let input = "text-red(disabled)";

        assert_eq!(
            Err(nom::Err::Error(Error {
                input: "text-red(disabled)",
                code: ErrorKind::Tag
            })),
            process_class(input)
        );

        let input = "#text-red(disabled)";

        assert_eq!(
            Err(nom::Err::Error(Error {
                input: "#text-red(disabled)",
                code: ErrorKind::Tag
            })),
            process_class(input)
        );
    }
}
