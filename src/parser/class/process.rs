use nom::{
    bytes::complete::{tag, take_till1},
    IResult,
};

pub fn process_class(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag(".")(input)?;
    take_till1(|c| c == ' ' || c == '.' || c == '#' || c == '(' || c == '\n')(input)
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
    fn it_should_process_class_with_id() {
        let input = ".text-red#name";

        let (rest, class) = process_class(input).unwrap();

        assert_eq!(class, "text-red");
        assert_eq!(rest, "#name");
    }

    #[test]
    fn it_should_process_class_with_attribute() {
        let input = ".text-red(disabled)";

        let (rest, class) = process_class(input).unwrap();

        assert_eq!(class, "text-red");
        assert_eq!(rest, "(disabled)");
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
