use nom::IResult;

use super::process::process_text;

#[derive(Debug, PartialEq, Eq)]
pub struct TextNode {
    pub text: String,
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
