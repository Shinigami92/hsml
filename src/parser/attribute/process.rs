use nom::{bytes::complete::take_till1, IResult};

pub fn process_attribute(input: &str) -> IResult<&str, &str> {
    let mut input = input;
    let (remaining, attribute_key) =
        take_till1(|c: char| c.is_whitespace() || c == '.' || c == ',' || c == '\n' || c == '=')(
            input,
        )?;

    let (remaining2, attribute_value) = match remaining.chars().next() {
        Some('=') => {
            let (remaining, attribute_value) =
                take_till1(|c: char| c == ',' || c == '\n')(remaining)?;

            (remaining, Some(attribute_value))
        }
        _ => (remaining, None),
    };

    let mut attribute = attribute_key;

    if attribute_value.is_some() {
        let i = attribute_key.len() + 1 + attribute_value.unwrap().len();
        (attribute, input) = input.split_at(i);
    };

    Ok((input, attribute))
}

#[cfg(test)]
mod tests {
    use nom::error::{Error, ErrorKind};

    use crate::parser::attribute::process::process_attribute;

    #[test]
    fn it_should_process_attribute() {
        let input = "src=\"https://github.com/\"";

        let (rest, attribute) = process_attribute(input).unwrap();

        assert_eq!(attribute, "src=\"https://github.com/\"");
        assert_eq!(rest, "");
    }

    #[test]
    fn it_should_process_attribute_without_value() {
        let input = "disabled";

        let (rest, attribute) = process_attribute(input).unwrap();

        assert_eq!(attribute, "disabled");
        assert_eq!(rest, "");
    }

    #[test]
    fn it_should_process_attribute_followed_by_another_attribute() {
        let input = "disabled required";

        let (rest, attribute) = process_attribute(input).unwrap();

        assert_eq!(attribute, "disabled");
        assert_eq!(rest, " required");
    }

    #[test]
    fn it_should_process_attribute_followed_by_another_attribute_separated_by_comma() {
        let input = "disabled, required";

        let (rest, attribute) = process_attribute(input).unwrap();

        assert_eq!(attribute, "disabled");
        assert_eq!(rest, ", required");
    }

    #[test]
    fn it_should_process_attribute_with_angular_binding() {
        let input = "color=\"{{ color }}\", required";

        let (rest, attribute) = process_attribute(input).unwrap();

        assert_eq!(attribute, "color=\"{{ color }}\"");
        assert_eq!(rest, ", required");
    }

    #[test]
    fn it_should_process_attribute_with_angular_ng_model() {
        let input = "[(ngModel)]=\"name\", required";

        let (rest, attribute) = process_attribute(input).unwrap();

        assert_eq!(attribute, "[(ngModel)]=\"name\"");
        assert_eq!(rest, ", required");
    }

    #[test]
    fn it_should_process_attribute_with_angular_event() {
        let input = "(click)=\"setValue()\", required";

        let (rest, attribute) = process_attribute(input).unwrap();

        assert_eq!(attribute, "(click)=\"setValue()\"");
        assert_eq!(rest, ", required");
    }

    #[test]
    fn it_should_process_attribute_with_vue_binding() {
        let input = ":src=\"image\", alt=\"Image\"";

        let (rest, attribute) = process_attribute(input).unwrap();

        assert_eq!(attribute, ":src=\"image\"");
        assert_eq!(rest, ", alt=\"Image\"");
    }

    #[test]
    fn it_should_process_attribute_with_vue_event() {
        let input = "@click=\"setValue()\", color=\"primary\"";

        let (rest, attribute) = process_attribute(input).unwrap();

        assert_eq!(attribute, "@click=\"setValue()\"");
        assert_eq!(rest, ", color=\"primary\"");
    }

    #[test]
    fn it_should_process_attribute_with_vue_slot() {
        let input = "#header=\"slot\"";

        let (rest, attribute) = process_attribute(input).unwrap();

        assert_eq!(attribute, "#header=\"slot\"");
        assert_eq!(rest, "");
    }

    // Negative tests

    #[test]
    fn it_should_not_process_attribute_with_number() {
        let input = "1src=\"https://github.com\"";

        assert_eq!(
            Err(nom::Err::Error(Error {
                input: "1src=\"https://github.com\"",
                code: ErrorKind::TakeTill1
            })),
            process_attribute(input)
        );
    }

    #[test]
    fn it_should_not_process_attribute_with_whitespace() {
        let input = " src=\"https://github.com\"";

        assert_eq!(
            Err(nom::Err::Error(Error {
                input: " src=\"https://github.com\"",
                code: ErrorKind::TakeTill1
            })),
            process_attribute(input)
        );
    }

    #[test]
    fn it_should_not_process_attribute_with_dot() {
        let input = ".src=\"https://github.com\"";

        assert_eq!(
            Err(nom::Err::Error(Error {
                input: ".src=\"https://github.com\"",
                code: ErrorKind::TakeTill1
            })),
            process_attribute(input)
        );
    }

    #[test]
    fn it_should_not_process_attribute_with_comma() {
        let input = ",src=\"https://github.com\"";

        assert_eq!(
            Err(nom::Err::Error(Error {
                input: ",src=\"https://github.com\"",
                code: ErrorKind::TakeTill1
            })),
            process_attribute(input)
        );
    }

    #[test]
    fn it_should_not_process_attribute_with_line_ending() {
        let input = "\nsrc=\"https://github.com\"";

        assert_eq!(
            Err(nom::Err::Error(Error {
                input: "\nsrc=\"https://github.com\"",
                code: ErrorKind::TakeTill1
            })),
            process_attribute(input)
        );
    }
}
