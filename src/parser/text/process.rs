use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag, take_until1},
};

use crate::parser::HsmlProcessContext;

pub fn process_text_block<'a>(
    input: &'a str,
    context: &mut HsmlProcessContext,
) -> IResult<&'a str, &'a str> {
    let (rest, _) = tag(".")(input)?;

    // eat one \r\n or \n
    let (rest, _) = alt((tag("\r\n"), tag("\n")))(rest)?;

    let indent_string: &str = if let Some(indent_string) = &context.indent_string {
        indent_string
    } else {
        "  "
    };

    let indent_string: &str = &indent_string.repeat(context.indent_level + 1);

    let mut text_block_index = 0;

    // loop over each line until we find a line that does not fulfill the indentation
    for (index, c) in rest.chars().enumerate() {
        if c == '\n' {
            // if next char is also a \n, then continue
            let next_char = rest.chars().nth(index + 1);
            if next_char == Some('\n') {
                text_block_index = index + 1;
                continue;
            }

            let line = &rest[index + 1..];

            // otherwise check the indentation and if it does not fulfill the indentation, then break
            if !line.starts_with(indent_string) {
                break;
            }
        } else {
            text_block_index = index;
            continue;
        }
    }

    let text_block = &rest[..text_block_index + 1];

    let rest = &rest[text_block_index + 1..];

    Ok((rest, text_block))
}

pub fn process_text(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag(" ")(input)?;
    take_until1("\n")(input)
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        HsmlProcessContext,
        text::process::{process_text, process_text_block},
    };

    #[test]
    fn it_should_process_text_block() {
        let mut context = HsmlProcessContext {
            indent_string: Some(String::from("  ")),
            indent_level: 1,
        };

        let input = r#".
    this is just some text
    it can be multiline

    and also contain blank lines
span other text
"#;

        let (rest, text_block) = process_text_block(input, &mut context).unwrap();

        assert_eq!(
            text_block,
            r#"    this is just some text
    it can be multiline

    and also contain blank lines"#
        );
        assert_eq!(
            rest,
            r#"
span other text
"#
        );
    }

    #[test]
    fn test_process_text() {
        let input = " hello world\n";

        let (rest, text) = process_text(input).unwrap();

        assert_eq!(text, "hello world");
        assert_eq!(rest, "\n");
    }
}
