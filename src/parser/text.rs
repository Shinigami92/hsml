use nom::{
    bytes::complete::{tag, take_until1},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
pub struct TextNode {
    pub text: String,
}

pub fn process_text(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag(" ")(input)?;
    take_until1("\n")(input)
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
