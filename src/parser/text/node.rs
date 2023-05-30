use nom::IResult;

use crate::parser::HsmlProcessContext;

use super::process::{process_text, process_text_block};

#[derive(Debug, PartialEq, Eq)]
pub struct TextNode {
    pub text: String,
}

pub fn text_block_node<'a>(
    input: &'a str,
    context: &mut HsmlProcessContext,
) -> IResult<&'a str, TextNode> {
    let (input, text) = process_text_block(input, context)?;

    Ok((
        input,
        TextNode {
            text: text.to_string(),
        },
    ))
}

pub fn text_node(input: &str) -> IResult<&str, TextNode> {
    let (input, text) = process_text(input)?;

    Ok((
        input,
        TextNode {
            text: text.to_string(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        text::node::{text_block_node, TextNode},
        HsmlProcessContext,
    };

    #[test]
    fn it_should_return_text_block_node() {
        let context = &mut HsmlProcessContext {
            indent_string: Some(String::from("  ")),
            indent_level: 3,
        };

        let (input, text_block) = text_block_node(
            r#".
        "Tailwind CSS is the only framework that I've seen scale
        on large teams. It's easy to customize, adapts to any design,
        and the build size is tiny."
    figcaption.font-medium"#,
            context,
        )
        .unwrap();

        assert_eq!(
            text_block,
            TextNode {
                text: String::from(
                    r#"        "Tailwind CSS is the only framework that I've seen scale
        on large teams. It's easy to customize, adapts to any design,
        and the build size is tiny.""#
                ),
            }
        );

        assert_eq!(input, "\n    figcaption.font-medium");
    }
}
